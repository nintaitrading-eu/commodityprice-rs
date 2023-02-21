# commodityprice
# See LICENSE file for copyright and license details.

include config.mk

SRC = ${BUILDDIR}/commodityprice

all: commodityprice

commodityprice:
	@echo cargo build --release
	cargo build --release

clean:
	@echo Cargo clean...
	cargo clean

dist: clean
	@echo creating dist tarball
	@mkdir -p commodityprice-${VERSION}
	@cp -R LICENSE.txt Makefile README.adoc config.mk \
	commodityprice.1 ${SRC} commodityprice-${VERSION}
	@tar -cf commodityprice-${VERSION}.tar commodityprice-${VERSION}
	@gzip commodityprice-${VERSION}.tar
	@rm -rf commodityprice-${VERSION}

install:
	@echo installing executable file to ${DESTDIR}${PREFIX}/bin
	@mkdir -p ${DESTDIR}${PREFIX}/bin
	@cp -f ${BUILDDIR}/commodityprice ${DESTDIR}${PREFIX}/bin
	@chmod 755 ${DESTDIR}${PREFIX}/bin/commodityprice
	@mkdir -p ${DESTDIR}${SHARE}/commodityprice
	@echo installing manual page to ${DESTDIR}${MANPREFIX}/man1
	@mkdir -p ${DESTDIR}${MANPREFIX}/man1
	@sed "s/VERSION/${VERSION}/g" < commodityprice.1 > ${DESTDIR}${MANPREFIX}/man1/commodityprice.1
	@chmod 644 ${DESTDIR}${MANPREFIX}/man1/commodityprice.1

uninstall:
	@echo removing executable file from ${DESTDIR}${PREFIX}/bin
	@rm -f ${DESTDIR}${PREFIX}/bin/commodityprice
	@echo removing data in /usr/local/share from ${DESTDIR}${SHARE}/commodityprice
	@rm -rf ${DESTDIR}${SHARE}/commodityprice
	@echo removing manual page from ${DESTDIR}${MANPREFIX}/man1
	@rm -f ${DESTDIR}${MANPREFIX}/man1/commodityprice.1

.PHONY: all options clean dist install uninstall
