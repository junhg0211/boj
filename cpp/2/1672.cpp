#include <iostream>

char get_dna(char previous, char now)
{
    if (previous == now)
        return now;

    if (previous == 'A')
    {
        if (now == 'G')
            return 'C';
        else if (now == 'C')
            return 'A';

        return 'G';
    }

    if (previous == 'G')
    {
        if (now == 'A')
            return 'C';
        else if (now == 'C')
            return 'T';

        return 'A';
    }

    if (previous == 'C')
    {
        if (now == 'A')
            return 'A';
        else if (now == 'G')
            return 'T';

        return 'G';
    }

    // if previous == 'T'
    if (now == 'A')
        return 'G';
    else if (now == 'G')
        return 'A';

    return 'G';
}

int main()
{
    // getting DNA
    int length;
    std::cin >> length;

    char *dna = new char[length];
    std::cin >> dna;

    // calculating DNA
    for (int i = length-2; i >= 0; i--)
        dna[i] = get_dna(dna[i], dna[i+1]);

    // printing DNA
    std::cout << dna[0] << std::endl;

    // freeing memory
    delete[] dna;

    return 0;
}
