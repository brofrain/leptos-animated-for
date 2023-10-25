# Formats Rust files using rustfmt
_fmt-rustfmt flag='':
    #!/usr/bin/env sh
    (
        for f in `find src -name '*.rs'`; do
            rustfmt $f {{ flag }} &
        done
        wait
    )

# Formats Leptos components using leptosfmt
_fmt-leptosfmt flag='':
    leptosfmt src/components/**/*.rs {{ flag }}

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

# Lints source with Clippy
lint:
    cargo clippy -- -D warnings

_vscode-fmt:
    # Using `leptosfmt --stdin --rustfmt` seems to add redundant newlines
    leptosfmt --stdin | rustfmt
