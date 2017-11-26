#include <iostream>

#include <boost/filesystem.hpp>

#include <archive.h>
#include <archive_entry.h>

#include "utils.hh"
#include "directory_archive.hh"

bool
is_archive(const char *path) {
  auto archive = archive_read_new();

  archive_read_support_filter_all(archive);
  archive_read_support_format_all(archive);

  if (archive_open_and_read_from_path(path, archive, 10240) == ARCHIVE_OK) {
    archive_read_free(archive);
    return true;
  } else {
    return false;
  }
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
