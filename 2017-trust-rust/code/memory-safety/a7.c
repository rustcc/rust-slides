#include <stdlib.h>

void fun()
{
    char *c;
    c = malloc(10*sizeof(char));

    /* do some stuff here */

    free(c);
}

int main()
{
    fun(); 
}



