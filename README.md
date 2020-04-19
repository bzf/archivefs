# archivefs
  `archivefs` - mount your archives and browse them as regular files

## Usage
  ```man
  archivefs [-sdf] --directory=/some/absolute/path /mountpoint
  ```

## Quick example
  ```sh
  $ tree .
  .
  ├── archives
  │   └── maskinen.tar.gz
  └── mountpoint

  2 directories, 1 file
  $ archivefs --directory=$HOME/example/archives mountpoint/
  $ tree .
  .
  ├── archives
  │   └── maskinen.tar.gz
  └── mountpoint
      └── maskinen
          ├── pengar.txt
          └── segertaget.txt

  3 directories, 3 files
  $ cat mountpoint/maskinen/segertaget.txt | head -n5
  [Vers 1]
  He-he-hej, här kommer segertåget
  Rullar in som teknodromen
  Tryck på play, du sätter på den
  Dansar som på, äh, vi kör igen
  ```

## Description
  `archivefs` is a FUSE file system for mounting and reading from archive files
  instead of unpacking them to read the content.

## Building
  To build `archivefs` there are some packages that are required to be
  installed on the system:

  * [libfuse](https://github.com/libfuse/libfuse) (or [osxfuse](https://github.com/osxfuse/osxfuse))
  * [libarchive](https://github.com/libarchive/libarchive)
  * [Rust](https://www.rustup.rs/)
  * A C compiler
  * Homebrew

  To build the project you run the `make` command:
  ```sh
  $ brew bundle # Installs dependencies for building the project
  $ make
  ```

## Bugs
  `archivefs` does not handle nested directories inside an archive file
  properly. For now only flat archvies are compatiable.

  If you get the following error message you can try setting the
  `LD_LIBRARY_PATH` environment variable to the path of the `libarchivefs`
  library:

  ```
  $ ./archivefs
  ./archivefs: error while loading shared libraries: libarchivefs.so: cannot open shared object file: No such file or directory
  $ LD_LIBRARY_PATH=./target/release/deps ./archivefs
  Need to set which archive you want to mount
  ```


## Good links
  * https://fossies.org/dox/fuse-2.9.7/fuse__compat_8h_source.html
