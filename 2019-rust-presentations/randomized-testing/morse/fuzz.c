#include "morse.h"

#include <stdint.h>
#include <stdlib.h>
#include <string.h>

extern int LLVMFuzzerTestOneInput(const uint8_t *data, size_t size) {
	char buf[size + 1];
	memcpy(buf, data, size);
	buf[size] = '\0';
	demorse(buf);
	return 0;
}
