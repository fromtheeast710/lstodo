(use-modules (guix packages)
             (guix download)
             (guix build-system cargo)
             (guix licenses))

(package
    (name "lstodo")
    (version "0.1.3")
    (source (origin
              (file-name "./."))
              (sha256
                (base32 "")))
    (build-system cargo-build-system)
    (description "Small and simple cli todo manager.")
    (license gpl3))
