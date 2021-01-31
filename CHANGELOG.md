## Unreleased

## v0.1.3 - 2021-01-31

* Fix memory leak in `ffi::archive_open_and_read_from_path`
* Remove unnecessary `archive_entry_clone()` call that leaked memory
* Ignore directories containing an `.archivefs-ignore` file

## v0.1.2 - 2020-04-19

* Handle `.tar.gz` extensions showing as directory ending with `.tar`

## v0.1.1 - 2020-04-19

* Added man page
