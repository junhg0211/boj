#include <stdio.h>

int main() {
  int a, b, c;
  scanf("%d %d %d", &a, &b, &c);

  int number = a * b * c;
  int count[10] = {};
  while (number > 0) {
    count[number % 10]++;
    number /= 10;
  }

  for (int i = 0; i < 10; i++) {
    printf("%d\n", count[i]);
  }

  return 0;
}
