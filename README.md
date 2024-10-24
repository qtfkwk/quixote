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
quixote 0.6.1
~~~

# Example

## Create Markdown files with questions and answers

* `example/src`
    * [`addition.md`]
    * [`multiple-answer.md`]
    * [`subtraction.md`]
    * [`word-problems.md`]
    * [`match.md`]
    * [`true-false.md`]

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
  "description": "Quiz 1 - Period 1",
  "students": {
    "Alvin Anderson":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Beatrice Brown":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Chris Clark":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Denise Dixon":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Erik Edwards":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Francesca Franklin":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "George Green":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Harriet Halloway":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Isabelle Izzard":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "James Jones":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Kelly Kennedy":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Lawrence Lewis":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Michelle Miller":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Nikolai Nixon":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Olga Olson":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Patrick Poole":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Qira Quinn":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Ralph Rogers":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Sally Smith":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Thomas Taylor":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Ursula Upton":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Victor Vogel":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Winnie Walters":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Xavier Xerxes":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Yasmine York":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]},
    "Zander Zuckerman":{"1":["A"],"2":["A"],"3":["C"],"4":["A"],"5":["D","A","C","B"],"6":["D","A","C","B"],"7":["C"],"8":["D"],"9":["A","B","C","D","E","F"],"10":["B"],"11":["C","G"],"12":["D"]}
  }
}
```

Answer key ([`answers.json`]):

```json
{"1":[["A"],false],"2":[["A"],false],"3":[["C"],false],"4":[["A"],false],"5":[["D","A","C","B"],true],"6":[["D","A","C","B"],true],"7":[["C"],false],"8":[["D"],false],"9":[["A","B","C","D","E","F"],false],"10":[["B"],false],"11":[["C","G"],false],"12":[["D"],false]}
```

Run:

```bash
quixote -a example/1/answers.json example/1/period-1.json \
>example/1/period-1.md
```

Output ([`period-1.md`]):

```md
# Quiz 1 - Period 1

| Name               | Score | Percent | Grade | Questions |
|--------------------|------:|--------:|-------|-----------|
| Alvin Anderson     |    24 |  100.0% | A     |           |
| Beatrice Brown     |    24 |  100.0% | A     |           |
| Chris Clark        |    24 |  100.0% | A     |           |
| Denise Dixon       |    24 |  100.0% | A     |           |
| Erik Edwards       |    24 |  100.0% | A     |           |
| Francesca Franklin |    24 |  100.0% | A     |           |
| George Green       |    24 |  100.0% | A     |           |
| Harriet Halloway   |    24 |  100.0% | A     |           |
| Isabelle Izzard    |    24 |  100.0% | A     |           |
| James Jones        |    24 |  100.0% | A     |           |
| Kelly Kennedy      |    24 |  100.0% | A     |           |
| Lawrence Lewis     |    24 |  100.0% | A     |           |
| Michelle Miller    |    24 |  100.0% | A     |           |
| Nikolai Nixon      |    24 |  100.0% | A     |           |
| Olga Olson         |    24 |  100.0% | A     |           |
| Patrick Poole      |    24 |  100.0% | A     |           |
| Qira Quinn         |    24 |  100.0% | A     |           |
| Ralph Rogers       |    24 |  100.0% | A     |           |
| Sally Smith        |    24 |  100.0% | A     |           |
| Thomas Taylor      |    24 |  100.0% | A     |           |
| Ursula Upton       |    24 |  100.0% | A     |           |
| Victor Vogel       |    24 |  100.0% | A     |           |
| Winnie Walters     |    24 |  100.0% | A     |           |
| Xavier Xerxes      |    24 |  100.0% | A     |           |
| Yasmine York       |    24 |  100.0% | A     |           |
| Zander Zuckerman   |    24 |  100.0% | A     |           |

| Description         | Value | Percent | Grade |
|---------------------|------:|--------:|-------|
| Number of students  |    26 |         |       |
| Number of questions |    12 |         |       |
| Total points        |    24 |         |       |
| High score          |    24 |  100.0% | A     |
| Low score           |    24 |  100.0% | A     |
| Mean score          |    24 |  100.0% | A     |
| A                   |    26 |  100.0% |       |
| B                   |     0 |    0.0% |       |
| C                   |     0 |    0.0% |       |
| D                   |     0 |    0.0% |       |
| F                   |     0 |    0.0% |       |

```

# Changelog

* 0.1.0 (2023-12-06): Initial release
    * 0.1.1 (2023-12-06): Save the quiz grading report to a file
    * 0.1.2 (2023-12-06): Fix typo
    * 0.1.3 (2023-12-07): Clean up readme and code
* 0.2.0 (2023-12-07): Streamline design: `-g` is now `-q`, improved tables; debug: bat pager, don't create quiz directories; fix typo
* 0.3.0 (2023-12-08): Change answers list to a task list; miscellaneous optimizations and code clean up; improved example; update dependencies
    * 0.3.1 (2023-12-08): Right align value and percent columns in summary table
    * 0.3.2 (2023-12-11): Unstring `Stat` properties; fix markdown table function; update dependencies
* 0.4.0 (2023-12-12): Use [`veg`] to generate markdown tables
* 0.5.0 (2024-04-22): Add match questions; update dependencies
    * 0.5.1 (2024-04-22): Update dependencies
* 0.6.0 (2024-07-31): Order true/false answers; fix makefile; fix changelog; update dependencies
    * 0.6.1 (2024-10-24): Update dependencies

[`veg`]: https://crates.io/crates/veg

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
[`match.md`]: example/src/match.md
[`quiz.md`]: example/1/quiz.md
[`answers.md`]: example/1/answers.md
[`answers.json`]: example/1/answers.json
[`period-1.json`]: example/1/period-1.json
[`period-1.md`]: example/1/period-1.md

[`glob`]: https://crates.io/crates/glob
[Documentation for the `glob::Pattern` struct]: https://docs.rs/glob/latest/glob/struct.Pattern.html

