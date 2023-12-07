# Usage

~~~text
$ quixote -h
!run:../target/release/quixote -h 2>&1
~~~

~~~text
$ quixote -V
!run:../target/release/quixote -V 2>&1
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
    quixote example/src/addition.md example/src/subtraction.md \\
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
!inc:../example/1/period-1.json
```

Answer key ([`answers.json`]):

```json
!inc:../example/1/answers.json
```

Run:

```bash
quixote -a example/1/answers.json example/1/period-1.json \\
>example/1/period-1.md
!run:../target/release/quixote -a ../example/1/answers.json \
../example/1/period-1.json >../example/1/period-1.md
```

Output ([`period-1.md`]):

```md
!inc:../example/1/period-1.md
```

!inc:../CHANGELOG.md

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

