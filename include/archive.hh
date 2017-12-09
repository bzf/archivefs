#ifndef ARCHIVE_H_
#define ARCHIVE_H_ value

#include <map>
#include <vector>

#include <archive.h>
#include <archive_entry.h>
#include <errno.h>

#include "archive_facade.hh"
#include "node.hh"

class Archive : public ArchiveFacade {
  public:
    Archive(const std::string &path);

    std::vector<std::string> list_files_in_root();
    Node *get_node_for_path(const char *path);
    std::vector<Node *> get_nodes_in_directory(const char *directory);

    std::map<std::string, Node> _dict;
    archive *_archive = NULL;

  private:
    void *_archivefs_archive = nullptr;
};

#endif /* ifndef ARCHIVE_H_ */
