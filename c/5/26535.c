#include <stdio.h>
#include <math.h>

int main() {
    // get number of chickens
    int count;
    scanf("%d", &count);

    // calculate side size
    int size = (int)ceil(sqrt(count));

    // -- print square pen

    // top
    for (int i = 0; i < size + 2; i++) {
        printf("x");
    }
    printf("\n");

    // middle
    for (int j = 0; j < size; j++) {
        printf("x");
        for (int i = 0; i < size; i++) {
            printf(".");
        }
        printf("x\n");
    }

    // bottom
    for (int i = 0; i < size + 2; i++) {
        printf("x");
    }

    return 0;
}
