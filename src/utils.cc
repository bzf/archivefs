#include "utils.hh"

#include "libarchivefs.hh"

#include <algorithm>
#include <cstring>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

char *convert(const std::string &s) {
    char *pc = new char[s.size() + 1];
    std::strcpy(pc, s.c_str());
    pc[s.size()] = '\0';
    return pc;
}

bool does_file_exist(const std::string &path) {
    auto pointer = path.c_str();
    return archivefs_does_file_exist(pointer);
}

std::string filename_without_rar_extension(const std::string path) {
    const char *pointer = path.c_str();
    return archivefs_filename_without_extension(pointer, ".rar");
}

bool is_multipart_rar_file(const std::string path) {
    const char *pointer = path.c_str();
    return archivefs_is_multipart_rar_file(pointer);
}

int archive_open_and_read_from_path(const std::string &path, archive *archive,
                                    size_t buffer_size) {
    if (is_multipart_rar_file(path)) {
        printf("It's a multipart RAR file! Need ot find all parts\n");

        std::vector<std::string> parts;
        parts.push_back(path);

        for (int i = 0; true; i++) {
            std::string filename = filename_without_rar_extension(path);

            std::stringstream path;
            path << filename << ".r" << std::setfill('0') << std::setw(2) << i;

            const std::string filepath = path.str();
            if (does_file_exist(filepath)) {
                std::cout << "File '" << filepath << "' totally exists!"
                          << std::endl;
                parts.push_back(path.str());
            } else {
                break;
            }
        }

        std::vector<const char *> vc;
        std::transform(parts.begin(), parts.end(), std::back_inserter(vc),
                       convert);

        const char *foo[vc.size() + 1];
        for (size_t i = 0; i < vc.size(); i++) {
            foo[i] = vc[i];
        }
        foo[vc.size()] = nullptr;

        return archive_read_open_filenames(archive, foo, buffer_size);
    } else {
        // Load the singel fiel
        return archive_read_open_filename(archive, path.c_str(), buffer_size);
    }
}
