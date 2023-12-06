use alpha_counter::AlphaCounter;
use anyhow::{anyhow, Result};
use glob::glob;
use pulldown_cmark as pd;
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

//--------------------------------------------------------------------------------------------------

/**
Create a counter to use for "numbering" the answers
*/
fn answer_counter() -> AlphaCounter {
    AlphaCounter::upper(0)
}

//--------------------------------------------------------------------------------------------------

/**
Helper function to write string content to a file
*/
pub fn write_file(path: &Path, data: &str) -> Result<()> {
    let f = File::create(path)?;
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes())?;
    Ok(())
}

//--------------------------------------------------------------------------------------------------

/**
Calculate basic statistics on a slice of f32 numbers

```
use quixote::f32_stats;

let (min, max, mean, sum, count) =
    f32_stats(&[1.0, 2.0, 3.0, 3.2, 4.7, 5.5, 1.0]);

assert_eq!(min, 1.0);
assert_eq!(max, 5.5);
assert_eq!(mean, 2.9142857);
assert_eq!(sum, 20.4);
assert_eq!(count, 7);
```
*/
pub fn f32_stats(v: &[f32]) -> (f32, f32, f32, f32, usize) {
    let count = v.len();
    let mut sum = v[0];
    let mut min = v[0];
    let mut max = v[0];
    for i in v.iter().skip(1) {
        sum += i;
        min = min.min(*i);
        max = max.max(*i);
    }
    let mean = sum / (count as f32);
    (min, max, mean, sum, count)
}

//--------------------------------------------------------------------------------------------------

/**
Question bank
*/
#[derive(Debug)]
pub struct Bank {
    questions: Vec<Question>,
}

impl Bank {
    /**
    Create a new [`Bank`] from one or more paths / globs
    */
    pub fn new(input_files: &[PathBuf]) -> Result<Bank> {
        // Glob out input files
        let input_files = input_files
            .par_iter()
            .map(|x| {
                let g = if x.is_dir() {
                    x.join("**/*.md")
                } else {
                    x.clone()
                }
                .display()
                .to_string();
                let globbed_files = glob(&g).unwrap().filter_map(|x| x.ok()).collect::<Vec<_>>();
                if globbed_files.is_empty() {
                    Err(anyhow!(format!("`{g}`")))
                } else {
                    Ok(globbed_files)
                }
            })
            .collect::<Vec<_>>();
        let errors: Vec<_> = input_files
            .iter()
            .filter_map(|x| x.as_ref().err())
            .collect();
        if !errors.is_empty() {
            let errors: Vec<_> = errors.iter().map(|x| x.to_string()).collect();
            return Err(anyhow!(format!(
                "Arguments did not resolve to any files: {}!",
                errors.join(", ")
            )));
        }
        let input_files: Vec<_> = input_files.into_iter().flat_map(|x| x.unwrap()).collect();

        // Fail if zero input files
        if input_files.is_empty() {
            return Err(anyhow!("No input files!"));
        }

        // Read input files
        let questions: Vec<Result<Vec<Question>>> = input_files
            .par_iter()
            .map(|x| {
                if let Ok(input) = std::fs::read_to_string(x) {
                    let mut input = input.trim().to_string();
                    input.push_str(if input.ends_with("---") {
                        "\n\n"
                    } else {
                        "\n\n---\n\n"
                    });
                    let mut depth = 0;
                    let mut content = vec![];
                    let mut questions = vec![];
                    for (event, range) in
                        pd::Parser::new_ext(&input, pd::Options::all()).into_offset_iter()
                    {
                        match event {
                            pd::Event::Start(_) => {
                                depth += 1;
                            }
                            pd::Event::End(_) => {
                                depth -= 1;
                                if depth == 0 {
                                    content.push(input[range.clone()].trim().to_string());
                                }
                            }
                            pd::Event::Rule => {
                                questions.push(Question::new(&content));
                                content = vec![];
                            }
                            _ => {}
                        }
                    }
                    Ok(questions)
                } else {
                    Err(anyhow!(format!("`{}`", x.display())))
                }
            })
            .collect();
        let errors: Vec<_> = questions.iter().filter_map(|x| x.as_ref().err()).collect();
        if errors.is_empty() {
            let questions = questions
                .iter()
                .flat_map(|x| x.as_ref().unwrap())
                .cloned()
                .collect();
            Ok(Bank { questions })
        } else {
            let errors: Vec<_> = errors.iter().map(|x| x.to_string()).collect();
            Err(anyhow!(format!(
                "Could not read files: {}!",
                errors.join(", ")
            )))
        }
    }

    /**
    Generate a quiz
    */
    pub fn quiz(&self) -> Quiz {
        Quiz::new(&self.questions)
    }
}

