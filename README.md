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

My `.bashrc` file. Lives in: "C:\Users\\[user]\\.bashrc"

```shell
alias ls='ls -F --color --show-control-chars'

alias gs='git status'
alias ga='git add '
alias gaa='git add -A'
commitPush() { git commit -m "$1" && git push; }
alias gc=commitPush

# Careful! Will remove unpushed commits as well
alias reset='git reset --hard origin/main && git clean -fd'

alias pl='git pull -r'
alias push='git push'

# For CP :)
alias yolo='git add -A . && git commit -m "Daily programming practice" && git push'

# K8s
source <(kubectl completion bash)
alias k='kubectl'
complete -F __start_kubectl k

# Unity
alias gounity='cd "C:\Users\priet\unity-projects"'
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
