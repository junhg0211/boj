#include <stdio.h>

int main() {
    int hour, minute;
    int delta;
    scanf("%d %d", &hour, &minute);
    scanf("%d", &delta);

    minute += delta;

    hour += minute / 60;
    minute %= 60;
    hour %= 24;

    printf("%d %d\n", hour, minute);

    return 0;
}
