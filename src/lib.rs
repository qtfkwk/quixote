use alpha_counter::AlphaCounter;
use anyhow::{anyhow, Result};
use glob::glob;
use pulldown_cmark as pd;
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use veg::Veg;

//--------------------------------------------------------------------------------------------------

/**
Letter grade scale
*/
const LETTER_GRADES: [(f32, char); 5] = [
    (90.0, 'A'),
    (80.0, 'B'),
    (70.0, 'C'),
    (60.0, 'D'),
    (0.0, 'F'),
];

//--------------------------------------------------------------------------------------------------

/**
Get the letter grade for a percentage score
*/
fn letter_grade(pct: f32) -> char {
    LETTER_GRADES
        .iter()
        .find_map(|(threshold, letter)| {
            if pct >= *threshold {
                Some(*letter)
            } else {
                None
            }
        })
        .unwrap()
}

//--------------------------------------------------------------------------------------------------

/**
Create a counter to use for "numbering" the answers
*/
fn answer_counter() -> AlphaCounter {
    AlphaCounter::upper(0)
}

//--------------------------------------------------------------------------------------------------

/**
Calculate basic statistics

```text
let (min, max, mean, sum, count) = stats(&[1.0, 2.0, ...]);
```
*/
fn calc_stats(v: &[f32]) -> (f32, f32, f32, f32, usize) {
    let mut min = v[0];
    let mut max = v[0];
    let mut sum = v[0];
    for i in v.iter().skip(1) {
        min = min.min(*i);
        max = max.max(*i);
        sum += i;
    }
    let count = v.len();
    let mean = sum / (count as f32);
    (min, max, mean, sum, count)
}

//--------------------------------------------------------------------------------------------------

