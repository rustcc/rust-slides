#include <strings.h>
#include <stdio.h>

int main()
{
    char s[100] = "hello";
    char *p;

    p = index(s, 'e');
    *p = 'a';

    printf("%s\n", s);

    return 0;
}
