# rust-play

A playground for the Rust systems programming language and some other personal stuff that I store here for some reason.

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

This goes on: "C:\Users\\[user]\\.bashrc"

```shell
alias ls='ls -F --color --show-control-chars'

alias gs='git status'
alias ga='git add '
alias gaa='git add -A'
alias gc='git commit -m '

alias pl='git pull -r'
alias ps='git push origin master'

alias yolo='git add -A . && git commit -m "minor changes" && git push origin master'
```

----

### Git push requires username and password?

```shell
git remote set-url origin git@github.com:username/repo.git
```

----

### Add shortcut (on Windows) to automate going to git repo manually

1) Run `C:\Users\Axel\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup`
2) Create shortcut with Target:
```shell
C:\Programs\Git\git-bash.exe --cd="C:\Users\Axel\Desktop\algorithms-java" -c "yolo;echo -e '\e[1m--------------------';pl;echo -e '-------------------\e[22m-';yolo;echo -e '\e[32m*** Happy Coding! ***';exec $SHELL"
```
