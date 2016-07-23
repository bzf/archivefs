#ifndef ARCHIVE_H_
#define ARCHIVE_H_ value

#include <map>

#include <archive.h>
#include <archive_entry.h>
#include <errno.h>

#include "node.hh"

class Archive {
  public:
    Archive(const std::string &path);

    std::map<std::string, Node> _dict;
    archive *_archive = NULL;

  private:
    const std::string _archive_path;

    std::string correct_path(const std::string &path);
};

#endif /* ifndef ARCHIVE_H_ */
