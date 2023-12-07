use anyhow::{anyhow, Result};
use clap::Parser;
use quixote::*;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

#[cfg(unix)]
use pager::Pager;

//--------------------------------------------------------------------------------------------------

const README: &str = include_str!("../../README.md");

//--------------------------------------------------------------------------------------------------

#[derive(Parser)]
#[clap(
    about = "\
Quizzes and tests in Markdown

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

    /// Generate quiz(zes)
    #[arg(short, value_name = "PATH")]
    quizzes: Vec<PathBuf>,

    /// Grade quiz(zes)
    #[arg(short, value_name = "answers.json")]
    answers: Option<PathBuf>,

    /// Disable randomization
    #[arg(short = 'R', hide = true)]
    no_random: bool,

    /// Print readme
    #[arg(short)]
    readme: bool,

    #[arg(value_name = "PATH/GLOB")]
    arguments: Vec<PathBuf>,
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

    // Grade quiz(zes)
    if let Some(json) = &cli.answers {
        let answers = Answers::from(json)?;
        for path in &cli.arguments {
            let mut class = Class::from(path)?;
            class.grade(&answers);
            println!("{}", class.markdown());
        }
        return Ok(());
    }

    // Must have `-q` option (or `-d`)...
    if !cli.debug && cli.quizzes.is_empty() {
        return Err(anyhow!("Please provide a quiz path via the `-q` option"));
    }

    // Create quiz directories
    let quizzes = cli
        .quizzes
        .iter()
        .map(|x| {
            if cli.debug {
                Ok(x)
            } else {
                if !x.exists() {
                    std::fs::create_dir_all(x)?;
                }
                if x.is_dir() {
                    Ok(x)
                } else {
                    Err(anyhow!(format!("`{}`", x.display())))
                }
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
    let bank = Bank::new(&cli.arguments)?;
    if cli.debug {
        #[cfg(unix)]
        Pager::with_pager("bat -pl rust").setup();

        println!("{bank:#?}\n");
    }

    // Generate quiz(zes)
    for dir in &quizzes {
        let quiz = bank.quiz(!cli.no_random);
        let answers = quiz.answers();
        if cli.debug {
            println!("{quiz:#?}\n");
            println!("{answers:#?}\n");
        } else {
            let answers = quiz.answers();
            for (filename, content) in [
                ("quiz.md", &quiz.markdown()),
                ("answers.md", answers.markdown().as_ref().unwrap()),
                ("answers.md", &answers.json()),
            ] {
                write_file(&dir.join(filename), content)?;
            }
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
