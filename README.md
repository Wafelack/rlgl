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

rlgl reads a files list on the standard input and takes a command to run as argument.

Read `rlgl --help` for more information about possible flags.

Examples
--------

Recompile your project each time a file is edited:

```bash
$ find -name *.rs | rlgl -qs cargo build
```

Link your dotfiles when they are edited (with [`rdfm`](https://github.com/wafelack/rdfm)):
```bash
$ sed '/^#/d' ~/.config/.dotfiles/dotfiles.rdfm \ # Remove comments from dotfiles.rdfm
  | sed -r '/^\s*$/d' \ # Remove blank lines
  | awk -F'=' '{ print $1 }' \ # Split on '=' and take the first part.
  | rlgl -qs rdfm link
```
