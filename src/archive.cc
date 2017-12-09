#include "archive.hh"

#include "libarchivefs.hh"
#include "utils.hh"

#include <algorithm>
#include <cstring>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

Archive::Archive(const std::string &path) {
    _archivefs_archive = archivefs_archive_new(path.c_str());
}

std::vector<std::string> Archive::list_files_in_root() {
    std::vector<std::string> vector;
    return vector;
}

Node *Archive::get_node_for_path(const char *path) {
    void *node_ptr =
        archivefs_archive_get_node_for_path(_archivefs_archive, path);

    if (node_ptr == nullptr) {
        return nullptr;
    } else {
        Node *ptr = new Node(node_ptr);
        return ptr;
    }
};

std::vector<Node *>
Archive::get_nodes_in_directory(const char *directory_prefix) {
    auto length = archivefs_archive_count_nodes_in_directory(_archivefs_archive,
                                                             directory_prefix);

    std::vector<Node *> vector;

    for (int i = 0; i < length; i++) {
        void *ptr = archivefs_archive_get_node_in_directory(
            _archivefs_archive, directory_prefix, i);
        Node *node = new Node(ptr);
        vector.push_back(node);
    }

    return vector;
}
