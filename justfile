host := `uname -a`

# run the program
r *ARG:
  cargo run -- {{ARG}}

# expand rust code
e:
  cargo expand --bin lstodo | bat -l rs

# cargo build release binary
cb:
  cargo build --frozen --release --all-features

# remove the target dir
cc:
  cargo clean

# nix build release derivative
nb:
  nix build |& nom

# nix update flake.lock
nu:
  nix flake update

# nix check flake
nc:
  nix flake check

# nix show flake's info
ns:
  nix flake show

# reload environment
d:
  direnv reload