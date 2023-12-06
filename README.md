# About

Generate quizzes from Markdown

# Usage

~~~text
$ quixote -h
Generate quizzes from Markdown

<https://crates.io/crates/quixote> / <https://github.com/qtfkwk/quixote>

---

Usage: quixote [OPTIONS] [PATH/GLOB]...

Arguments:
  [PATH/GLOB]...  One or more paths/globs

Options:
  -g <DIRECTORY>         Generate one or more quizzes
  -a <answers.json>      Grade completed quiz(zes)
  -r                     Print readme
  -h, --help             Print help
  -V, --version          Print version
~~~

~~~text
$ quixote -V
quixote 0.1.1
~~~

# Example

Create one or more Markdown files with quiz questions and answers:

* `example/src`
    * [`addition.md`]
    * [`mulitple-answer.md`]
    * [`subtraction.md`]
    * [`word-problems.md`]

- Use multiple files to organize questions and enable generating quizzes from
  any subset.
- Write questions in one or more paragraphs, optionally with any Markdown span
  syntax and/or tables, images, lists, etc.
- Place answers after all question content as an unordered list with the correct
  answer(s) in bold/strong.
- Use a *rule* (`---`) between questions.

Generate a quiz:

```bash
quixote example/src -g example/1
```

* `example/1`
    * [`quiz.md`]: Quiz for students
    * [`answers.md`]: Quiz with answers
    * [`answers.json`]: Answer key

- The quiz includes all questions and answers, both in random order.
- To generate a quiz from a subset of source files, use one or more paths or
  globs to specify it; for example, to generate a quiz with only questions from
  [`addition.md`] and [`subtraction.md`], the command is:

    ```bash
    quixote example/src/addition.md example/src/subtraction.md \
    -g example/addition-subraction
    ```

- Path/glob arguments:
    - Absolute or relative path to a file or directory
    - Use your shell's globbing
    - If using the `-g` option and an argument is a directory, it converts to
      `directory/**/*.md` to include all `*.md` files under `directory/`.
    - Use built-in globbing by properly quoting and/or escaping the argument so
      it is not interpreted by your shell.
      See the [reference section on globbing below](#globbing) for more details.

[`glob`]: https://crates.io/crates/glob

Grade a quiz:

* Completed quiz ([`period-1.json`]):

    ```json
    {
      "description":"Quiz 1 - Period 1",
      "students":{
        "Alvin Anderson":{"1":["A"],"2":["C"],"3":["A"],"4":["B"],"5":["A"],"6":["A"],"7":["A"],"8":["A"]},
        "Beatrice Brown":{"1":["E","G"],"2":["C"],"3":["E"],"4":["B"],"5":["C"],"6":["A"],"7":["A","B","C","D","E","F"],"8":["A"]},
        "Chris Clark":{"1":["E"],"2":["C"],"3":["E"],"4":["B"],"5":["C"],"6":["A"],"7":["A","B","D","E","F"],"8":["A"]},
        "Denise Dixon":{"1":["E","G"],"2":["C"],"3":["E"],"4":["B"],"5":["C"],"6":["A"],"7":["A","B","C"],"8":["A"]},
        "Erik Edwards":{"1":["E","G"],"2":["D"],"3":["B"],"4":["A"],"5":["C"],"6":["C"],"7":["A","B","C","D","E","F"],"8":["B"]}
      }
    }
    ```

* Answer key ([`answers.json`]):

    ```json
    {"1":["E","G"],"2":["C"],"3":["E"],"4":["B"],"5":["C"],"6":["A"],"7":["A","B","C","D","E","F"],"8":["A"]}
    ```

```bash
quixote -a example/1/answers.json example/1/period-1.json >example/1/period-1.md
```

[`period-1.md`]

```md
# Quiz 1 - Period 1

Name | Score | Percentage | Grade | Wrong
-----|------:|-----------:|-------|-------
Beatrice Brown | 14 | 100.0% | A | none
Chris Clark | 12 | 85.7% | B | 1, 7
Denise Dixon | 11 | 78.6% | C | 7
Erik Edwards | 9 | 64.3% | D | 2, 3, 4, 6, 8
Alvin Anderson | 4 | 28.6% | F | 1, 3, 5, 7

Description        | Value
-------------------|------------
Number of problems | 8
Total points       | 14
High score         | 14 (100.0%)
Low score          | 4 (28.6%)
Mean score         | 10.0 (71.4%)
A                  | 1
B                  | 1
C                  | 1
D                  | 1
F                  | 1

```

# Changelog

* 0.1.0 (2023-12-06): Initial release
* 0.1.1 (2023-12-06): Save the quiz grading report to a file

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

*See also `glob` on [`crates.io`](https://crates.io/crates/glob),
[`docs.rs`](https://docs.rs/glob), or specifically the documentation for the
[`glob::Pattern` struct](https://docs.rs/glob/latest/glob/struct.Pattern.html).*

[`addition.md`]: example/src/addition.md
[`mulitple-answer.md`]: example/src/mulitple-answer.md
[`subtraction.md`]: example/src/subtraction.md
[`word-problems.md`]: example/src/word-problems.md
[`quiz.md`]: example/1/quiz.md
[`answers.md`]: example/1/answers.md
[`answers.json`]: example/1/answers.json
[`period-1.json`]: example/1/period-1.json
[`period-1.md`]: example/1/period-1.md

