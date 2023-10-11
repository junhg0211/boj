#include <stdio.h>
#include <stdlib.h>

typedef int QueueElement;
typedef struct _linkedQueueNode {
    QueueElement element;
    struct _linkedQueueNode *next;
    struct _linkedQueueNode *previous;
} LinkedQueueNode;

typedef struct _linkedQueue {
    LinkedQueueNode *front;
    LinkedQueueNode *rear;
} LinkedQueue;

LinkedQueue *lqinit() {
    LinkedQueue *result = (LinkedQueue *)malloc(sizeof(LinkedQueue));
    return result;
}

int lqis_empty(LinkedQueue *queue) {
    return queue->front == NULL && queue->rear == NULL;
}

void lqenqueue(LinkedQueue *queue, QueueElement element) {
    LinkedQueueNode *node = (LinkedQueueNode *)malloc(sizeof(LinkedQueueNode));
    node->next = NULL;
    node->element = element;

    if (lqis_empty(queue)) {
        node->previous = NULL;
        queue->front = node;
        queue->rear = node;
        return;
    }

    queue->rear->next = node;
    node->previous = queue->rear;
    queue->rear = node;
}

QueueElement lqdequeue(LinkedQueue *queue) {
    if (lqis_empty(queue)) {
        fprintf(stderr, "queue is empty, on lqdequeue\n");
        return 0;
    }

    QueueElement result;

    if (queue->front == queue->rear) {
        result = queue->front->element;
        free(queue->front);
        queue->front = NULL;
        queue->rear = NULL;
        return result;
    }

    result = queue->front->element;
    queue->front->next->previous = NULL;
    LinkedQueueNode *front = queue->front;
    queue->front = queue->front->next;
    free(front);
    return result;
}

void lqfree(LinkedQueue *queue) {
    while (!lqis_empty(queue))
        lqdequeue(queue);

    free(queue);
}

int main() {
    int total_floors, now, target, up_offset, down_offset;
    scanf("%d %d %d %d %d",
          &total_floors, &now, &target, &up_offset, &down_offset);

    LinkedQueue *queue = lqinit();
    int beens[1000001] = {};

    lqenqueue(queue, now);
    lqenqueue(queue, 0);
    beens[now] = 1;

    while (!lqis_empty(queue)) {
        int here = lqdequeue(queue);
        int history = lqdequeue(queue);

        if (here == target) {
            printf("%d\n", history);
            lqfree(queue);
            return 0;
        }

        int up = here + up_offset;
        int down = here - down_offset;

        if (up <= total_floors && !beens[up]) {
            lqenqueue(queue, up);
            lqenqueue(queue, history + 1);
            beens[up] = 1;
        }

        if (down >= 1 && !beens[down]) {
            lqenqueue(queue, down);
            lqenqueue(queue, history + 1);
            beens[down] = 1;
        }
    }

    printf("use the stairs\n");
    lqfree(queue);
    return 0;
}
