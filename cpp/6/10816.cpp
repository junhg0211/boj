#include <iostream>
#include <map>

int main() {
    std::map<int, int> counts;

    int card_count;
    scanf("%d", &card_count);
    for (int i = 0; i < card_count; i++) {
        int card;
        scanf("%d", &card);

        auto it = counts.find(card);
        if (it == counts.end()) {
            counts.insert(std::pair<int, int>(card, 1));
        } else {
            it->second++;
        }
    }

    int question_count;
    scanf("%d", &question_count);
    for (int i = 0; i < question_count; i++) {
        int card;
        scanf("%d", &card);

        auto it = counts.find(card);
        if (it == counts.end()) {
            printf("0 ");
            continue;
        }

        int count = counts.find(card)->second;
        printf("%d ", count);
    }
    printf("\n");

    return 0;
}
