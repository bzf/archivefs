.PHONY: clean

CFLAGS_OSXFUSE += -DFUSE_USE_VERSION=26
CFLAGS_OSXFUSE += -D_FILE_OFFSET_BITS=64
CFLAGS_OSXFUSE += -D_DARWIN_USE_64_BIT_INODE

CFLAGS = -Wall -Wextra -Werror -std=c++11 -fdiagnostics-color=auto -Iinclude -I/usr/local/include -I/usr/local/opt/libarchive/include 
LDFLAGS = -losxfuse -larchive -g

all: archivefs

archivefs: src/main.cc node arc utils | create_build_directory
	g++ src/main.cc build/*.o -o archivefs $(LDFLAGS) $(CFLAGS) -D_FILE_OFFSET_BITS=64 -L/usr/local/opt/libarchive/lib -L/usr/local/lib $(CFLAGS_OSXFUSE) 

node: src/node.cc include/node.hh | create_build_directory
	g++ src/node.cc -c -o build/node.o $(CFLAGS) -D_FILE_OFFSET_BITS=64

arc: src/archive.cc include/archive.hh | create_build_directory
	g++ src/archive.cc -c -o build/archive.o $(CFLAGS) -D_FILE_OFFSET_BITS=64

utils: src/utils.cc include/utils.hh | create_build_directory
	g++ src/utils.cc -c -o build/utils.o $(CFLAGS) -D_FILE_OFFSET_BITS=64

create_build_directory:
	mkdir -p build/

clean:
	rm -rf build/ archivefs
