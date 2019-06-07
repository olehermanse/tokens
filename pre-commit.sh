#!/bin/sh
#
# Run format and test before commiting
#   cp -a pre-commit.sh .git/hooks/pre-commit

fail=0

git diff > before.diff
cargo fmt                   || fail="cargo fmt failed"
git diff > after.diff
diff before.diff after.diff || fail="you forgot to run cargo fmt"
rm before.diff after.diff

cargo build || fail="cargo build"
cargo doc   || fail="cargo doc"
cargo test  || fail="cargo test"

echo "Commit hook errors: "$fail
test $fail = 0
