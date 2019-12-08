#include <pthread.h>
#include <stdio.h>

#define N 1000000

int count = 0;

void* fun(void *x) 
{
    int i;
    for(i = 0; i < N; i++)
        count++; 
}

int main()
{
    pthread_t t1, t2;

    pthread_create(&t1, 0, fun, 0);
    pthread_create(&t2, 0, fun, 0);

    pthread_join(t1, 0);
    pthread_join(t2, 0);

    printf("count = %d\n", count);
}
