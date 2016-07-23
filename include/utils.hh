#ifndef UTILS_H
#define UTILS_H value

#include <algorithm>
#include <cstring>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>
#include <sys/stat.h>
#include <vector>

#include <archive.h>
#include <archive_entry.h>
#include <errno.h>

int archive_open_and_read_from_path(const std::string &path, archive *archive,
                                    size_t buffer_size);

char *convert(const std::string &s);
bool does_file_exist(const std::string &path);
const std::string filename_without_rar_extension(const std::string &path);
bool is_multipart_rar_file(const std::string &path);

#endif /* ifndef UTILS_H */
