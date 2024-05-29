#include <stdio.h>

void tick() {
  char flips[41];
  scanf("%s", flips);

  int count[8] = {};

  for (int i = 2; i < 40; i++) {
    char pp = flips[i - 2];
    char p = flips[i - 1];
    char n = flips[i];

    int index = 0;
    if (pp == 'H') {
      index |= 4;
    }
    if (p == 'H') {
      index |= 2;
    }
    if (n == 'H') {
      index |= 1;
    }

    count[index]++;
  }

  for (int i = 0; i < 8; i++) {
    printf("%d ", count[i]);
  }
  printf("\n");
}

int main() {
  int testcase_count;
  scanf("%d", &testcase_count);

  for (int i = 0; i < testcase_count; i++) {
    tick();
  }

  return 0;
}
