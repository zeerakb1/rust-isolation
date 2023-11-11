#include <stddef.h>
#include <limits.h>

void modify_vector(int* ptr, size_t len) {
    for (size_t i = 0; i < len; i++) {
        ptr[i] = ptr[i + 10000];
    }
    ptr[len] = -99999;
}