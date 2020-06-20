#!/bin/bash
set -eu
TARGET="thumbv7m-none-eabi"
cargo build --release --target="${TARGET}" --target-dir="${MYNEWT_PKG_BIN_DIR}"
cp "${MYNEWT_PKG_BIN_DIR}"/${TARGET}/debug/*.a "${MYNEWT_PKG_BIN_ARCHIVE}"
