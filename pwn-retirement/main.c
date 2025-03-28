#include <stdio.h>
#include <ctype.h>

int main() {
    char buf[35];
    char *ptr = buf;

    puts("<Insert prompt here>: ");

    gets(buf);
    
    // atBash cipher
    for (int i = 0; ptr[i] != '\0'; i++) {
        if (isupper(ptr[i])) {
            ptr[i] = 'Z' - (ptr[i] - 'A');
        } else if (islower(ptr[i])) {
            ptr[i] = 'z' - (ptr[i] - 'a');
        }
    }

    printf(buf);
}