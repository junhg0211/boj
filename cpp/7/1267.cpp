#include <iostream>

int yeongsik(int duration)
{
    return (duration / 30 + 1) * 10;
}

int minsik(int duration)
{
    return (duration / 60 + 1) * 15;
}

int main()
{
    int count;
    std::cin >> count;

    int y = 0, m = 0;
    for (int i = 0; i < count; ++i)
    {
        int duration;
        std::cin >> duration;

        y += yeongsik(duration);
        m += minsik(duration);
    }

    if (y < m)
        std::cout << "Y " << y << std::endl;
    else if (y > m)
        std::cout << "M " << m << std::endl;
    else
        std::cout << "Y M " << y << std::endl;

    return 0;
}
