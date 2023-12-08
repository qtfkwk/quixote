# Usage

~~~text
$ quixote -h
Quizzes and tests in Markdown

<https://crates.io/crates/quixote> / <https://github.com/qtfkwk/quixote>

---

Usage: quixote [OPTIONS] [PATH/GLOB]...

Arguments:
  [PATH/GLOB]...  

Options:
  -q <PATH>              Generate quiz(zes)
  -a <answers.json>      Grade quiz(zes)
  -r                     Print readme
  -h, --help             Print help
  -V, --version          Print version
~~~

~~~text
$ quixote -V
quixote 0.3.1
~~~

# Example

## Create Markdown files with questions and answers

* `example/src`
    * [`addition.md`]
    * [`multiple-answer.md`]
    * [`subtraction.md`]
    * [`word-problems.md`]

- Use multiple files to organize questions and enable generating quizzes from
  any subset.
- Write questions in one or more paragraphs, optionally with any Markdown span
  syntax and/or tables, images, lists, etc.
- Place answers after all question content as an unordered list with the correct
  answer(s) in bold/strong.
- Use a *rule* (`---`) between questions.

## Generate a quiz

```bash
quixote example/src -q example/1
```

* `example/1`
    * [`quiz.md`]: Quiz for students
    * [`answers.md`]: Quiz with answers
    * [`answers.json`]: Answer key

- The quiz includes all questions and answers, both in random order.
- To generate a quiz from a subset of source files, use one or more paths or
  globs to specify it; for example, to only include questions from
  [`addition.md`] and [`subtraction.md`], the command is:

    ```bash
    quixote example/src/addition.md example/src/subtraction.md \
    -q example/addition-subtraction
    ```

