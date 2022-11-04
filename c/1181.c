#include <stdio.h>
#include <stdlib.h>

int compare(const void *a, const void *b) {
    char *a_string = (char*)a, *b_string = (char*)b;

    int a_length = 0, b_length = 0;
    for (int i = 0; a_length == 0 || b_length == 0; i++) {
        if (a_length == 0 && a_string[i] == '\0') {
            a_length = i;
        }
        if (b_length == 0 && b_string[i] == '\0') {
            b_length = i;
        }
    }

    if (a_length != b_length) {
        return a_length - b_length;
    }

    int compared = 0;
    for (int i = 0; i < 50; i++) {
        compared = a_string[i] - b_string[i];
        if (compared) {
            return compared;
        }
    }
    return 0;
}

int not_equals(char *a, char *b, int length) {
    for (int i = 0; i < length; i++) {
        if (a[i] != b[i]) {
            return i+1;
        }
        if (a[i] == '\0') {
            return 0;
        }
    }
    return 0;
}

int main() {
    int count;

    scanf("%d", &count);

    char words[count][51];

    for (int i = 0; i < count; i++) {
        scanf("%s", words[i]);
    }

    qsort(words, count, sizeof(char[51]), compare);

    for (int i = 0; i < count; i++) {
        if (i == 0 || not_equals(words[i-1], words[i], 50)) {
            printf("%s\n", words[i]);
        }
    }

    return 0;
}

