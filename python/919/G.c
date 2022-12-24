#include <stdio.h>
#include <math.h>

void tick()
{
    double variance = 0;
    double f;
    for (int i = 0; i < 5000; i++)
    {
        scanf("%lf", &f);
        variance += (f-0.5)*(f-0.5);
    }
    variance /= 5000;

    if (fabs(1/12 - variance) > fabs(0.1 - variance))
    {
        printf("A\n");
    }
    else
    {
        printf("B\n");
    }
}

int main(void)
{
    for (int i = 0; i < 1; i++)
    {
        tick();
    }

    return 0;
}
