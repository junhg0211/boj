#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

typedef struct Route {
    int start;
    int end;
    int cost;
} Route;

bool contains(int value, int bottom, int top) {
    return bottom <= value && value <= top;
}

bool is_duplicate(Route* a, Route* b) {
    return contains(a->start, b->start, b->end)
        || contains(a->end, b->start, b->end)
        || contains(b->start, a->start, a->end)
        || contains(b->end, a->start, a->end);
}

int min(int a, int b) {
    return a < b ? a : b;
}

int max(int a, int b) {
    return a < b ? b : a;
}

Route* merge(Route* a, Route* b) {
    Route* result = (Route*)malloc(sizeof(Route));
    result->start = min(a->start, b->start);
    result->end = max(a->end, b->end);
    result->cost = min(a->cost, b->cost);
    return result;
}

int main() {
    int number;

    Route* routes[200001];

    scanf("%d", &number);
    for (int i = 0; i < number; i++) {
        Route* route = (Route*)malloc(sizeof(Route));
        scanf("%d %d %d", &route->start, &route->end, &route->cost);
        routes[i] = route;
    }

    int offset = 0;
    for (int i = 0; i < number - offset; i++) {
        for (int j = i + 1; j < number - offset; j++) {
            if (is_duplicate(routes[i], routes[j])) {
                routes[i] = merge(routes[i], routes[j]);
                offset++;
                for (int k = j; k < number - offset; k++) {
                    routes[k] = routes[k+1];
                }
                j--;
            }
        }
    }

    printf("%d\n", number - offset);
    for (int i = 0; i < number - offset; i++) {
        Route route = *routes[i];
        printf("%d %d %d\n", route.start, route.end, route.cost);
    }

    return 0;
}
