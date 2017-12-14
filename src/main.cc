#include <iostream>

#define FUSE_USE_VERSION 31

#include <fuse.h>
#include <string.h>

#include "libarchivefs.hh"

static void *g_directory_archive = nullptr;

static int getattr_callback(const char *path, struct stat *stbuf) {
    return archivefs_handle_getattr_callback(g_directory_archive, path, stbuf);
}

int readdir_callback(const char *directory_prefix, void *buf,
                     fuse_fill_dir_t filler, off_t, struct fuse_file_info *) {
    filler(buf, ".", NULL, 0);
    filler(buf, "..", NULL, 0);

    if (strcmp(directory_prefix, "/") == 0) {
        int nodes_in_root = archivefs_directory_archive_count_nodes_in_root(
            g_directory_archive);

        for (int i = 0; i < nodes_in_root; i++) {
            void *node = archivefs_directory_archive_get_node_in_root(
                g_directory_archive, i);
            const char *node_name = archivefs_node_name(node);
            filler(buf, node_name, NULL, 0);
        }
    } else {
        int nodes_in_directory =
            archivefs_directory_archive_count_nodes_in_directory(
                g_directory_archive, directory_prefix);

        for (int i = 0; i < nodes_in_directory; i++) {
            void *node = archivefs_directory_archive_get_node_in_directory(
                g_directory_archive, directory_prefix, i);

            if (archivefs_node_is_directory(node)) {
                continue;
            }

            const char *node_name = archivefs_node_name(node);
            filler(buf, node_name, NULL, 0);
        }
    }

    return 0;
}

// https://fossies.org/dox/fuse-2.9.7/structfuse__operations.html#a08a085fceedd8770e3290a80aa9645ac
int open_callback(const char *path, fuse_file_info *) {
    void *node = archivefs_directory_archive_get_node_for_path(
        g_directory_archive, path);
    if (node != nullptr) {
        archivefs_node_open(node);
    }

    return 0;
}

int read_callback(const char *path, char *buf, size_t size, off_t offset,
                  fuse_file_info *) {
    void *node = archivefs_directory_archive_get_node_for_path(
        g_directory_archive, path);
    if (node) {
        return archivefs_node_write_to_buffer(node, buf, size, offset);
    } else {
        return -ENOENT;
    }
}

int release_callback(const char *path, fuse_file_info *) {
    void *node = archivefs_directory_archive_get_node_for_path(
        g_directory_archive, path);
    if (node) {
        return archivefs_node_close(node);
    } else {
        return -ENOENT;
    }
}

typedef struct archivefs_conf {
    char *directory_path;
} archivefs_conf;

static struct fuse_opt archivefs_opts[] = {
    {"--directory=%s", offsetof(archivefs_conf, directory_path), 0},
    FUSE_OPT_END,
};

int main(int argc, char **argv) {
    struct fuse_args args = FUSE_ARGS_INIT(argc, argv);
    struct fuse_operations operations {};
    operations.getattr = &getattr_callback;
    operations.open = open_callback;
    operations.read = read_callback;
    operations.readdir = readdir_callback;
    operations.release = release_callback;

    struct archivefs_conf configuration;
    fuse_opt_parse(&args, &configuration, archivefs_opts, NULL);

    if (configuration.directory_path == nullptr) {
        std::cerr << "Need to set which archive you want to mount" << std::endl;
        return 1;
    }

    g_directory_archive =
        archivefs_directory_archive_new(configuration.directory_path);

    return fuse_main(args.argc, args.argv, &operations, NULL);
}
