# LsTodo

A small and simple CLI todo manager, inspired by [sioodmy's todo](https://github.com/sioodmy/todo).

## New Features

+ Rewrite an existing todo.
+ Move the position of 2 todos.
+ Color tag important todos.

## Installation

```sh
# clone and move into the repo
git clone https://github.com/fromtheeast710/lstodo
cd lstodo
```

From there you can either use:

+ `makepkg -fsic` for Arch Linux users.
+ If you want to install via Cargo:
  ```sh
  cargo build --release --all-features
  cp target/release/lstodo /usr/bin
  ```
+ For installation with nix:
  ```sh
  nix profile install .#lstodo
  # or
  nix profile install fromtheeast710#watchers
  ```

## Help

```
LsTodo v0.1.2
Usage: lstodo [COMMAND] [ARGUMENTS]
Commands:
  [h]elp                      show this help message
  [l]ist                      list all tasks
  [a]dd [TASK]                add new task(s)
  [d]one [INDEX]              mark task(s) as done
  [u]ndo [INDEX]              mark task(s) as undone
  [r]emove [INDEX]            remove task(s)
  [s]ort                      sort completed and uncompleted tasks
  [n]ote [d/i/e/u/h] [INDEX]  highlight important task
  [c]hange [INDEX] [TASK]     change the content of a task
  [m]ove [INDEX] [INDEX]      switch the position of two tasks
  reset                       remove all tasks
```