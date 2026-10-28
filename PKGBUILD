# Maintainer: OmniMem Contributors
pkgname=omnimem-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="Git for AI Agent Knowledge - local-first agent memory and code intelligence"
arch=('x86_64' 'aarch64')
url="https://github.com/foysalkhan288-rgb/omnimem"
license=('Apache-2.0')
provides=('omnimem')
conflicts=('omnimem')
source_x86_64=("$pkgname-$pkgver-x86_64.tar.gz::https://github.com/foysalkhan288-rgb/omnimem/releases/download/v$pkgver/omnimem-x86_64-unknown-linux-gnu.tar.gz")
source_aarch64=("$pkgname-$pkgver-aarch64.tar.gz::https://github.com/foysalkhan288-rgb/omnimem/releases/download/v$pkgver/omnimem-aarch64-unknown-linux-gnu.tar.gz")
sha256sums_x86_64=('REPLACE_WITH_SHA256')
sha256sums_aarch64=('REPLACE_WITH_SHA256')

package() {
    install -Dm755 omnimem "$pkgdir/usr/bin/omnimem"
}