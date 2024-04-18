#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_STACK_SIZE 30

typedef struct {
    int stack[MAX_STACK_SIZE];
    int length;
} Stack;

Stack *new_stack() {
    Stack *stack = (Stack *)malloc(sizeof(Stack));
    stack->length = 0;
    return stack;
}

void free_stack(Stack *stack) { free(stack); }

int is_empty(Stack *stack) { return stack->length == 0; }

void push(Stack *stack, int value) {
    stack->stack[stack->length] = value;
    stack->length++;
}

int pop(Stack *stack) {
    int result = stack->stack[stack->length - 1];
    stack->length--;
    return result;
}

int peek(Stack *stack) { return stack->stack[stack->length - 1]; }

int main() {
    char string[31];
    scanf("%s", string);

    Stack *parenthesis = new_stack();
    Stack *numbers = new_stack();
    push(numbers, 0);

    int error = 0;
    int string_length = strlen(string);
    for (int i = 0; i < string_length; i++) {
        char c = string[i];

        if (c == '(' || c == '[') {
            push(parenthesis, c);
            push(numbers, 0);
            continue;
        }

        if (is_empty(parenthesis)) {
            error = 1;
            break;
        }

        if (c == ']' && peek(parenthesis) != '[') {
            error = 1;
            break;
        }
        if (c == ')' && peek(parenthesis) != '(') {
            error = 1;
            break;
        }
        pop(parenthesis);

        int buffer = 0;
        while (1) {
            int number = pop(numbers);
            buffer += number;
            if (number == 0) {
                break;
            }
        }
        buffer = buffer == 0 ? 1 : buffer;

        push(numbers, buffer * (c == ']' ? 3 : 2));
    }

    if (!is_empty(parenthesis)) {
        error = 1;
    }

    if (error) {
        printf("0\n");
    } else {
        int value = 0;
        while (!is_empty(numbers)) {
            value += pop(numbers);
        }

        printf("%d\n", value);
    }

    free_stack(numbers);
    free_stack(parenthesis);

    return 0;
}
