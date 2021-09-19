#include <stdio.h>

const char* print_str(char *str, int len) {
    printf("str: %s\nlen: %d\n", str, len);
    return "Done!";
}