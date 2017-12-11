#ifndef DIRECTORY_ARCHIVE_H_
#define DIRECTORY_ARCHIVE_H_ value

#include <map>

#include "archive.hh"
#include "archive_facade.hh"

class DirectoryArchive : public ArchiveFacade {
  public:
    DirectoryArchive(const char *);

    std::vector<std::string> list_files_in_root();
    Node *get_node_for_path(const char *path);
    std::vector<Node *> get_nodes_in_directory(const char *directory);

  private:
    void *_directory_archive = nullptr;
};

#endif /* ifndef DIRECTORY_ARCHIVE_H_ */
