#include <stdio.h>

int main(void)
{
    int count, target_sum;
    scanf("%d %d", &count, &target_sum);

    int numbers[20];
    for (int i = 0; i < count; i++)
    {
        scanf("%d", &numbers[i]);
    }

    int result = 0;
    int sum;

    for (int i = 1; i < 1 << count; i++)
    {
        sum = 0;

        int combination = i;
        int j = 0;
        while (combination)
        {
            if (combination & 1)
                sum += numbers[j];
            combination >>= 1;
            j++;
        }

        if (sum == target_sum)
        {
            result++;
        }
    }

    printf("%d\n", result);

    return 0;
}
