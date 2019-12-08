#include "morse.h"

#include <stdio.h>

#define BUF_SIZE 1024

int main() {
	char buf[BUF_SIZE];
	while (fgets(buf, BUF_SIZE, stdin)) {
		demorse(buf);
		puts(buf);
	}
	return 0;
}
