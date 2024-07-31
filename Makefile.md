# all

* clippy
* test
* build
* doc

# check

* outdated
* audit

# update

* update-toml
* update-lock

# run

* `target/release/{dirname}`

```
target/release/{dirname}
```

# clippy

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`

```
cargo clippy -- -D clippy::all
```

# test

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`

```
cargo test
```

# build

* `target/release/{dirname}`

# `target/release/{dirname}`

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`
* `README.md`
* `example/1/period-1.json`

```
cargo build --release
```

# `README.md`

* `t/README.md`
* `Cargo.toml`
* `CHANGELOG.md`
* `**/*.rs`
* `example/**/*`

```
cargo build --release
kapow {0} >{target}
```

# `example/1/period-1.json`

```
head -3 {target} >{target}.new
cat $(dirname {target})/answers.json |sed "s/\[\[/[/g;s/,false\]//g;s/,true\]//g" >$(dirname {target})/answers-class.json
jq -r '.students|keys.[]' {target} |perl -e 'chomp($a=`cat \$(dirname {target})/answers-class.json`);while(<>){chomp;print "    \"$_\":$a,\n"}' >>{target}.new
sed -i '$ s/,$//' {target}.new
echo '  }\n}' >>{target}.new
mv {target}.new {target}
rm -f $(dirname {target})/answers-class.json
```

# doc

```
cargo doc
```

# outdated

```
cargo outdated --exit-code=1
```

# audit

```
cargo audit
```

# update-toml

```
cargo upgrade -i
```

# update-lock

```
cargo update
```

# install

* `README.md`

```
cargo install --path .
```

# uninstall

```
cargo uninstall {dirname}
```

# install-deps

```
cargo install cargo-audit cargo-edit cargo-outdated cocomo dtg kapow tokei toml-cli
```

# clean

```
cargo clean
```

# cocomo

```bash -eo pipefail
tokei; echo
cocomo -o sloccount
cocomo
```

# publish

```
cargo publish
git push
git push --tags
```

# full

* update
* check
* all
* install

