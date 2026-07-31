#!/usr/bin/env bash
# Official AIR.SKILLS Installation Script for Linux/macOS
set -euo pipefail

REPO="Chethankumar443/AIR-SKILLS"
VERSION="1.0.0"

echo "────────────────────────────────────────────────"
echo "        AIR.SKILLS Installer (v${VERSION})"
echo "────────────────────────────────────────────────"

# Detect OS & Arch
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "${ARCH}" in
  x86_64) ARCH="x86_64" ;;
  arm64|aarch64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: ${ARCH}"; exit 1 ;;
esac

case "${OS}" in
  darwin) TARGET="${ARCH}-apple-darwin" ;;
  linux) TARGET="${ARCH}-unknown-linux-gnu" ;;
  *) echo "Unsupported OS: ${OS}"; exit 1 ;;
esac

BINARY_URL="https://github.com/${REPO}/releases/download/v${VERSION}/air-${TARGET}.tar.gz"
DEST_DIR="/usr/local/bin"

echo "Downloading AIR CLI binary for ${TARGET}..."
TMP_DIR="$(mktemp -d)"
curl -sSL "${BINARY_URL}" | tar -xz -C "${TMP_DIR}"

if [ -w "${DEST_DIR}" ]; then
  mv "${TMP_DIR}/air" "${DEST_DIR}/air"
else
  echo "Installing to ${DEST_DIR} requires sudo permissions:"
  sudo mv "${TMP_DIR}/air" "${DEST_DIR}/air"
fi

rm -rf "${TMP_DIR}"
chmod +x "${DEST_DIR}/air"

echo "────────────────────────────────────────────────"
echo " ✓ AIR.SKILLS CLI successfully installed to ${DEST_DIR}/air"
echo " Run 'air --help' to get started."
echo "────────────────────────────────────────────────"
