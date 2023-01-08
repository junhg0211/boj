#include <stdio.h>
#include <math.h>

void tick()
{
    int number;
    scanf("%d", &number);

    int limit = ceil(sqrt(number));
    for (int i = 2; i <= limit; i++)
    {
        int count = 0;
        while (number % i == 0)
        {
            number /= i;
            count++;
        }

        if (count > 0)
        {
            printf("%d %d\n", i, count);
        }
    }

    if (number != 1)
    {
        printf("%d 1\n", number);
    }
}

int main(void)
{
    int count;
    scanf("%d", &count);

    for (int i = 0; i < count; i++)
    {
        tick();
    }

    return 0;
}
