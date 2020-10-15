#!/usr/bin/env bash
set -ex

. "$(dirname $0)/utils.sh"

case "$TRAVIS_OS_NAME" in
  windows)
    bin_name="gitall.exe"
    ;;
  *)
    bin_name="gitall"
    ;;
esac

build() {
  cargo build --target "$TARGET" --release
}

mk_tarball() {
  local tmpdir="$(mktemp -d)"
  echo "tmpdir = $tmpdir"
  local name="gitall-${TRAVIS_TAG}-${TARGET}"
  local staging="$tmpdir/$name"
  mkdir -p "$staging/complete"

  local out_dir="${TRAVIS_BUILD_DIR:-$PWD}/deployment"
  mkdir -p "$out_dir"

  local cargo_out="$(cargo_out_dir "target/$TARGET")"

  cp "target/$TARGET/release/${bin_name}" "$staging/"
  if [[ "$TRAVIS_OS_NAME" != "windows" ]]; then
    strip "$staging/${bin_name}"
  fi
  cp CHANGELOG.md COPYING LICENSE-MIT README.md UNLICENSE "$staging/"

  # copy shell completion files
  cp "$cargo_out"/gitall.{bash,elv,fish} "$cargo_out"/_gitall{,.ps1} "$staging/complete/"

  pushd "$tmpdir"
  if [[ "$TRAVIS_OS_NAME" = "windows" ]]; then
    7z a -r "$out_dir/$name.zip" "$name"
  else
    tar czf "$out_dir/$name.tar.gz" "$name"
  fi
  popd
  rm -rf "$tmpdir"
}

main() {
  build
  mk_tarball
}

main
