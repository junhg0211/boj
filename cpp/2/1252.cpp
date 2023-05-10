#include <iostream>

int main()
{
    // get number
    std::string ar, br;
    std::cin >> ar >> br;

    int al = ar.length(), bl = br.length();

    std::string result = "";
    bool carry = false;
    bool ap, bp;
    for (int i = 0; i < 80; i++)
    {
        ap = i < al && ar.at(al - i - 1) == '1';
        bp = i < bl && br.at(bl - i - 1) == '1';

        result = (ap ^ bp ^ carry ? '1' : '0') + result;
        carry = ap&bp | ap&carry | bp&carry;
    }

    if (carry)
        std::cout << '1' << result << std::endl;
    else
    {
        int i = 0;
        for (i = 0; i < result.length(); i++)
        {
            if (result.at(i) == '1')
                break;
        }
        if (i == result.length())
            std::cout << "0";
        else
        {
            for (; i < result.length(); i++)
                std::cout << result.at(i);
        }

        std::cout << std::endl;
    }

    return 0;
}
