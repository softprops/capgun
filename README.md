# capgun

[![Build Status](https://travis-ci.org/softprops/capgun.svg)](https://travis-ci.org/softprops/capgun)

> fire when ready

Capgun is a simple utility that watches files and fires a specified command when they do

## install

```bash
$ cargo build --release
$ chmod u+x target/release/capgun
$ cp target/release/capgun SOMEWHERE_ON_YOUR_PATH
```

## usage

restart your cargo tests when src changes

```bash
$ capgun -c 'cargo tests' src
```

rerun your cargo tests when src changes

```bash
$ capgun -c 'cargo run' src
```

usage is not limited to rust workflows so use your imagination



Doug Tangren (softprops) 2015
