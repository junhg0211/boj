#include <stdio.h>
#include <string.h>

int main() {
    int testcase_count;
    scanf("%d\n", &testcase_count);

    for (int i = 0; i < testcase_count; i++) {
        char string[100];
        fgets(string, 100, stdin);
        int length = strlen(string);

        int streak = 0;
        int score = 0;
        for (int j = 0; j < length - 1; j++) {
            if (string[j] == 'O') {
                streak += 1;
            } else {
                streak = 0;
            }
            score += streak;
        }

        printf("%d\n", score);
    }

    return 0;
}
