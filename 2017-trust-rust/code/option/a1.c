#include <stdio.h>

int main()
{
    FILE *fp;
    char buf[1000];

    fp = fopen("abc.txt", "r");

    fscanf(fp, "%s", buf);
}
