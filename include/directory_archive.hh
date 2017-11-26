#ifndef DIRECTORY_ARCHIVE_H_
#define DIRECTORY_ARCHIVE_H_ value

#include <map>

#include "archive.hh"

class DirectoryArchive {
  public:
    DirectoryArchive(const char*);

  private:
    const std::string _directory_path;
    std::map<std::string, Archive> _dict;
};

#endif /* ifndef DIRECTORY_ARCHIVE_H_ */
