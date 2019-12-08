#include <stdio.h>
static void (*Do)();

static void EraseAll() {
  printf("remove all files....\n");
}

void NeverCalled() {
  Do = EraseAll;  
}

int main() {
  Do();
}

// I compiled with clang 3.8.0 with -O3 flag
// and got some interesting results!

// One of those programs which kills your hope
// of someday "understanding" C

// Ref: https://www.reddit.com/r/cpp/comments/6xeqr3/compiler_undefined_behavior_calls_nevercalled/ 

