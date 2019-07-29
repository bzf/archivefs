.PHONY: clean libarchivefs

UNAME := $(shell uname)

CFLAGS = -Wall -Wextra -Werror -fdiagnostics-color=auto \
				 -Iinclude -I/usr/local/include/osxfuse -I/usr/local/opt/libarchive/include
LDFLAGS = -larchive

ifeq ($(UNAME),Darwin)
	LDFLAGS += -losxfuse
	CFLAGS += -D_DARWIN_USE_64_BIT_INODE
endif
ifeq ($(UNAME),Linux)
	LDFLAGS += -lfuse
endif

all: archivefs

archivefs: src/main.c libarchivefs | create_build_directory
	gcc -D_FILE_OFFSET_BITS=64 -L/usr/local/opt/libarchive/lib -L/usr/local/lib \
		src/main.c -o archivefs \
		$(LDFLAGS) $(CFLAGS) \
		-L./target/release/ -larchivefs \
		-Wl,-R/usr/local/opt/archivefs/lib

create_build_directory:
	mkdir -p build/

libarchivefs: src/lib.rs Cargo.toml
	cargo build --release

clean:
	rm -rf build/ archivefs
	cargo clean
