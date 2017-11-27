#ifndef ARCHIVE_FACADE_H_
#define ARCHIVE_FACADE_H_ value

#include <vector>

#include "node.hh"

class ArchiveFacade {
  public:
    virtual std::vector<std::string> list_files_in_root() = 0;
    virtual Node *get_node_for_path(const char *path) = 0;
    virtual std::vector<Node *>
    get_nodes_in_directory(const char *directory) = 0;
};

#endif /* ifndef ARCHIVE_FACADE_H_ */
