PREFIX ?= /usr
BINARY := target/release/rlgl

all: src/
	cargo build --release
install: all
	strip $(BINARY)
	cp $(BINARY) $(PREFIX)/bin/
	cp rlgl.1 $(PREFIX)/share/man/man1/
