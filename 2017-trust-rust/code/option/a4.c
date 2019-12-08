#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <unistd.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>

int main() 
{
    int fd, n;
    char buf[1024];

    fd = open("abc", O_RDONLY);

    // should check whether fd < 0

    n = read(fd, buf, sizeof(buf));

    if (n < 0) {
        perror("error in read");
        exit(1);
    }

    return 0;
}
