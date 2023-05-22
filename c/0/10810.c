#include <stdio.h>

int main()
{
    int balls, sessions;
    scanf("%d %d", &balls, &sessions);

    int baguni[balls];
    for (int i = 0; i < balls; i++)
    {
        baguni[i] = 0;
    }

    int from, to, what;
    for (int i = 0; i < sessions; i++)
    {
        scanf("%d %d %d", &from, &to, &what);
        for (int j = from - 1; j < to; j++)
        {
            baguni[j] = what;
        }
    }

    for (int i = 0; i < balls; i++)
    {
        printf("%d ", baguni[i]);
    }

    return 0;
}
