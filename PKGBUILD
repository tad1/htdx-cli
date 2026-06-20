# Maintainer: tad1 <aur@tad1.dev>

pkgname=htdx
pkgver=0.1.0
pkgrel=0
pkgdesc='A CLI tool for HTDX'
url=''
license=()
makedepends=('cargo')
depends=()
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
source=(
    "_htdx"
    "git+https://github.com/tad1/htdx.git"
)
sha256sums=('SKIP' 'SKIP')

prepare() {
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target host-tuple

}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/${pkgname}"
    install -Dm644 _htdx "$pkgdir/usr/share/zsh/site-functions/_htdx"

    install -d "$pkgdir/usr/share/htdx"
    cp -r htdx/content/* "$pkgdir/usr/share/htdx/"
    chmod -R u+rX,go+rX "$pkgdir/usr/share/htdx"
}
