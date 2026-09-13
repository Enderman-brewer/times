pkgname=times
pkgver=0.0.2
pkgrel=1
pkgdesc="A time program, but something's off"
arch=('x86_64' 'aarch64')
depends=('glibc')
makedepends=('cargo' 'rust')
source=()
sha256sums=()

build() {
    cargo build --release --locked --target-dir ./build
}

check() {
    cargo test --release --locked --target-dir ./build
}

package() {
    install -Dm755 build/release/times \
        "$pkgdir/usr/bin/times"
}