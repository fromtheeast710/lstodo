host := `uname -a`

# run the program
r +ARG:
  cargo run -- {{ARG}}

# build release binary
b:
  cargo build --frozen --release --all-features

# reload environment
d:
  direnv reload