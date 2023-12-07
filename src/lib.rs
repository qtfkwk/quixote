use alpha_counter::AlphaCounter;
use anyhow::{anyhow, Result};
use glob::glob;
use pulldown_cmark as pd;
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::*;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};
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
Calculate basic statistics on a slice of f32 numbers
*/
fn f32_stats(v: &[f32]) -> (f32, f32, f32, f32, usize) {
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

const LETTER_GRADES: [(f32, char); 5] = [
    (90.0, 'A'),
    (80.0, 'B'),
    (70.0, 'C'),
    (60.0, 'D'),
    (0.0, 'F'),
];

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
    fn new(questions: &[Question], shuffle: bool) -> Quiz {
        let mut questions = questions.to_vec();

        if shuffle {
            let mut rng = thread_rng();
            questions.shuffle(&mut rng);
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
    Generate quiz [`Answers`]
    */
    pub fn answers(&self) -> Answers {
        Answers::new(self)
    }
}

//--------------------------------------------------------------------------------------------------

/**
Quiz answers

This struct is used in two ways:

1. Generated via [`Quiz.answers()`] during quiz generation in order to generate
   content for the `answers.json` and `answers.md` files
2. Loaded from a saved `answers.json` file for grading a quiz
*/
#[derive(Debug)]
pub struct Answers {
    answers: BTreeMap<usize, Vec<String>>,
    markdown: Option<String>,
}

impl Answers {
    fn new(quiz: &Quiz) -> Answers {
        let answers: BTreeMap<usize, Vec<String>> = quiz
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

        let markdown = Some(
            quiz.questions
                .iter()
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
                                    format!("    * **{letter}) {}**\n\n", x.content)
                                } else {
                                    format!("    * {letter}) {}\n\n", x.content)
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
    Load [`Answers`] from a JSON file
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
            wrongs.insert(
                name,
                wrong,
                /*
                if wrong.is_empty() {
                    String::from("none")
                } else {
                    wrong
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                },
                */
            );
            if let Some(students) = scores.get_mut(score) {
                students.push(name);
            } else {
                scores.insert(*score, vec![name]);
            }
            scores_sum += score;
        }

        let mut pcts = vec![];
        let mut grades = Grades::new();
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
            }
        }

        let mut stats = Stats::new();
        let scores_keys = scores.keys().collect::<Vec<_>>();
        let (min_pct, max_pct, mean_pct, _sum, count) = f32_stats(&pcts);
        let mean_score = (scores_sum as f32) / (count as f32);
        stats.push(Stat::new(
            "Number of questions",
            &self.questions.to_string(),
        ));
        stats.push(Stat::new("Total points", &self.total.to_string()));
        stats.push(Stat::new(
            "High score",
            &format!(
                "{}, {max_pct:.1}%, {}",
                scores_keys.last().unwrap(),
                letter_grade(max_pct),
            ),
        ));
        stats.push(Stat::new(
            "Low score",
            &format!(
                "{}, {min_pct:.1}%, {}",
                scores_keys.first().unwrap(),
                letter_grade(min_pct),
            ),
        ));
        stats.push(Stat::new(
            "Mean score",
            &format!("{mean_score}, {mean_pct:.1}%, {}", letter_grade(mean_pct)),
        ));
        for (letter, count) in grades.histogram() {
            stats.push(Stat::new(&letter.to_string(), &count.to_string()));
        }

        format!(
            "# {}\n\n{}\n\n{}\n\n",
            self.description,
            grades.markdown(),
            stats.markdown()
        )
    }
}

//--------------------------------------------------------------------------------------------------

struct Grade {
    name: String,
    score: usize,
    pct: f32,
    grade: char,
    wrong: Vec<usize>,
}

impl Grade {
    fn new(name: &str, score: usize, pct: f32, grade: char, wrong: &[usize]) -> Grade {
        Grade {
            name: name.to_string(),
            score,
            pct,
            grade,
            wrong: wrong.to_vec(),
        }
    }

    fn markdown(&self) -> Vec<String> {
        vec![
            self.name.clone(),
            self.score.to_string(),
            format!("{:.1}%", self.pct),
            self.grade.to_string(),
            self.wrong
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        ]
    }
}

struct Grades {
    grades: Vec<Grade>,
}

impl Grades {
    fn new() -> Grades {
        Grades { grades: vec![] }
    }

    fn push(&mut self, grade: Grade) {
        self.grades.push(grade);
    }

    fn markdown(&self) -> String {
        let mut rows = vec!["Name,Score,Percent,Grade,Wrong"
            .split(',')
            .map(|x| x.to_string())
            .collect::<Vec<_>>()];
        rows.append(&mut self.grades.iter().map(|x| x.markdown()).collect::<Vec<_>>());
        md_table(&rows, &[1, 2])
    }

    fn histogram(&self) -> BTreeMap<char, usize> {
        let mut r = LETTER_GRADES
            .iter()
            .map(|(_, x)| (*x, 0))
            .collect::<BTreeMap<char, usize>>();
        for grade in &self.grades {
            r.entry(grade.grade).and_modify(|x| *x += 1);
        }
        r
    }
}

//--------------------------------------------------------------------------------------------------

struct Stat {
    description: String,
    value: String,
}

impl Stat {
    fn new(description: &str, value: &str) -> Stat {
        Stat {
            description: description.to_string(),
            value: value.to_string(),
        }
    }

    fn markdown(&self) -> Vec<String> {
        vec![self.description.clone(), self.value.clone()]
    }
}

struct Stats {
    stats: Vec<Stat>,
}

impl Stats {
    fn new() -> Stats {
        Stats { stats: vec![] }
    }

    fn push(&mut self, stat: Stat) {
        self.stats.push(stat);
    }

    fn markdown(&self) -> String {
        let mut rows = vec!["Description,Value"
            .split(',')
            .map(|x| x.to_string())
            .collect::<Vec<_>>()];
        rows.append(&mut self.stats.iter().map(|x| x.markdown()).collect::<Vec<_>>());
        md_table(&rows, &[])
    }
}

//--------------------------------------------------------------------------------------------------

fn measure(rows: &[Vec<String>]) -> Vec<usize> {
    let mut r = rows[0].iter().map(|_| 0).collect::<Vec<_>>();
    for row in rows.iter() {
        for (i, cell) in row.iter().enumerate() {
            r[i] = r[i].max(cell.chars().collect::<Vec<_>>().len());
        }
    }
    r
}

fn md_table(rows: &[Vec<String>], right: &[usize]) -> String {
    let widths = measure(rows);
    let mut r = vec![];
    for (i, row) in rows.iter().enumerate() {
        let mut s = vec![];
        for (j, cell) in row.iter().enumerate() {
            let sep = if j == 0 { "" } else { " | " };
            s.push(if right.contains(&j) {
                format!("{sep}{cell:>0$}", widths[j])
            } else {
                format!("{sep}{cell:<0$}", widths[j])
            });
        }
        r.push(format!("| {} |\n", s.join("")));
        if i == 0 {
            let mut s = vec![];
            for (j, _cell) in row.iter().enumerate() {
                s.push(format!(
                    "{}{}{}",
                    if j == 0 { "" } else { "|-" },
                    "-".repeat(widths[j]),
                    if right.contains(&j) { ':' } else { '-' }
                ));
            }
            r.push(format!("|-{}|\n", s.join("")));
        }
    }
    r.join("")
}
