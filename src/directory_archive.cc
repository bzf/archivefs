#include <iostream>

#include <boost/filesystem.hpp>
#include <cstring>

#include "directory_archive.hh"
#include "libarchivefs.hh"

DirectoryArchive::DirectoryArchive(const char *directory_path) {
    _directory_archive = archivefs_directory_archive_new(directory_path);
}

std::vector<std::string> DirectoryArchive::list_files_in_root() {
    std::vector<std::string> vector;
    auto index =
        archivefs_directory_archive_count_nodes_in_root(_directory_archive);

    for (int i = 0; i < index; i++) {
        auto ptr =
            archivefs_directory_archive_get_node_in_root(_directory_archive, i);

        vector.push_back(ptr);
    }

    return vector;
}

Node *DirectoryArchive::get_node_for_path(const char *c_path) {
    // Ignore leading `/`
    auto node_ptr = archivefs_directory_archive_get_node_for_path(
        _directory_archive, c_path);

    Node *node = new Node(node_ptr);
    return node;
}

std::vector<Node *>
DirectoryArchive::get_nodes_in_directory(const char *directory_path) {
    std::vector<Node *> vector;
    auto index = archivefs_directory_archive_count_nodes_in_directory(
        _directory_archive, directory_path + 1);

    for (int i = 0; i < index; i++) {
        // Get the node, remove leading / in `directory_path`
        auto node_ptr = archivefs_directory_archive_get_node_in_directory(
            _directory_archive, directory_path + 1, i);
        Node *node = new Node(node_ptr);
        vector.push_back(node);
    }

    return vector;
}
