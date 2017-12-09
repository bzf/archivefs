#ifndef LIBARCHIVEFS_H_
#define LIBARCHIVEFS_H_ value

#include <archive_entry.h>

extern "C" const char *archivefs_correct_path(const char *path);

extern "C" const char *
archivefs_filename_without_extension(const char *path, const char *extension);

extern "C" bool archivefs_is_multipart_rar_file(const char *path);

extern "C" void *archivefs_new_node(const char *path,
                                    const archive_entry *entry,
                                    const char *name, size_t buffer_size);

extern "C" bool archivefs_node_is_directory(void *archivefs_node);

extern "C" const char *archivefs_node_name(void *archivefs_node);

extern "C" int64_t archivefs_node_size(void *archivefs_node);

extern "C" bool archivefs_does_file_exist(const char *path);

extern "C" void archivefs_node_open(void *archivefs_node);

extern "C" int archivefs_node_close(void *archivefs_node);

extern "C" int archivefs_node_write_to_buffer(void *archivefs_node, char *buf,
                                              size_t size, off_t offset);

extern "C" void *archivefs_archive_new(const char *path);

extern "C" void *archivefs_archive_get_node_for_path(void *archive,
                                                     const char *path);

extern "C" void *archivefs_archive_get_node_in_directory(void *archive,
                                                         const char *prefix,
                                                         int index);

extern "C" int archivefs_archive_count_nodes_in_directory(void *archive,
                                                          const char *prefix);

#endif /* ifndef LIBARCHIVEFS_H_ */
