host := `uname -a`

# run the program
r *ARG:
  cargo run -- {{ARG}}

# expand rust code
e:
  cargo expand --bin lstodo | bat -l rs

# build release binary
bc:
  cargo build --frozen --release --all-features

bn:
  nix build |& nom

# remove the target dir
c:
  cargo clean

# reload environment
d:
  direnv reload