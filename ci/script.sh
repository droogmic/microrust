set -euxo pipefail

main() {
    # test that building the book works
    mdbook build

    linkchecker book

    # first (fast) pass: check that examples compile
    for chapter in $(echo src/*); do
        if [ ! -f $chapter/Cargo.toml ]; then
            continue
        fi

        pushd $chapter
        cargo check
        popd
    done

    # second (slow) pass: check that examples link
    for chapter in $(echo src/*); do
        if [ ! -f $chapter/Cargo.toml ]; then
            continue
        fi

        pushd $chapter
        cargo build
        cargo build --release
        popd
    done
}

if [ $TRAVIS_BRANCH != master ]; then
    main
fi
