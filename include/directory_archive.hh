#ifndef DIRECTORY_ARCHIVE_H_
#define DIRECTORY_ARCHIVE_H_ value

#include "archive.hh"

class DirectoryArchive {
  public:
    DirectoryArchive(const char*);

  private:
    const std::string _directory_path;
};

#endif /* ifndef DIRECTORY_ARCHIVE_H_ */
