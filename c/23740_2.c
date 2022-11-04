#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

bool contains(const int* value, const int* bottom, const int* top) {
    return *bottom <= *value && *value <= *top;
}

int min(const int* a, const int* b) {
    return *a < *b ? *a : *b;
}

int max(const int* a, const int* b) {
    return *a < *b ? *b : *a;
}

typedef struct Route {
    int start;
    int end;
    int cost;
} Route;

Route* pop(Route** routes, const int* index, const int* length) {
    Route* result = routes[*index];
    for (int i = *index; i < *length - 1; i++) {
        routes[i] = routes[i+1];
    }
    return result;
}

void sort(Route** routes, const int* length) { // NOLINT(misc-no-recursion)
    Route* smalls[100001]; int smalls_count = 0;
    Route* bigs[100001]; int bigs_count = 0;
    Route* anchor = routes[0];
    for (int i = 0; i < *length; i++) {
        Route* route = routes[i];
        if (route->start < anchor->start) {
            smalls[smalls_count++] = route;
        } else if (route->start > anchor->start) {
            bigs[bigs_count++] = route;
        }
    }
    if (smalls_count > 1) sort(smalls, &smalls_count);
    for (int i = 0; i < smalls_count; i++) {
        routes[i] = smalls[i];
    }
    if (bigs_count > 1) sort(bigs, &bigs_count);
    for (int i = 0; i < bigs_count; i++) {
        routes[i + smalls_count + 1] = bigs[i];
    }
    routes[smalls_count] = anchor;
}

int main() {
    Route* routes[200001];
    int routes_count = 0;

    int number;
    scanf("%d", &number); // NOLINT(cert-err34-c)
    for (int _ = 0; _ < number; _++) {
        int s, e, c;
        scanf("%d %d %d", &s, &e, &c); // NOLINT(cert-err34-c)
        for (int i = 0; i < routes_count; i++) {
            Route* route = routes[i];
            if (contains(&s, &route->start, &route->end)
                    || contains(&e, &route->start, &route->end)
                    || contains(&route->start, &s, &e)
                    || contains(&route->end, &s, &e)) {
                pop(routes, &i, &routes_count);
                i--; routes_count--;
                s = min(&route->start, &s);
                e = max(&route->end, &e);
                c = min(&route->cost, &c);
            }
        }
        Route* route = (Route*)malloc(sizeof(Route));
        route->start = s;
        route->end = e;
        route->cost = c;
        routes[routes_count++] = route;
    }

    printf("%d\n", routes_count);
    sort(routes, &routes_count);
    for (int i = 0; i < routes_count; i++) {
        Route* route = routes[i];
        printf("%d %d %d\n", route->start, route->end, route->cost);
    }

    for (int i = 0; i < routes_count; i++) {
        free(routes[i]);
    }

    return 0;
}
