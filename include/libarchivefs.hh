#ifndef LIBARCHIVEFS_H_
#define LIBARCHIVEFS_H_ value

#include <archive_entry.h>

extern "C" int archivefs_handle_getattr_callback(void *directory_archive,
                                                 const char *path,
                                                 struct stat *stbuf);

extern "C" int archivefs_handle_readdir_callback(void *directory_archive,
                                                 const char *directory_prefix,
                                                 void *buf,
                                                 fuse_fill_dir_t filler, off_t,
                                                 struct fuse_file_info *);

extern "C" void archivefs_node_open(void *archivefs_node);

extern "C" int archivefs_node_close(void *archivefs_node);

extern "C" int archivefs_node_write_to_buffer(void *archivefs_node, char *buf,
                                              size_t size, off_t offset);

extern "C" void *archivefs_directory_archive_new(const char *path);

extern "C" void *
archivefs_directory_archive_get_node_for_path(void *archive, const char *path);

#endif /* ifndef LIBARCHIVEFS_H_ */
