#include <iostream>

#include <boost/filesystem.hpp>
#include <cstring>

#include <archive.h>
#include <archive_entry.h>

#include "utils.hh"
#include "directory_archive.hh"

bool
is_archive(const char *path) {
  auto archive = archive_read_new();

  archive_read_support_filter_all(archive);
  archive_read_support_format_all(archive);

  /* if (archive_open_and_read_from_path(path, archive, 10240) == ARCHIVE_OK) { */
  std::string foo(path);
  std::string::size_type rar_position = foo.find(".rar");
  std::cout << "is_multipart_rar_file: " << foo << std::endl;
  printf("rar at pos: %li\n", rar_position);

  return rar_position != std::string::npos;
}

DirectoryArchive::DirectoryArchive(const char *directory_path)
  : _directory_path(directory_path) {

    std::cout << "DirectoryArchive(): " << directory_path << std::endl;
    auto file_iterator =
      boost::filesystem::recursive_directory_iterator(_directory_path);

    for (auto it : file_iterator) {
      auto path = it.path();
      const std::string filename_without_format = path.stem().generic_string();
      const std::string filename = path.filename().generic_string();
      const std::string full_path = path.generic_string();

      std::cout << "Filename: " << filename << std::endl;
      std::cout << "Filename with format: " << filename_without_format << std::endl;
      std::cout << "Full path: " << full_path << "\n" << std::endl;

      if (is_archive(full_path.c_str())) {
        auto my_archive = Archive(full_path);
        _dict.insert({ filename_without_format, my_archive });
      }
    }

    std::cout << "DirectoryArchive::_dict.size(): " << _dict.size() << std::endl;
}

std::vector<std::string>
DirectoryArchive::list_files_in_root() {
  std::vector<std::string> vector;

  auto it = _dict.begin();
  for (; it != _dict.end(); it++) {
    vector.push_back(it->first);
  }

  return vector;
}

Node*
DirectoryArchive::get_node_for_path(const char *path) {
  // If it's in the _dict as a key, return a node saying it's a directory
  auto it = _dict.begin();
  for (; it != _dict.end(); it++) {
    auto compare_index = strcmp(path, ("/" + it->first).c_str());
    /* std::cout << "DirectoryArchive::get_node_for_path compare_index: " << compare_index << std::endl; */

    if (compare_index == 0) {
      /* std::cout << "Exact match, return a directory node" << std::endl; */
      return new Node("", nullptr, it->first);
    }

    // If it starts with something in the names, substring everything that
    // matches and pass it to the `Archive`

    if (compare_index > 0) {
      /* std::cout << "get_node_for_path: found something that amtches path" << std::endl; */
      /* std::cout << (path + compare_index - 5) << "\n" << std::endl; */
      return it->second.get_node_for_path(path + compare_index - 5);
      /* return new Node("", nullptr, it->first); */
    }
  }

  return nullptr;
}

std::vector<Node*>
DirectoryArchive::get_nodes_in_directory(const char *directory_path) {
  std::cout << "DirectoryArchive::get_nodes_in_directory: " << directory_path << std::endl;

  std::vector<Node*> vector;
  auto it = _dict.begin();
  for (; it != _dict.end(); it++) {
    auto compare_index = strcmp(it->first.c_str(), directory_path + 1);
    std::cout << "compare_index: " << compare_index << std::endl;
    if (compare_index == 0) {
      /* it->second.get_nodes_in_directory( */
      std::cout << "Exact match, return root in archive (it->second)" <<std::endl;
      return it->second.get_nodes_in_directory("/");
      /* std::cout << (directory_path + compare_index) <<std::endl; */
    }
  }

  return vector;
}
