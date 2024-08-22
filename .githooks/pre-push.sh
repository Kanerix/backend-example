BRANCH=$(git rev-parse --abbrev-ref HEAD)

if [[ $BRANCH == "main" | $BRANCH == "master" ]]; then
    cargo clippy -- -D warnings
elif [[ $BRANCH == "staging" ]]; then
    cargo clippy --
fi