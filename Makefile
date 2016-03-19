PREFIX = /usr/local

.PHONY: build
build:
	cargo build --release

.PHONY: debug
debug:
	cargo build

.PHONY: try
try: debug
	# Shortcut to quickly test things when developing
	RUST_LOG=info sudo -E target/debug/malc

.PHONY: install
install:
	install -Dm 4755 target/release/malc $(DESTDIR)$(PREFIX)/bin/malc

.PHONY: uninstall
uninstall:
	rm -f $(DESTDIR)$(PREFIX)/bin/malc
