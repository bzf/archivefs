# archivefs
  `archivefs` - mount your archives and browse them as regular files

## Usage
  ```man
  archivefs [-sdf] --file=/some/absolute/path /mountpoint
  ```

## Description
  `archivefs` is a FUSE file system for mounting and reading from archive files
  instead of unpacking them to read the content.

## Building
  To build `archivefs` there are some packages that are required to be
  installed on the system:

  * [libfuse](https://github.com/libfuse/libfuse) (or [osxfuse](https://github.com/osxfuse/osxfuse))
  * [libarchive](https://github.com/libarchive/libarchive)
  * A C++11 compatiable compiler

  To build the project you run the `make` command:
  ```sh
  $ make
  ```

## Bugs
  `archivefs` does not handle nested directories inside an archive file
  properly. For now only flat archvies are compatiable.

## Good links
  * https://fossies.org/dox/fuse-2.9.7/fuse__compat_8h_source.html
