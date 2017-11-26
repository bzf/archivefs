#ifndef DIRECTORY_ARCHIVE_H_
#define DIRECTORY_ARCHIVE_H_ value

#include <map>

#include "archive.hh"
#include "archive_facade.hh"

class DirectoryArchive : public ArchiveFacade {
  public:
    DirectoryArchive(const char*);

    Node* get_node_for_path(const char *path);
    std::vector<Node*> get_nodes_in_directory(const char *directory);

  private:
    const std::string _directory_path;
    std::map<std::string, Archive> _dict;
};

#endif /* ifndef DIRECTORY_ARCHIVE_H_ */
