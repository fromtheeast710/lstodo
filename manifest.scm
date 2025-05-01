(setenv "CARGO_TERM_COLOR" "always")

(use-modules
    (guix profiles)
    ((gnu packages rust-apps)
        ; #:select (rust-cargo)
        #:select (rust-analyzer))
    ((gnu packages crates-io)
        #:select (rust-clippy-0.0)))

(concatenate-manifests
    (list (packages->manifest (list
        ; rust-cargo
        rust-analyzer
        rust-clippy-0.0))))
