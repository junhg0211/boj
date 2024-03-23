#include <stdio.h>
#include <string.h>

int main() {
    // get name
    char name[80];
    fgets(name, 80, stdin);
    name[strlen(name)-1] = 0;

    // print with ??!
    printf("%s?\?!\n", name);

    // finish
    return 0;
}