- Path/glob arguments:
    - Absolute or relative path to a file or directory
    - Use your shell's globbing.
    - If using the `-q` option and an argument is a directory, it converts to
      `directory/**/*.md` to include all `*.md` files under `directory/`.
    - Use built-in globbing by properly quoting and/or escaping the argument so
      it is not interpreted by your shell.
      See the [reference section on globbing below](#globbing) for more details.

## Grade a quiz

Completed quiz ([`period-1.json`]):

```json
{
  "description":"Quiz 1 - Period 1",
  "students":{
    "Alvin Anderson":{"1":["A","B","C","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Beatrice Brown":{"1":["A","B","C","D","E","F"],"2":["B"],"3":["D"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Chris Clark":{"1":["A","B","C","D","E","F"],"2":["B"],"3":["D"],"4":["A"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Denise Dixon":{"1":["B","C","D","E"],"2":["A"],"3":["C"],"4":["D"],"5":["C"],"6":["F"],"7":["A"],"8":["B"]},
    "Erik Edwards":{"1":["A","B","C","D","E","F"],"2":["B"],"3":["C"],"4":["B"],"5":["B"],"6":[],"7":["A"],"8":["C"]},
    "Francesca Franklin":{"1":["A"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["B"],"8":["B"]},
    "George Green":{"1":["A","C","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["B","C","D"],"7":["A"],"8":["B"]},
    "Harriet Halloway":{"1":[],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":[],"7":["A"],"8":["B"]},
    "Isabelle Izzard":{"1":["A","B","C"],"2":["B"],"3":["B"],"4":["B"],"5":["C"],"6":["B","F"],"7":["A"],"8":["B"]},
    "James Jones":{"1":["D","E","F"],"2":["C"],"3":["D"],"4":["A"],"5":["A"],"6":[],"7":["B"],"8":["B"]},
    "Kelly Kennedy":{"1":["B","D","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["B","C","D","E"],"7":["D"],"8":["D"]},
    "Lawrence Lynch":{"1":["B","C","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Michelle Miller":{"1":["A","C","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Nikolai Nixon":{"1":["A","B","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Olga Olson":{"1":["A","B","C","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Patrick Poole":{"1":["A","B","C","D","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Qira Quinn":{"1":["A","B","C","D","E"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Ralph Rogers":{"1":["A","B","C","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Sally Smith":{"1":["A","B","C","D","E","F"],"2":["A"],"3":["A"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Thomas Taylor":{"1":["A","B","C","D","E","F"],"2":["A"],"3":["C"],"4":["C"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Ursula Upton":{"1":["A","B","C","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["C"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Victor Vogel":{"1":["A","B","C","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["F"],"7":["A"],"8":["B"]},
    "Winnie Walters":{"1":["A","B","C","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]},
    "Xavier Xerxes":{"1":["A","B","C","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["B"],"8":["B"]},
    "Yasmine York":{"1":["A","B","C","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["A"]},
    "Zander Zuckerman":{"1":["C","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]}
  }
}
```

Answer key ([`answers.json`]):

```json
{"1":["A","B","C","D","E","F"],"2":["A"],"3":["C"],"4":["D"],"5":["B"],"6":["A","F"],"7":["A"],"8":["B"]}
```

Run:

```bash
quixote -a example/1/answers.json example/1/period-1.json \
>example/1/period-1.md
```

Output ([`period-1.md`]):

```md
# Quiz 1 - Period 1

| Name               | Score | Percent | Grade | Questions           |
|--------------------|------:|--------:|-------|---------------------|
| Ralph Rogers       |    14 |  100.0% | A     |                     |
| Winnie Walters     |    14 |  100.0% | A     |                     |
| Alvin Anderson     |    13 |   92.9% | A     | 1                   |
| Lawrence Lynch     |    13 |   92.9% | A     | 1                   |
| Michelle Miller    |    13 |   92.9% | A     | 1                   |
| Nikolai Nixon      |    13 |   92.9% | A     | 1                   |
| Olga Olson         |    13 |   92.9% | A     | 1                   |
| Patrick Poole      |    13 |   92.9% | A     | 1                   |
| Qira Quinn         |    13 |   92.9% | A     | 1                   |
| Sally Smith        |    13 |   92.9% | A     | 3                   |
| Thomas Taylor      |    13 |   92.9% | A     | 4                   |
| Ursula Upton       |    13 |   92.9% | A     | 5                   |
| Victor Vogel       |    13 |   92.9% | A     | 6                   |
| Xavier Xerxes      |    13 |   92.9% | A     | 7                   |
| Yasmine York       |    13 |   92.9% | A     | 8                   |
| Beatrice Brown     |    12 |   85.7% | B     | 2, 3                |
| Zander Zuckerman   |    12 |   85.7% | B     | 1                   |
| Chris Clark        |    11 |   78.6% | C     | 2, 3, 4             |
| Denise Dixon       |    10 |   71.4% | C     | 1, 5, 6             |
| Erik Edwards       |     9 |   64.3% | D     | 2, 4, 6, 8          |
| Francesca Franklin |     8 |   57.1% | F     | 1, 7                |
| George Green       |     7 |   50.0% | F     | 1, 6                |
| Harriet Halloway   |     6 |   42.9% | F     | 1, 6                |
| Isabelle Izzard    |     5 |   35.7% | F     | 1, 2, 3, 4, 5, 6    |
| James Jones        |     4 |   28.6% | F     | 1, 2, 3, 4, 5, 6, 7 |
| Kelly Kennedy      |     3 |   21.4% | F     | 1, 6, 7, 8          |

| Description         | Value | Percent | Grade |
|---------------------|------:|--------:|-------|
| Number of students  |    26 |         |       |
| Number of questions |     8 |         |       |
| Total points        |    14 |         |       |
| High score          |    14 |  100.0% | A     |
| Low score           |     3 |   21.4% | F     |
| Mean score          |    11 |   78.6% | C     |
| A                   |    15 |   57.7% |       |
| B                   |     2 |    7.7% |       |
| C                   |     2 |    7.7% |       |
| D                   |     1 |    3.8% |       |
| F                   |     6 |   23.1% |       |

```

# Changelog

* 0.1.0 (2023-12-06): Initial release
    * 0.1.1 (2023-12-06): Save the quiz grading report to a file
    * 0.1.2 (2023-12-06): Fix typo
    * 0.1.3 (2023-12-07): Clean up readme and code
* 0.2.0 (2023-12-07): Streamline design: `-g` is now `-q`, improved tables;
  debug: bat pager, don't create quiz directories; fix typo
* 0.3.0 (2023-12-08): Change answers list to a task list; miscellaneous
  optimizations and code clean up; improved example; update dependencies
    * 0.3.1 (2023-12-08): Right align value and percent columns in summary table

# Reference

## Globbing

* `?` matches any single character.
* `*` matches any (possibly empty) sequence of characters.
* `**` matches the current directory and arbitrary subdirectories.
  This sequence must form a single path component, so both `**a` and `b**` are
  invalid and will result in an error.
  A sequence of more than two consecutive `*` characters is also invalid.
* `[...]` matches any character inside the brackets.
  Character sequences can also specify ranges of characters, as ordered by
  Unicode, so e.g. `[0-9]` specifies any character between 0 and 9 inclusive.
  An unclosed bracket is invalid.
* `[!...]` is the negation of `[...]`, i.e. it matches and characters not in the
  brackets.
* The metacharacters `?`, `*`, `[`, `]` can be matched by using brackets (e.g.
  `[?]`).
  When a `]` occurs immediately following `[` or `[!` then it is interpreted as
  being part of, rather then ending, the character set, so `]` and NOT `]` can
  be matched by `[]]` and `[!]]` respectively.
  The `-` character can be specified inside a character sequence pattern by
  placing it at the start or the end, e.g. `[abc-]`.

Source: [Documentation for the `glob::Pattern` struct]

*See also `glob` on [`crates.io`][`glob`] or [`docs.rs`](https://docs.rs/glob).*

[`addition.md`]: example/src/addition.md
[`multiple-answer.md`]: example/src/multiple-answer.md
[`subtraction.md`]: example/src/subtraction.md
[`word-problems.md`]: example/src/word-problems.md
[`quiz.md`]: example/1/quiz.md
[`answers.md`]: example/1/answers.md
[`answers.json`]: example/1/answers.json
[`period-1.json`]: example/1/period-1.json
[`period-1.md`]: example/1/period-1.md

[`glob`]: https://crates.io/crates/glob
[Documentation for the `glob::Pattern` struct]: https://docs.rs/glob/latest/glob/struct.Pattern.html

