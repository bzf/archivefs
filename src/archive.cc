#include "archive.hh"

#include "utils.hh"

#include <algorithm>
#include <cstring>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

Archive::Archive(const std::string &path)
    : _archive(archive_read_new()), _archive_path(path) {
    // Read the full archive and parse it into folders and files
    archive_read_support_filter_all(_archive);
    archive_read_support_format_all(_archive);

    // TODO: Are there more files that needs to be read?
    printf("Some filename: %s\n", path.c_str());

    archive_open_and_read_from_path(path, _archive, 10240);
    std::cerr << "[archive] archive_open_and_read_from_path" << std::endl;

    struct archive_entry *entry;
    while (archive_read_next_header(_archive, &entry) == ARCHIVE_OK) {
        const std::string pathname = archive_entry_pathname(entry);
        auto ptr = archive_entry_clone(entry);
        Node node = Node(path, ptr);

        _dict.insert({"/" + correct_path(pathname), node});

        archive_read_data_skip(_archive);
    }

    std::cerr << "[archive] freeing!" << std::endl;
    archive_read_free(_archive);
}

Node* Archive::get_node_for_path(const char* path) {
  auto it = _dict.begin();
  for (; it != _dict.end(); it++) {
    if (path == it->first) {
      return &(it->second);
    }
  }

  return nullptr;
};

/* We can't have paths ending in `/` */
std::string Archive::correct_path(const std::string &path) {
    char last_character = path[path.size() - 1];
    if (last_character == '/') {
        return path.substr(0, path.size() - 1);
    } else {
        return path;
    }
}
