#!/usr/bin/env bash
set -ex

. "$(dirname $0)/utils.sh"

cargo build --target "$TARGET" --verbose

# show build.rs stderr
set +x
stderr="$(find "target/$TARGET/debug" -name stderr -print0 | xargs -0 ls -t | head -n1)"
if [ -s "$stderr" ]; then
  echo "===== $stderr ====="
  cat "$stderr"
  echo "====="
fi
set -x

file "target/$TARGET/debug/gitall"

# make sure we made completion scripts
outdir="$(cargo_out_dir "target/$TARGET/debug")"
file "$outdir"/_gitall* "$outdir"/gitall.*

cargo test --target "$TARGET" --verbose
