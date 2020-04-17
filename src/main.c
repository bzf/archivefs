#define FUSE_USE_VERSION 31

#include <fuse.h>
#include <stdio.h>
#include <string.h>

#include "libarchivefs.h"

static void *g_directory_archive = NULL;
static void *g_filesystem = NULL;

static int getattr_callback(const char *path, struct stat *stbuf) {
    return archivefs_handle_getattr_callback(g_filesystem, path, stbuf);
}

int readdir_callback(const char *directory_prefix, void *buf,
                     fuse_fill_dir_t filler, off_t offset,
                     struct fuse_file_info *file_info) {
    return archivefs_handle_readdir_callback(
        g_filesystem, directory_prefix, buf, filler, offset, file_info);
}

int read_callback(const char *path, char *buf, size_t size, off_t offset,
                  struct fuse_file_info *file_info) {
    return archivefs_handle_read_callback(g_directory_archive, path, buf, size,
                                          offset, file_info);
}

int release_callback(const char *path, struct fuse_file_info *file_info) {
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

static struct fuse_operations operations = {
    .getattr = getattr_callback,
    .read = read_callback,
    .readdir = readdir_callback,
    .release = release_callback,
};

int main(int argc, char **argv) {
    struct fuse_args args = FUSE_ARGS_INIT(argc, argv);
    struct archivefs_conf configuration;
    fuse_opt_parse(&args, &configuration, archivefs_opts, NULL);

    if (configuration.directory_path == NULL) {
        fprintf(stderr, "Need to set which archive you want to mount\n");
        return 1;
    }

    g_directory_archive =
        archivefs_directory_archive_new(configuration.directory_path);

    g_filesystem =
        archivefs_filesystem_new(configuration.directory_path);

    return fuse_main(args.argc, args.argv, &operations, NULL);
}
