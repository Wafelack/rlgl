rlgl
====

Play red light, green light with files.

rlgl allows you to run a command when files change.

Installation
------------

You will need to have `git` and `cargo` installed to install rlgl.

```bash
$ git clone https://github.com/wafelack/rlgl.git
$ cargo install --path rlgl/
```

Usage
-----

rlgl reads a file list on the standard input and take a command to run as argument.

Read `rlgl --help` for more information about possible flags.

Examples
--------

Recompile your project each time a file is edited:

```bash
$ find -name *.rs | rlgl -qs cargo build
```
