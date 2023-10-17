#include <stdio.h>
#include <string.h>

int main() {
    char word[1000001];
    scanf("%s", word);

    int frequency[26];
    int length = strlen(word);
    for (int i = 0; i < length; i++) {
        char c = word[i];

        if (c > 'Z') {
            c -= 'a' - 'A';
        }

        c -= 'A';

        frequency[c]++;
    }

    int max_frequency = 0;
    char max_char = 0;
    int duplicated = 0;
    for (int i = 0; i < 26; i++) {
        if (frequency[i] > max_frequency) {
            max_frequency = frequency[i];
            max_char = 'A' + i;
            duplicated = 0;
        } else if (frequency[i] == max_frequency) {
            duplicated = 1;
        }
    }

    if (duplicated) {
        printf("?\n");
    } else {
        printf("%c\n", max_char);
    }

    return 0;
}
