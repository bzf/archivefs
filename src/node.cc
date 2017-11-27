#include "node.hh"

#include "utils.hh"

#include <archive.h>
#include <archive_entry.h>
#include <stdexcept>

Node::Node(const std::string archive_path, archive_entry *entry,
           const std::string name, size_t buffer_size)
    : _archive_path(archive_path), _entry(entry), _name(name),
      _buffer_size(buffer_size) {}

bool Node::isDirectory() {
    if (_entry == nullptr) {
        return true;
    } else {
        return (archive_entry_filetype(_entry) == 16384);
    }
}

int64_t Node::size() { return archive_entry_size(_entry); }

const std::string Node::name() { return _name; }

void Node::open() {
    if (_archive != nullptr) {
        return;
    }

    _archive = archive_read_new();
    _buffer = new char[_buffer_size];
    archive_read_support_filter_all(_archive);
    archive_read_support_format_all(_archive);

    archive_open_and_read_from_path(_archive_path, _archive, _buffer_size);

    struct archive_entry *entry;
    const std::string our_entry_path = archive_entry_pathname(_entry);
    while (archive_read_next_header(_archive, &entry) == ARCHIVE_OK) {
        std::string their_entry_path = archive_entry_pathname(entry);
        if (their_entry_path == our_entry_path) {
            return;
        }
    }

    throw std::runtime_error("Could not find the path in the archive");
}

int Node::write_to_buffer(char *buf, size_t size, off_t offset) {
    if (_buffer == nullptr) {
        throw std::runtime_error("Must call open() before write_to_buffer()");
    }

    if (offset != -1) {
        archive_seek_data(_archive, offset, 0);
    }

    return archive_read_data(_archive, buf, size);
}

int Node::close() {
    archive_read_free(_archive);
    _archive = nullptr;

    return 0;
}
