#include <iostream>

#include <boost/filesystem.hpp>
#include <cstring>

#include <archive.h>
#include <archive_entry.h>

#include "directory_archive.hh"
#include "utils.hh"

bool is_archive(const char *path) {
    auto archive = archive_read_new();

    archive_read_support_filter_all(archive);
    archive_read_support_format_all(archive);

    std::string foo(path);
    std::string::size_type rar_position = foo.find(".rar");
    printf("rar at pos: %li\n", rar_position);

    return rar_position != std::string::npos;
}

DirectoryArchive::DirectoryArchive(const char *directory_path)
    : _directory_path(directory_path) {

    auto file_iterator =
        boost::filesystem::recursive_directory_iterator(_directory_path);

    for (auto it : file_iterator) {
        auto path = it.path();
        const std::string filename_without_format =
            path.stem().generic_string();
        const std::string filename = path.filename().generic_string();
        const std::string full_path = path.generic_string();

        if (is_archive(full_path.c_str())) {
            auto my_archive = Archive(full_path);
            _dict.insert({filename_without_format, my_archive});
        }
    }
}

std::vector<std::string> DirectoryArchive::list_files_in_root() {
    std::vector<std::string> vector;

    auto it = _dict.begin();
    for (; it != _dict.end(); it++) {
        vector.push_back(it->first);
    }

    return vector;
}

Node *DirectoryArchive::get_node_for_path(const char *c_path) {
    std::string path(c_path);

    // If it's in the _dict as a key, return a node saying it's a directory
    auto it = _dict.begin();
    for (; it != _dict.end(); it++) {
        if (path.compare("/" + it->first) == 0) {
            return new Node("", nullptr, path);
        }

        // If it starts with something in the names, substring everything that
        // matches and pass it to the `Archive`
        if (path.compare("/" + it->first) == 1) {
            std::string subpath = path;
            subpath.replace(0, it->first.length() + 1, "");
            return it->second.get_node_for_path(subpath.c_str());
        }
    }

    return nullptr;
}

std::vector<Node *>
DirectoryArchive::get_nodes_in_directory(const char *directory_path) {
    std::vector<Node *> vector;
    auto it = _dict.begin();
    for (; it != _dict.end(); it++) {
        auto compare_index = strcmp(it->first.c_str(), directory_path + 1);
        if (compare_index == 0) {
            return it->second.get_nodes_in_directory("/");
        }
    }

    return vector;
}
