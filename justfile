build:
    cargo build --workspace --all-targets --features k8s-openapi/latest

static:
    cargo build --target x86_64-unknown-linux-musl

clean:
    cargo clean

test:
    cargo test --workspace -- --report-time -Z unstable-options

clippy:
    cargo clippy --workspace --all-targets  --features k8s-openapi/latest

c:
    cargo c

pedantic:
    cargo clippy --workspace --all-targets --features pedantic --features k8s-openapi/latest

update:
    cargo update

cbuild: clean build

ctest: clean test

rustfmt:
    cargo fmt --all --check

alias fmt := rustfmt

check: rustfmt update test clippy
fresh: clean update clippy test build

fixlock:
    rm Cargo.lock
    cargo update
    git add Cargo.lock

release:
    cargo release --no-confirm patch
