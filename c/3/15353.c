#include <stdio.h>
#include <string.h>

int max(int a, int b) { return a > b ? a : b; }

int main() {
    char a[10001] = {0}, b[10001] = {0};
    char c[10001] = {0};

    scanf("%s %s", a, b);

    int a_length = strlen(a);
    int b_length = strlen(b);

    int length = max(a_length, b_length);
    int carry = 0;
    int i;
    for (i = 0; i < length || carry > 0; i++) {
        int a_i = a_length - i - 1;
        int b_i = b_length - i - 1;

        int a_j = a_i < 0 ? 0 : a[a_i] - '0';
        int b_j = b_i < 0 ? 0 : b[b_i] - '0';

        int c_j = (a_j + b_j + carry) % 10;
        carry = (a_j + b_j + carry) / 10;

        c[10000 - i] = '0' + c_j;
    }

    for (int j = 10000 - i + 1; j <= 10000; j++) {
        printf("%c", c[j]);
    }
    printf("\n");

    return 0;
}
