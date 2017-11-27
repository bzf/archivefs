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
        Node node = Node(path, ptr, pathname);

        _dict.insert({"/" + correct_path(pathname), node});

        archive_read_data_skip(_archive);
    }

    std::cerr << "[archive] freeing!" << std::endl;
    archive_read_free(_archive);
}

std::vector<std::string> Archive::list_files_in_root() {
    std::vector<std::string> vector;

    auto it = _dict.begin();
    for (; it != _dict.end(); it++) {
        std::string name = it->first;
        name.replace(0, 1, "");
        vector.push_back(name);
    }

    return vector;
}

Node *Archive::get_node_for_path(const char *path) {
    auto it = _dict.begin();
    for (; it != _dict.end(); it++) {
        if (path == it->first) {
            return &(it->second);
        }
    }

    return nullptr;
};

std::vector<Node *>
Archive::get_nodes_in_directory(const char *directory_prefix) {
    std::vector<Node *> vector;

    auto it = _dict.begin();
    for (; it != _dict.end(); it++) {
        // If the path of the it->first is longer than the directory_prefix, it
        // can't be that node we're looking for
        if (it->first.length() <= strlen(directory_prefix)) {
            continue;
        }

        std::string compare_path =
            it->first.substr(0, strlen(directory_prefix) - 1);
        if (compare_path.c_str(), directory_prefix) {
            // If it contains a `/`, it's a subdir so a no go
            std::string path_without_directory_prefix = it->first.substr(
                strlen(directory_prefix), it->first.length() - 1);

            // Remove any leading slashes
            if (path_without_directory_prefix[0] == '/') {
                path_without_directory_prefix =
                    path_without_directory_prefix.substr(
                        1, path_without_directory_prefix.length() - 1);
            }

            // If the folder doesn't end on "/", show it in the directory
            if (path_without_directory_prefix.find("/") == std::string::npos) {
                vector.push_back(&(it->second));
            }
        }
    }

    return vector;
}

/* We can't have paths ending in `/` */
std::string Archive::correct_path(const std::string &path) {
    char last_character = path[path.size() - 1];
    if (last_character == '/') {
        return path.substr(0, path.size() - 1);
    } else {
        return path;
    }
}
