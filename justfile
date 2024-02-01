# Formats Rust files using rustfmt
_fmt-rustfmt *args:
    #!/usr/bin/env sh
    (
        for f in `find src -name '*.rs'`; do
            rustfmt $f {{ args }} &
        done
        wait
    )

# Formats Leptos components using leptosfmt
_fmt-leptosfmt *args:
    leptosfmt src/**/*.rs {{ args }}

# Formats justfile
_fmt-justfile:
    just --unstable --fmt

# Formats Rust files including Leptos component syntax
fmt:
    just _fmt-rustfmt
    just _fmt-leptosfmt
    just _fmt-justfile

_fmt-check-rustfmt:
    just _fmt-rustfmt --check

_fmt-check-leptosfmt:
    just _fmt-leptosfmt --check

_fmt-check-justfile:
    just --unstable --fmt --check

# Checks formatting
fmt-check:
    just _fmt-check-rustfmt
    just _fmt-check-leptosfmt
    just _fmt-check-justfile

# Checks examples formatting
fmt-check-examples:
    #!/usr/bin/env sh
    (cd examples/csr && just _fmt-check-rustfmt)
    (cd examples/csr && just _fmt-check-leptosfmt)
    (cd examples/ssr && just _fmt-check-rustfmt)
    (cd examples/ssr && just _fmt-check-leptosfmt)

# Lints source with Clippy
lint:
    cargo clippy -- -D warnings

# Lints examples with Clippy
lint-examples:
    #!/usr/bin/env sh
    (cd examples/csr && cargo clippy -- -D warnings)
    (cd examples/ssr && cargo clippy --features ssr -- -D warnings)
    (cd examples/ssr && cargo clippy --lib --features hydrate -- -D warnings)

ci:
    just fmt-check
    just lint
    just fmt-check-examples
    just lint-examples
