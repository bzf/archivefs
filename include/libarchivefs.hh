#ifndef LIBARCHIVEFS_H_
#define LIBARCHIVEFS_H_ value

#include <archive_entry.h>

extern "C" bool archivefs_node_is_directory(void *archivefs_node);

extern "C" const char *archivefs_node_name(void *archivefs_node);

extern "C" int64_t archivefs_node_size(void *archivefs_node);

extern "C" void archivefs_node_open(void *archivefs_node);

extern "C" int archivefs_node_close(void *archivefs_node);

extern "C" int archivefs_node_write_to_buffer(void *archivefs_node, char *buf,
                                              size_t size, off_t offset);

extern "C" int
archivefs_directory_archive_count_nodes_in_root(void *directory_archive);

extern "C" void *
archivefs_directory_archive_get_node_in_root(void *directory_archive,
                                             int index);

extern "C" void *archivefs_directory_archive_get_node_in_directory(
    void *directory_archive, const char *prefix, int index);

extern "C" int
archivefs_directory_archive_count_nodes_in_directory(void *directory_archive,
                                                     const char *prefix);

extern "C" void *archivefs_directory_archive_new(const char *path);

extern "C" void *
archivefs_directory_archive_get_node_for_path(void *archive, const char *path);

#endif /* ifndef LIBARCHIVEFS_H_ */
