#!/usr/bin/env bash
set -ex

install_target() {
  rustup target add "$TARGET"
}

install_target
