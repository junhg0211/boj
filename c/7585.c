#include <stdio.h>
#include <string.h>

typedef struct _Stack {
    char stack[250];
    int length;
} Stack;

Stack stnew() {
    Stack stack;
    stack.length = 0;
    return stack;
}

int stis_empty(Stack *stack) {
    return stack->length == 0;
}

void stpush(Stack *stack, char value) {
    stack->stack[stack->length++] = value;
}

char stpop(Stack *stack) {
    if (stis_empty(stack)) {
        return 0;
    }

    return stack->stack[--stack->length];
}

int main() {
    char string[300];

    while (1) {
        // -- init stack
        Stack stack = stnew();

        // -- get line
        fgets(string, 300, stdin);
        string[strlen(string)-1] = 0;

        // -- terminate condition
        if (strcmp(string, "#") == 0) {
            break;
        }

        // -- do
        int length = strlen(string);
        int yes = 1;
        for (int i = 0; i < length; i++) {
            int c = string[i];
            if (c == '[' || c == '(' || c == '{') {
                stpush(&stack, c);
                continue;
            }

            if (c == ']') {
                if (stpop(&stack) != '[') {
                    yes = 0;
                    break;
                }
                continue;
            }

            if (c == '}') {
                if (stpop(&stack) != '{') {
                    yes = 0;
                    break;
                }
                continue;
            }

            if (c == ')') {
                if (stpop(&stack) != '(') {
                    yes = 0;
                    break;
                }
            }
        }

        if (!stis_empty(&stack)) {
            yes = 0;
        }

        // -- print result
        if (yes) {
            printf("Legal\n");
        } else {
            printf("Illegal\n");
        }
    }

    return 0;
}
