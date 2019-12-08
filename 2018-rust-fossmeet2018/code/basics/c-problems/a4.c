#include <stdlib.h>

void fun(char *t)
{
    /* do some stuff here */
    
    free(t);

}

int main()
{
    char *c;

    c =  malloc(10 * sizeof(char));
    fun(c);

    c[0] = 'A'; //bug! user-after-free
}