/**
Convert a percentage to a string uniformly
*/
fn fmt_percent(pct: f32) -> String {
    format!("{pct:.1}%")
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
    Create a new question bank from one or more paths / globs
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
        if input_files.par_iter().any(|x| x.is_err()) {
            return Err(anyhow!(format!(
                "Arguments did not resolve to any files: {}!",
                input_files
                    .iter()
                    .filter_map(|x| x.as_ref().err())
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
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
        if questions.par_iter().any(|x| x.is_err()) {
            Err(anyhow!(format!(
                "Could not read files: {}!",
                questions
                    .iter()
                    .filter_map(|x| x.as_ref().err())
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            )))
        } else {
            Ok(Bank {
                questions: questions
                    .iter()
                    .flat_map(|x| x.as_ref().unwrap())
                    .cloned()
                    .collect::<Vec<_>>(),
            })
        }
    }

    /**
    Generate a quiz
    */
    pub fn quiz(&self, shuffle: bool) -> Quiz {
        Quiz::new(&self.questions, shuffle)
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
    Create a new quiz question
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
    Create a new quiz answer
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
    Create a new quiz
    */
    fn new(questions: &[Question], shuffle: bool) -> Quiz {
        let mut questions = questions.to_vec();

        if shuffle {
            let mut rng = thread_rng();

            // Randomize questions
            questions.shuffle(&mut rng);

            // Randomize answers
            questions
                .iter_mut()
                .for_each(|x| x.answers.shuffle(&mut rng));
        }

        Quiz { questions }
    }

    /**
    Generate quiz markdown
    */
    pub fn markdown(&self) -> String {
        self.questions
            .par_iter()
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
                        .map(|x| format!("    * [ ] {}. {}\n\n", c.next().unwrap(), x.content))
                        .collect::<Vec<_>>()
                        .join("")
                )
            })
            .collect::<Vec<_>>()
            .join("")
    }

    /**
    Generate quiz [`Answers`]
    */
    pub fn answers(&self) -> Answers {
        Answers::new(self)
    }
}

//--------------------------------------------------------------------------------------------------

/**
Quiz answer key

This struct is used in two ways:

1. Generated via [`Quiz::answers()`] during quiz generation in order to generate
   content for the answer key (`answers.json`) and quiz with answers
   (`answers.md`) files
2. Loaded from a saved answer key (`answers.json`) for grading a quiz
*/
#[derive(Debug)]
pub struct Answers {
    answers: BTreeMap<usize, Vec<String>>,
    markdown: Option<String>,
}

impl Answers {
    /**
    Create a new quiz answer key
    */
    fn new(quiz: &Quiz) -> Answers {
        let answers: BTreeMap<usize, Vec<String>> = quiz
            .questions
            .par_iter()
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

        let markdown = Some(
            quiz.questions
                .par_iter()
                .enumerate()
                .map(|(i, x)| {
                    let mut c = answer_counter();
                    let n = i + 1;
                    let pre = format!("{n}. ");
                    let sep = format!("\n\n{}", " ".repeat(pre.len()));
                    let ans: HashSet<_> = answers.get(&n).unwrap().iter().collect();
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
                                    format!("    * [X] **{letter}. {}**\n\n", x.content)
                                } else {
                                    format!("    * [ ] {letter}. {}\n\n", x.content)
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(""),
                    )
                })
                .collect::<Vec<_>>()
                .join(""),
        );

        Answers { answers, markdown }
    }

    /**
    Load from a JSON file
    */
    pub fn from(path: &Path) -> Result<Answers> {
        Ok(Answers {
            answers: serde_json::from_str(&std::fs::read_to_string(path)?)?,
            markdown: None,
        })
    }

    /**
    Serialize to a JSON string
    */
    pub fn json(&self) -> String {
        serde_json::to_string(&self.answers).unwrap()
    }

    /**
    Calculate the total number of points in the quiz
    */
    fn total(&self) -> usize {
        self.answers.values().map(|x| x.len()).sum()
    }

    /**
    Return the number of questions on the quiz
    */
    fn questions(&self) -> usize {
        self.answers.len()
    }

    /**
    Get the answers for a particular problem
    */
    fn get(&self, key: &usize) -> Option<&Vec<String>> {
        self.answers.get(key)
    }

    /**
    Return the Markdown content
    */
    pub fn markdown(&self) -> &Option<String> {
        &self.markdown
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
    questions: usize,

    #[serde(skip)]
    scores: BTreeMap<String, (usize, Vec<usize>)>,
}

impl Class {
    /**
    Load from a JSON file
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
        self.questions = answers.questions();
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
        let mut wrongs: BTreeMap<&str, &[usize]> = BTreeMap::new();
        for (name, (score, wrong)) in &self.scores {
            wrongs.insert(name, wrong);
            if let Some(students) = scores.get_mut(score) {
                students.push(name);
            } else {
                scores.insert(*score, vec![name]);
            }
            scores_sum += score;
        }

        let mut pcts = vec![];
        let mut grades_hist = LETTER_GRADES
            .iter()
            .map(|(_, c)| (*c, 0))
            .collect::<BTreeMap<_, _>>();
        let mut grades = Veg::table("Name|Score|Percent|Grade|Questions\n-|-:|-:|-|-");
        for (score, students) in scores.iter().rev() {
            let pct = (*score as f32) / (self.total as f32) * 100.0;
            let grade = letter_grade(pct);
            for name in students {
                grades.push(Grade::new(
                    name,
                    *score,
                    pct,
                    grade,
                    wrongs.get(name).unwrap(),
                ));
                pcts.push(pct);
                grades_hist.entry(grade).and_modify(|x| *x += 1);
            }
        }

        let mut stats = Veg::table("Description|Value|Percent|Grade\n-|-:|-:|-");
        let scores_keys = scores.keys().collect::<Vec<_>>();
        let (min_pct, max_pct, _mean_pct, _sum, count) = calc_stats(&pcts);
        let mean_score = ((scores_sum as f32) / (count as f32) + 0.5) as usize;
        let mean_pct = (mean_score as f32) / (self.total as f32) * 100.0;
        let n_students = self.students.len();
        for (description, value, percent, grade) in [
            ("Number of students", n_students, None, None),
            ("Number of questions", self.questions, None, None),
            ("Total points", self.total, None, None),
            (
                "High score",
                **scores_keys.last().unwrap(),
                Some(max_pct),
                Some(letter_grade(max_pct)),
            ),
            (
                "Low score",
                **scores_keys.first().unwrap(),
                Some(min_pct),
                Some(letter_grade(min_pct)),
            ),
            (
                "Mean score",
                mean_score,
                Some(mean_pct),
                Some(letter_grade(mean_pct)),
            ),
        ] {
            stats.push(Stat::new(description, value, percent, grade));
        }
        for (letter, count) in grades_hist.iter() {
            stats.push(Stat::new(
                &letter.to_string(),
                *count,
                Some((*count as f32) / (n_students as f32) * 100.0),
                None,
            ));
        }

        format!(
            "# {}\n\n{}\n{}",
            self.description,
            grades.markdown().unwrap(),
            stats.markdown().unwrap(),
        )
    }
}

//--------------------------------------------------------------------------------------------------

/**
Individual grade in a grades table
*/
struct Grade {
    name: String,
    score: usize,
    pct: f32,
    grade: char,
    wrong: Vec<usize>,
}

impl Grade {
    /**
    Create an individual grade
    */
    fn new(name: &str, score: usize, pct: f32, grade: char, wrong: &[usize]) -> Box<Grade> {
        Box::new(Grade {
            name: name.to_string(),
            score,
            pct,
            grade,
            wrong: wrong.to_vec(),
        })
    }
}

impl veg::Table for Grade {
    /**
    Generate the table row
    */
    fn row(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.score.to_string(),
            fmt_percent(self.pct),
            self.grade.to_string(),
            self.wrong
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        ]
    }
}

//--------------------------------------------------------------------------------------------------

/**
Individual statistic in the summary table ([`Stats`])
*/
struct Stat {
    description: String,
    value: usize,
    pct: Option<f32>,
    grade: Option<char>,
}

impl Stat {
    /**
    Create a new statistic
    */
    fn new(description: &str, value: usize, pct: Option<f32>, grade: Option<char>) -> Box<Stat> {
        Box::new(Stat {
            description: description.to_string(),
            value,
            pct,
            grade,
        })
    }
}

impl veg::Table for Stat {
    /**
    Generate the table row
    */
    fn row(&self) -> Vec<String> {
        vec![
            self.description.clone(),
            self.value.to_string(),
            if let Some(pct) = &self.pct {
                fmt_percent(*pct)
            } else {
                String::new()
            },
            if let Some(grade) = &self.grade {
                grade.to_string()
            } else {
                String::new()
            },
        ]
    }
}
