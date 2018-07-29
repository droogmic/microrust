set -euxo pipefail

main() {

    if [ "$(mdbook -V)" != "mdbook v0.1.8" ]; then
        curl -LSfs https://japaric.github.io/trust/install.sh | \
            sh -s -- \
            --force \
            --git rust-lang-nursery/mdBook \
            --tag v0.1.8 \
            --target x86_64-unknown-linux-musl \
        || true
    fi

    if [ "$(mdbook -V)" != "mdbook v0.1.8" ]; then
        cargo install --target x86_64-unknown-linux-gnu --version 0.1.8 mdbook \
        || true
    fi

    if [ "$(mdbook -V)" != "mdbook v0.1.8" ]; then
        cargo install --force --target x86_64-unknown-linux-gnu --version 0.1.8 mdbook
    fi

    rustup target add thumbv6m-none-eabi

    pip install linkchecker --user

    # sanity check that a linker is present
    which arm-none-eabi-ld
}

main