//--------------------------------------------------------------------------------------------------

/**
Quiz question
*/
#[derive(Clone, Debug)]
pub struct Question {
    content: Vec<String>,
    answers: Vec<Answer>,
}

impl Question {
    /**
    Create a [`Question`]
    */
    fn new(content: &[String]) -> Question {
        let mut content = content.to_vec();
        let answer_content = content.pop().unwrap();
        let mut answers = vec![];
        let mut depth = 0;
        for (event, range) in
            pd::Parser::new_ext(&answer_content, pd::Options::all()).into_offset_iter()
        {
            match event {
                pd::Event::Start(_) => {
                    depth += 1;
                }
                pd::Event::End(pd::Tag::Item) => {
                    depth -= 1;
                    if depth == 1 {
                        answers.push(Answer::new(
                            answer_content[(range.start + 1)..range.end].trim(),
                        ));
                    }
                }
                pd::Event::End(_) => {
                    depth -= 1;
                }
                _ => {}
            }
        }
        answers.shuffle(&mut thread_rng());
        Question { content, answers }
    }
}

//--------------------------------------------------------------------------------------------------

/**
Quiz answer
*/
#[derive(Clone, Debug)]
struct Answer {
    content: String,
    is_correct: bool,
}

impl Answer {
    /**
    Create an [`Answer`]
    */
    fn new(content: &str) -> Answer {
        if (content.starts_with("**") && content.ends_with("**"))
            || (content.starts_with("__") && content.ends_with("__"))
        {
            Answer {
                content: content[2..(content.len() - 2)].to_string(),
                is_correct: true,
            }
        } else {
            Answer {
                content: content.to_string(),
                is_correct: false,
            }
        }
    }
}

//--------------------------------------------------------------------------------------------------

/**
Quiz
*/
#[derive(Debug)]
pub struct Quiz {
    questions: Vec<Question>,
}

impl Quiz {
    /**
    Create a new [`Quiz`]
    */
    fn new(questions: &[Question]) -> Quiz {
        let mut questions = questions.to_vec();
        let mut rng = thread_rng();
        questions.shuffle(&mut rng);
        questions
            .iter_mut()
            .for_each(|x| x.answers.shuffle(&mut rng));
        Quiz { questions }
    }

    /**
    Generate quiz markdown
    */
    pub fn markdown(&self) -> String {
        self.questions
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let mut c = answer_counter();
                let pre = format!("{}. ", i + 1);
                let sep = format!("\n\n{}", " ".repeat(pre.len()));
                format!(
                    "{pre}{}\n\n{}",
                    x.content
                        .iter()
                        .map(|x| x.replace('\n', &sep[1..]))
                        .collect::<Vec<_>>()
                        .join(&sep),
                    x.answers
                        .iter()
                        .map(|x| format!("    * {}) {}\n\n", c.next().unwrap(), x.content))
                        .collect::<Vec<_>>()
                        .join("")
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /**
    Generate quiz [`Answers`] and quiz with answers markdown
    */
    pub fn answers(&self) -> (Answers, String) {
        let a: BTreeMap<usize, Vec<String>> = self
            .questions
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let mut c = answer_counter();
                (
                    i + 1,
                    x.answers
                        .iter()
                        .filter_map(|x| {
                            let answer = c.next().unwrap();
                            if x.is_correct {
                                Some(answer)
                            } else {
                                None
                            }
                        })
                        .collect(),
                )
            })
            .collect();

        let md = self
            .questions
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let mut c = answer_counter();
                let n = i + 1;
                let pre = format!("{n}. ");
                let sep = format!("\n\n{}", " ".repeat(pre.len()));
                let ans: HashSet<_> = a.get(&n).unwrap().iter().collect();
                format!(
                    "{pre}{}\n\n{}",
                    x.content
                        .iter()
                        .map(|x| x.replace('\n', &sep[1..]))
                        .collect::<Vec<_>>()
                        .join(&sep),
                    x.answers
                        .iter()
                        .map(|x| {
                            let letter = c.next().unwrap();
                            if ans.contains(&letter) {
                                format!("    * **{letter}) {}**\n\n", x.content)
                            } else {
                                format!("    * {letter}) {}\n\n", x.content)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("")
                )
            })
            .collect::<Vec<_>>()
            .join("");

        (Answers { answers: a }, md)
    }
}

//--------------------------------------------------------------------------------------------------

/**
Quiz answers
*/
#[derive(Debug, Default, Deserialize, Clone)]
pub struct Answers {
    answers: BTreeMap<usize, Vec<String>>,
}

impl Answers {
    /**
    Load [`Answers`] from a JSON file
    */
    pub fn from(path: &Path) -> Result<Answers> {
        let answers: BTreeMap<usize, Vec<String>> =
            serde_json::from_str(&std::fs::read_to_string(path)?)?;
        Ok(Answers { answers })
    }

