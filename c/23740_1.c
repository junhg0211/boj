#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>

typedef struct Route {
    int start;
    int end;
    int cost;
} Route;

bool contains(const int* array, int length, int value) {
    for (int i = 0; i < length; i++) {
        if (array[i] == value)
            return true;
    }
    return false;
}

int min(int a, int b) { return a < b ? a : b; }
int max(int a, int b) { return a < b ? b : a; }

void routes_containing_point(
        const int point, const Route* routes, const int route_count,
        Route* result_routes, int* start_min, int* end_max, int* fee_min) {
    int result_count = 0;
    *start_min = 1000000000;
    *end_max = 0;
    *fee_min = 1000000000;
    for (int i = 0; i < route_count; i++) {
        Route route = routes[i];
        if (route.start <= point && point <= route.end) {
            result_routes[++result_count] = route;
            *start_min = min(route.start, *start_min);
            *end_max = max(route.end, *end_max);
            *fee_min = min(route.cost, *fee_min);
        }
    }
}

int main() {
    int number;
    int points[400001];
    int points_count = 0;

    scanf("%d", &number);
    for (int i = 0; i < number; i++) {
        Route* route = (Route*)malloc(sizeof(Route));
        scanf("%d %d %d", &route->start, &route->end, &route->cost);
        if (!contains(points, points_count, route->start)) {
            points[++points_count] = route->start;
        }
        if (!contains(points, points_count, route->end)) {
            points[++points_count] = route->end;
        }
    }

    Route* results[200001]; int results_count = 0;
    int previous_intersects[400001]; int previous_intersects_count = 0;

    return 0;
}