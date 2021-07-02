#include <stdio.h>

extern int entry() asm("entry");

int main(int argc, char** argv) { 
  int result = entry();
  printf("%d\n", result);
  return 0;
}