    /**
    Calculate the total number of points in the quiz
    */
    fn total(&self) -> usize {
        self.answers.values().map(|x| x.len()).sum()
    }

    /**
    Calculate the number of problems on the quiz
    */
    fn problems(&self) -> usize {
        self.answers.len()
    }

    /**
    Get the answers for a particular problem
    */
    fn get(&self, key: &usize) -> Option<&Vec<String>> {
        self.answers.get(key)
    }
}

impl std::fmt::Display for Answers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string(&self.answers).unwrap())
    }
}

//--------------------------------------------------------------------------------------------------

/**
Completed quiz
*/
#[derive(Deserialize)]
pub struct Class {
    description: String,
    students: BTreeMap<String, BTreeMap<usize, Vec<String>>>,

    #[serde(skip)]
    total: usize,

    #[serde(skip)]
    problems: usize,

    #[serde(skip)]
    scores: BTreeMap<String, (usize, Vec<usize>)>,
}

impl Class {
    /**
    Load a [`Class`] from a JSON file
    */
    pub fn from(path: &Path) -> Result<Class> {
        let class: Class = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        Ok(class)
    }

    /**
    Compute the scores
    */
    pub fn grade(&mut self, answers: &Answers) {
        self.total = answers.total();
        self.problems = answers.problems();
        self.scores = BTreeMap::new();
        for (name, quiz) in &self.students {
            let mut missed = 0;
            let mut wrong = BTreeSet::new();
            for (q, a) in quiz {
                let correct = answers.get(q).unwrap();
                if correct.len() == 1 {
                    // Single answer
                    if a != correct {
                        missed += 1;
                        wrong.insert(*q);
                    }
                } else {
                    // Multiple answer

                    // Count missing correct answers
                    for answer in a {
                        if !correct.contains(answer) {
                            missed += 1;
                            wrong.insert(*q);
                        }
                    }

                    // Count wrong answers
                    for answer in correct {
                        if !a.contains(answer) {
                            missed += 1;
                            wrong.insert(*q);
                        }
                    }
                }
            }
            self.scores.insert(
                name.clone(),
                (self.total - missed, wrong.into_iter().collect()),
            );
        }
    }

    /**
    Generate the grade report markdown
    */
    pub fn markdown(&self) -> String {
        let mut scores: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
        let mut scores_sum = 0;
        let mut wrongs: BTreeMap<&str, String> = BTreeMap::new();
        for (name, (score, wrong)) in &self.scores {
            wrongs.insert(
                name,
                if wrong.is_empty() {
                    String::from("none")
                } else {
                    wrong
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                },
            );
            if let Some(students) = scores.get_mut(score) {
                students.push(name);
            } else {
                scores.insert(*score, vec![name]);
            }
            scores_sum += score;
        }

        let mut r = vec![format!(
            "\
# {}

Name | Score | Percentage | Grade | Wrong
-----|------:|-----------:|-------|-------
\
            ",
            self.description,
        )];

        let mut pcts = vec![];
        let mut grades: BTreeMap<_, _> = ["A", "B", "C", "D", "F"]
            .iter()
            .map(|x| (x.to_string(), 0))
            .collect();

        for (score, students) in scores.iter().rev() {
            let pct = (*score as f32) / (self.total as f32) * 100.0;
            let grade = if pct >= 90.0 {
                "A"
            } else if pct >= 80.0 {
                "B"
            } else if pct >= 70.0 {
                "C"
            } else if pct >= 60.0 {
                "D"
            } else {
                "F"
            };
            grades.entry(grade.to_string()).and_modify(|x| *x += 1);
            for name in students {
                r.push(format!(
                    "{name} | {score} | {pct:.1}% | {grade} | {}\n",
                    wrongs.get(name).unwrap(),
                ));
                pcts.push(pct);
            }
        }

        let scores_keys = scores.keys().collect::<Vec<_>>();
        let (min_pct, max_pct, mean_pct, ..) = f32_stats(&pcts);
        let mean_score = (scores_sum as f32) / (pcts.len() as f32);
        r.push(format!(
            "\n\
Description        | Value
-------------------|------------
Number of problems | {}
Total points       | {}
High score         | {} ({max_pct:.1}%)
Low score          | {} ({min_pct:.1}%)
Mean score         | {mean_score:.1} ({mean_pct:.1}%)
\
            ",
            self.problems,
            self.total,
            scores_keys.last().unwrap(),
            scores_keys.first().unwrap(),
        ));
        for (k, v) in &grades {
            r.push(format!("{k}                  | {v}\n"));
        }
        r.push(String::new());

        r.join("")
    }
}
