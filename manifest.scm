(setenv "CARGO_TERM_COLOR" "always")

(use-modules
    (guix profiles)
    (gnu packages crates-io)
    ((gnu packages rust-apps)
        #:select (rust-analyzer)))

(concatenate-manifests
    (list (packages->manifest (list
        rust-analyzer
        rust-cargo-0.53
        rust-clippy-0.0))))
