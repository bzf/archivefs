#ifndef LIBARCHIVEFS_H_
#define LIBARCHIVEFS_H_ value

extern "C" const char *archivefs_correct_path(const char *path);

extern "C" const char *
archivefs_filename_without_extension(const char *path, const char *extension);

#endif /* ifndef LIBARCHIVEFS_H_ */
