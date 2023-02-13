#include <stdio.h>

int main(void)
{
    int sum = 0;

    int number;
    while (1)
    {
        scanf("%d", &number);
        if (number == -1)
            break;

        sum += number;
    }

    printf("%d\n", sum);

    return 0;
}
