#include <iostream>

#include <fuse.h>
#include <string.h>

#include "archive.hh"
#include "archive_facade.hh"
#include "directory_archive.hh"
#include "libarchivefs.hh"

static ArchiveFacade *g_archive = nullptr;

static int getattr_callback(const char *path, struct stat *stbuf) {
    memset(stbuf, 0, sizeof(struct stat));

    auto node = g_archive->get_node_for_path(path);
    if (node != nullptr) {
        stbuf->st_mode = (node->isDirectory() ? S_IFDIR : S_IFREG) | 0777;
        stbuf->st_nlink = (int)node->isDirectory() + 1;

        if (!node->isDirectory()) {
            stbuf->st_size = node->size();
        }

        return 0;
    }

    if (strcmp(path, "/") == 0) {
        stbuf->st_mode = S_IFDIR | 0755;
        stbuf->st_nlink = 2;
        return 0;
    }

    return -ENOENT;
}

/* https://sourceforge.net/p/fuse/mailman/message/32809727/ */
int getxattr_callback(const char *, const char *, char *, size_t, uint32_t) {
    return ENODATA;
}

int readdir_callback(const char *directory_prefix, void *buf,
                     fuse_fill_dir_t filler, off_t, struct fuse_file_info *) {
    filler(buf, ".", NULL, 0);
    filler(buf, "..", NULL, 0);

    if (strcmp(directory_prefix, "/") == 0) {
        auto names = g_archive->list_files_in_root();

        auto name_iterator = names.begin();
        for (; name_iterator != names.end(); name_iterator++) {
            filler(buf, name_iterator->c_str(), NULL, 0);
        }
    } else {
        auto nodes = g_archive->get_nodes_in_directory(directory_prefix);

        auto node_iterator = nodes.begin();
        for (; node_iterator != nodes.end(); node_iterator++) {
            if ((*node_iterator)->isDirectory()) {
                continue;
            }

            filler(buf, (*node_iterator)->name().c_str(), NULL, 0);
        }
    }

    return 0;
}

// https://fossies.org/dox/fuse-2.9.7/structfuse__operations.html#a08a085fceedd8770e3290a80aa9645ac
int open_callback(const char *path, fuse_file_info *) {
    auto node = g_archive->get_node_for_path(path);
    if (node != nullptr) {
        node->open();
    }

    return 0;
}

int read_callback(const char *path, char *buf, size_t size, off_t offset,
                  fuse_file_info *) {
    auto node = g_archive->get_node_for_path(path);
    if (node) {
        return node->write_to_buffer(buf, size, offset);
    } else {
        return -ENOENT;
    }
}

int flush_callback(const char *, fuse_file_info *) { return 0; }

int release_callback(const char *path, fuse_file_info *) {
    auto node = g_archive->get_node_for_path(path);
    if (node) {
        return node->close();
    } else {
        return -ENOENT;
    }
}

int statfs_callback(const char *, struct statvfs *) { return 0; }

int opendir_callback(const char *, struct fuse_file_info *) { return 0; }

int releasedir_callback(const char *, struct fuse_file_info *) { return 0; }

int fgetattr_callback(const char *, struct stat *, struct fuse_file_info *) {
    return ENOENT;
}

typedef struct archivefs_conf {
    char *archive_path;
    char *directory_path;
} archivefs_conf;

static struct fuse_opt archivefs_opts[] = {
    {"--file=%s", offsetof(archivefs_conf, archive_path), 0},
    {"--directory=%s", offsetof(archivefs_conf, directory_path), 0},
    FUSE_OPT_END,
};

struct fuse_operations build_operations() {
    struct fuse_operations operations;
    operations.getattr = getattr_callback;
    operations.open = open_callback;
    operations.read = read_callback;
    operations.readdir = readdir_callback;
    operations.opendir = opendir_callback;
    operations.flush = flush_callback;
    operations.release = release_callback;
    operations.releasedir = releasedir_callback;
    // operations.getxattr = getxattr_callback;
    operations.statfs = statfs_callback;
    // operations.fgetattr = fgetattr_callback;

    return operations;
};

int main(int argc, char **argv) {
    struct fuse_args args = FUSE_ARGS_INIT(argc, argv);
    struct fuse_operations operations = build_operations();

    archivefs_conf configuration;
    memset(&configuration, 0, sizeof(configuration));
    fuse_opt_parse(&args, &configuration, archivefs_opts, NULL);

    if (configuration.archive_path == nullptr &&
        configuration.directory_path == nullptr) {
        std::cerr << "Need to set which archive you want to mount" << std::endl;
        return 1;
    }

    if (configuration.archive_path != nullptr) {
        g_archive = new Archive(configuration.archive_path);
    } else if (configuration.directory_path != nullptr) {
        g_archive = new DirectoryArchive(configuration.directory_path);
    }

    return fuse_main(args.argc, args.argv, &operations, NULL);
}
