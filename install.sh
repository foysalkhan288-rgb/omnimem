#!/usr/bin/env bash
# OmniMem installer (Unix)
set -euo pipefail

REPO="foysalkhan288-rgb/omnimem"
VERSION="${OMNIMEM_VERSION:-latest}"
INSTALL_DIR="${OMNIMEM_INSTALL_DIR:-$HOME/.local/bin}"

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "$ARCH" in
    x86_64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *) echo "Unsupported arch: $ARCH"; exit 1 ;;
esac

case "$OS" in
    linux) TARGET="${ARCH}-unknown-linux-gnu" ;;
    darwin) TARGET="${ARCH}-apple-darwin" ;;
    *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

mkdir -p "$INSTALL_DIR"

if [ "$VERSION" = "latest" ]; then
    URL="https://github.com/${REPO}/releases/latest/download/omnimem-${TARGET}.tar.gz"
else
    URL="https://github.com/${REPO}/releases/download/v${VERSION}/omnimem-${TARGET}.tar.gz"
fi

echo "Downloading OmniMem ${VERSION} for ${TARGET}..."
tmpdir="$(mktemp -d)"
trap 'rm -rf "$tmpdir"' EXIT

curl -fsSL "$URL" -o "$tmpdir/omnimem.tar.gz"
tar -xzf "$tmpdir/omnimem.tar.gz" -C "$tmpdir"
install -m 755 "$tmpdir/omnimem" "$INSTALL_DIR/omnimem"

echo "Installed to $INSTALL_DIR/omnimem"
echo "Add $INSTALL_DIR to your PATH if not already present."
"$INSTALL_DIR/omnimem" --version || true