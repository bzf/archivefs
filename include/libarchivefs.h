#ifndef LIBARCHIVEFS_H_
#define LIBARCHIVEFS_H_ value

#include <archive_entry.h>

int archivefs_handle_getattr_callback(void *directory_archive, const char *path,
                                      struct stat *stbuf);

int archivefs_handle_readdir_callback(void *directory_archive,
                                      const char *directory_prefix, void *buf,
                                      fuse_fill_dir_t filler, off_t,
                                      struct fuse_file_info *);

int archivefs_handle_read_callback(void *directory_archive, const char *path,
                                   char *buffer, size_t size, off_t offset,
                                   struct fuse_file_info *);

int archivefs_handle_release_callback(void *directory_archive, const char *path,
                                      struct fuse_file_info *);

void *archivefs_directory_archive_new(const char *path);

#endif /* ifndef LIBARCHIVEFS_H_ */
