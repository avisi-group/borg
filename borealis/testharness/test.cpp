#include <aarch64-decode.h>

#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mman.h>
#include <sys/stat.h>
#include <unistd.h>
#include <new>

int main(int argc, char **argv)
{
    int fd = open("input.bin", O_RDONLY);
    if (fd < 0)
    {
        return EXIT_FAILURE;
    }

    struct stat64 st;
    fstat64(fd, &st);

    void *code = mmap(nullptr, st.st_size, PROT_READ, MAP_PRIVATE, fd, 0);
    close(fd);

    if (code == MAP_FAILED)
    {
        return EXIT_FAILURE;
    }

    const uint32_t *start = (const uint32_t *)code;
    const uint32_t *end = (const uint32_t *)((unsigned long)code + st.st_size);

    static char decode_data[128];

    while (start < end)
    {
        captive::arch::aarch64::aarch64_decode *d = new (decode_data) captive::arch::aarch64::aarch64_decode();

        d->decode(0, 0, (const uint8_t *)start);
        printf("instruction=%x, opcode=%d\n", *(const uint32_t *)start, d->opcode);

        start++;
    }

    return EXIT_SUCCESS;
}
