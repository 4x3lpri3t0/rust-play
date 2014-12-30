# rust-play

A playground for the Rust systems programming language

----

### Setting up a new project with Cargo

```shell
$ cd ~/projects
$ cargo new [project_name] --bin
$ cd [project_name]
```

* `--bin` flag, since we're making a binary, rather than a library.

* `cargo run` is kind of like `cargo build`, but it also then runs the produced executable.

* You can run all of your tests with `cargo test`

----

### Bash Aliases

(This has nothing to do with Rust)

This goes on: "C:\Users\\[user]\\.bashrc"

```shell
alias ls='ls -F --color --show-control-chars'

alias gs='git status'
alias ga='git add '
alias gaa='git add -A'
alias gc='git commit -m '

alias pl='git pull -r'
alias ps='git push origin master'

alias yolo='git commit -am "minor changes" && git push origin master'
```

----

### Git push requires username and password?

```shell
git remote set-url origin git@github.com:username/repo.git
```