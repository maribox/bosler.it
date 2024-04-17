#define _POSIX_C_SOURCE 200809L

#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <time.h>
#include <unistd.h>
#include <xmmintrin.h>

static char* read_file(const char* path) {
    // Read the contents of the file specified by path into a heap-allocated
    // buffer and return a pointer to that buffer.
    char *string = NULL;
    FILE *file;

    if (!(file = fopen(path, "r")))
    {
        perror("Error opening file");
        return NULL;
    }

    struct stat statbuf;
    if (fstat(fileno(file), &statbuf))
    {
        perror("Error retrieving file");

        fclose(file);

        return NULL;
    }
    (void) path;
    return NULL;
}

static void write_file(const char* path, const char* string) {
    // Write the string to the file specified by path.
    // TODO!
    (void) path;
    (void) string;
}

static char* gen_randstr(size_t len) {
    char* string = malloc(len + 4);
    if (!string) {
        perror("gen_randstr"); // Nice error message.
        exit(EXIT_FAILURE);
    }

    static const char prefix[] = "abcxyzABCXYZ01`{";
    strncpy(string, prefix, len);

    size_t prefix_len = sizeof prefix - 1;
    if (len > prefix_len) {
        // Generate a random string
        int fd = open("/dev/urandom", O_RDONLY);
        (void) !read(fd, string + prefix_len, len - prefix_len);
        close(fd);

        unsigned* words = (unsigned*) (string + prefix_len);
        for (size_t i = 0; i < (len - prefix_len) / 4 + 1; i++)
            words[i] = (words[i] | 0x40404040) & ~0x90909090; // valid ASCII
    }

    string[len] = 0; // Terminate
    return string;
}

static double curtime(void) {
    struct timespec t;
    clock_gettime(CLOCK_MONOTONIC, &t);
    return t.tv_sec + t.tv_nsec * 1e-9;
}

static void run(const char* name, void (*func)(char*), char* arg) {
    double start = curtime();
    func(arg);
    double end = curtime();
    printf("%-4s took %f seconds\n", name, end - start);
}

// Implemented in toupper.S
void toupper_asm(char* str);
void toupper_asm_simd(char* str);


void toupper_c(char* text) {
    for (; *text != '\0'; text++)
        if (*text >= 'a' && *text <= 'z')
            *text -= 0x20;
}

void toupper_c_simd(char* str) {
    //TODO
    return;
}

struct Variant {
    const char* name;
    void (*func)(char*);
};

typedef struct Variant Variant;

const Variant variants[] = {
    { "c", toupper_c },
    { "asm", toupper_asm },
    { "c_simd", toupper_c_simd },
    { "asm_simd", toupper_asm_simd },
    // feel free to add more variants here.
};

const char prefix[] = "abcxyzABCXYZ01`{";

int main(int argc, char** argv) {
    unsigned variant = 0;
    size_t randlen = 23;
    int maxoutlen = 50;
    const char* infile = NULL;
    const char* outfile = NULL;

    int opt;
    while ((opt = getopt(argc, argv, "r:v:")) != -1) {
        switch (opt) {
        case 'r': randlen = strtoul(optarg, NULL, 0); infile = NULL; break;
        case 'v': variant = strtoul(optarg, NULL, 0); break;
        default:
            fprintf(stderr, "usage: %s [-r randomlen] [-v variant] [infile] [outfile]\n", argv[0]);
            fprintf(stderr, "example: %s -r 35 -v 1\n", argv[0]);
            return EXIT_FAILURE;
        }
    }

    if (optind < argc)
        infile = argv[optind];

    if (optind + 1 < argc)
        outfile = argv[optind + 1];

    if (optind < argc - 2)
        fprintf(stderr, "%s: ignoring extra arguments\n", argv[0]);

    if (variant >= sizeof(variants) / sizeof(Variant)) {
        fprintf(stderr, "%s: invalid variant %u\n", argv[0], variant);
        return EXIT_FAILURE;
    }
    const Variant* variantStruct = &variants[variant];

    char* string = NULL;
    if (infile) {
        if(!(string = read_file(infile)))
            return 1;
    } else {
        string = gen_randstr(randlen);
        printf("%.*s\n", maxoutlen, string);
    }

    run(variantStruct->name, variantStruct->func, string);

    if (outfile) {
        write_file(outfile, string);
    } else {
        printf("%.*s\n", maxoutlen, string);
    }

    free(string);
    return 0;
}
