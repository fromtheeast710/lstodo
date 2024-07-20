pkgname=lstodo
pkgver=0.1.0
pkgrel=0
pkgdesc="Small and simple CLI Todo manager"
url="https://github.com/fromtheeast710/lstodo"
license=('GPLv3')
# makedepends=('cargo')
arch=('x86_64')
source=("$url/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('cdd53a436306ee55ea4af1bd9cfd73b53fa2333afcd05564346cd4717c2eb9d8')

build() {
  export RUSTUP_TOOLCHAIN=stable
	export CARGO_TARGET_DIR=target

	cd "$pkgname-$pkgver"

	cargo build --frozen --release --all-features
}

package() {
	cd "$pkgname-$pkgver"

	install -D "target/release/lstodo" "$pkgdir/usr/bin/lstodo"
}