set -euxo pipefail

MDBOOK_VER=0.2.1

main() {

    if [ "$(mdbook -V)" != "mdbook v$MDBOOK_VER" ]; then
        curl -LSfs https://japaric.github.io/trust/install.sh | \
            sh -s -- \
            --force \
            --git rust-lang-nursery/mdBook \
            --tag v$MDBOOK_VER \
            --target x86_64-unknown-linux-musl \
        || true
    fi

    if [ "$(mdbook -V)" != "mdbook v$MDBOOK_VER" ]; then
        cargo install --target x86_64-unknown-linux-gnu --version $MDBOOK_VER mdbook \
        || true
    fi

    if [ "$(mdbook -V)" != "mdbook v$MDBOOK_VER" ]; then
        cargo install --force --target x86_64-unknown-linux-gnu --version $MDBOOK_VER mdbook
    fi

    rustup target add thumbv6m-none-eabi

    pip install linkchecker --user

    # sanity check that a linker is present
    which arm-none-eabi-ld
}

main
