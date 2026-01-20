#include <stdio.h>

void tick() {
    int case_number;
    int numbers[20];
    int swap_count = 0;

    scanf("%d", &case_number);
    for (int i = 0; i < 20; i++) {
        scanf("%d", &numbers[i]);
    };

    for (int i = 0; i < 20; i++) {
        for (int j = i-1; j >= 0; j--) {
            if (numbers[j] > numbers[j+1]) {
                swap_count++;

                int temp = numbers[j+1];
                numbers[j+1] = numbers[j];
                numbers[j] = temp;
            }
        }
    }

    printf("%d %d\n", case_number, swap_count);
}

int main() {
    int case_count;
    scanf("%d", &case_count);

    for (; case_count > 0; case_count--) {
        tick();
    }
    return 0;
}
