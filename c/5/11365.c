#include <stdio.h>
#include <string.h>

int input(char *str)
{
    int i;
    char c;
    for (i = 0; (c = getchar()) != '\n'; i++)
    {
        str[i] = c;
    }
    str[i] = '\0';
    return i;
}

int main(void)
{
    char password[500];
    int length;

    while (1)
    {
        length = input(password);

        if (strcmp(password, "END") == 0)
            break;

        for (int i = length-1; i >= 0; i--)
        {
            printf("%c", password[i]);
        }
        printf("\n");
    }

    return 0;
}
