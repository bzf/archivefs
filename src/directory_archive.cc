#include <iostream>

#include <boost/filesystem.hpp>

#include "directory_archive.hh"

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
    }
}
