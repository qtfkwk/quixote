use anyhow::{anyhow, Result};
use clap::Parser;
use quixote::*;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};

#[cfg(unix)]
use pager::Pager;

//--------------------------------------------------------------------------------------------------

const README: &str = include_str!("../../README.md");

//--------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[clap(
    about = "\
Generate quizzes from Markdown

<https://crates.io/crates/quixote> / <https://github.com/qtfkwk/quixote>

---\
    ",
    version,
    max_term_width = 80
)]
struct Cli {
    /// Debug
    #[arg(short, hide = true)]
    debug: bool,

    /// Generate one or more quizzes
    #[arg(short, value_name = "DIRECTORY")]
    generate_quizzes: Vec<PathBuf>,

    /// Grade completed quiz(zes)
    #[arg(short, value_name = "answers.json")]
    answers_json: Option<PathBuf>,

    /// Print readme
    #[arg(short)]
    readme: bool,

    /// One or more paths/globs
    #[arg(value_name = "PATH/GLOB")]
    input_files: Vec<PathBuf>,
}

//--------------------------------------------------------------------------------------------------

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Print readme
    if cli.readme {
        #[cfg(unix)]
        Pager::with_pager("bat -pl md").setup();

        print!("{README}");
        return Ok(());
    }

    // Grade against answers.json
    if let Some(answers_json) = &cli.answers_json {
        let answers = Answers::from(answers_json)?;
        for path in &cli.input_files {
            let mut class = Class::from(path)?;
            class.grade(&answers);
            println!("{}", class.markdown());
        }
        return Ok(());
    }

    // Must have `-g` option (or `-d`)...
    if !cli.debug && cli.generate_quizzes.is_empty() {
        return Err(anyhow!("Please provide a quiz path via the `-g` option"));
    }

    // Create quiz directories
    let quizzes = cli
        .generate_quizzes
        .iter()
        .map(|x| {
            if !x.exists() {
                std::fs::create_dir_all(x)?;
            }
            if x.is_dir() {
                Ok(x)
            } else {
                Err(anyhow!(format!("`{}`", x.display())))
            }
        })
        .collect::<Vec<_>>();
    let errors: Vec<_> = quizzes.iter().filter_map(|x| x.as_ref().err()).collect();
    if !errors.is_empty() {
        return Err(anyhow!(format!(
            "Could not use arguments as quiz directories: {}",
            errors
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        )));
    }
    let quizzes: Vec<_> = quizzes.into_iter().map(|x| x.unwrap()).collect();

    // Create question bank
    let bank = Bank::new(&cli.input_files)?;
    if cli.debug {
        println!("{bank:#?}\n");
    }

    // Generate quiz(zes)
    for dir in &quizzes {
        let quiz = bank.quiz();
        if cli.debug {
            println!("{quiz:#?}\n");
        } else {
            let (answers, answers_md) = quiz.answers();
            write_file(&dir.join("quiz.md"), &quiz.markdown())?;
            write_file(&dir.join("answers.md"), &answers_md)?;
            write_file(&dir.join("answers.json"), &answers.to_string())?;
        }
    }

    Ok(())
}

//--------------------------------------------------------------------------------------------------

fn write_file(path: &Path, data: &str) -> Result<()> {
    let f = File::create(path)?;
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes())?;
    Ok(())
}
