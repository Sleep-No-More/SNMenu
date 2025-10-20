# Maintainer: Sleep-No-More <sleepnomore@example.com>
pkgname=cpmenu
pkgver=0.2.1
pkgrel=1
pkgdesc="Modern circular power menu for Wayland desktop environments"
arch=('x86_64')
url="https://github.com/Sleep-No-More/cpmenu"
license=('MIT')
depends=('gtk3' 'cairo')
source=("https://github.com/Sleep-No-More/cpmenu/releases/download/v${pkgver}/cpmenu")
sha256sums=('30d60bf853b7d57fa003ad971ee49f3891b6256d285d84920113031d2e7189b2')

package() {
    install -Dm755 "cpmenu" "$pkgdir/usr/bin/cpmenu"
}
