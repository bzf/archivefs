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
                     fuse_fill_dir_t filler, off_t offset,
                     struct fuse_file_info *file_info) {
    return archivefs_handle_readdir_callback(
        g_directory_archive, directory_prefix, buf, filler, offset, file_info);
}

int open_callback(const char *path, fuse_file_info *file_info) {
    return archivefs_handle_open_callback(g_directory_archive, path, file_info);
}

int read_callback(const char *path, char *buf, size_t size, off_t offset,
                  fuse_file_info *file_info) {
    return archivefs_handle_read_callback(g_directory_archive, path, buf, size,
                                          offset, file_info);
}

int release_callback(const char *path, fuse_file_info *file_info) {
    return archivefs_handle_release_callback(g_directory_archive, path,
                                             file_info);
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
