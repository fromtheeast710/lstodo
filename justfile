host := `uname -a`

# run the program
r +ARG:
  cargo run -- {{ARG}}

# expand rust code
e:
  cargo expand --bin lstodo | bat -l rs

# build release binary
b:
  cargo build --frozen --release --all-features

# remove the target dir
c:
  cargo clean

# reload environment
d:
  direnv reload