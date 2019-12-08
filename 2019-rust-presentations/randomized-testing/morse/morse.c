#include "morse.h"

#include <stdlib.h>

static const char *tbl = " ETIANMSURWDKGOHVF?L?PJBXCYZQ??54?3???2???????16???????7???8?90";

void demorse(char *buff) {
	size_t acc = 1;
	char *write = buff;
	for (char *c = buff; *c; c ++) {
		switch (*c) {
			case '.':
				acc *= 2;
				break;
			case '-':
				acc = acc * 2 + 1;
				break;
			default:
				*write ++ = tbl[acc - 1];
				acc = 1;
		}
	}
	*write = '\0';
}
