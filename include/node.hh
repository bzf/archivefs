#ifndef NODE_H_
#define NODE_H_ value

#include <string>

#include <archive.h>
#include <archive_entry.h>

class Node {
  public:
    Node(const std::string &archive_path, archive_entry *entry,
        const std::string &name, size_t buffer_size = 8096);

    bool isDirectory();

    int64_t size();
    const std::string name();

    void open();

    int write_to_buffer(char *buf, size_t size, off_t offset = -1);
    int close();

  private:
    const std::string _archive_path;
    archive_entry *_entry;

    archive *_archive = nullptr;
    char *_buffer = nullptr;
    const std::string _name;
    const size_t _buffer_size;
};

#endif /* ifndef NODE_H_ */
