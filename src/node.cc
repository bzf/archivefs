#include "node.hh"

#include "libarchivefs.hh"
#include "utils.hh"

#include <archive.h>
#include <archive_entry.h>
#include <stdexcept>

Node::Node(const std::string archive_path, archive_entry *entry,
           const std::string name, size_t buffer_size) {
    _node = archivefs_new_node(archive_path.c_str(), entry, name.c_str(),
                               buffer_size);
}

Node::Node(void *node) : _node(node) {}

bool Node::isDirectory() { return archivefs_node_is_directory(_node); }

int64_t Node::size() { return archivefs_node_size(_node); }

const std::string Node::name() {
    std::string name(archivefs_node_name(_node));
    return name;
}

void Node::open() { return archivefs_node_open(_node); }

int Node::write_to_buffer(char *buf, size_t size, off_t offset) {
    return archivefs_node_write_to_buffer(_node, buf, size, offset);
}

int Node::close() { return archivefs_node_close(_node); }
