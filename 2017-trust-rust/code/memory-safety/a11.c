#include <limits.h>
#include <stdio.h>

int main() {
    int c = INT_MAX;

    if (c+1 < c) 
        printf("hello\n");

    printf("%d\n", c+1);
}
