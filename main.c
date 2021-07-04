#include <stdio.h>

extern int entry() asm("start");

int main(int argc, char** argv) { 
  int result = start();
  printf("res: %d\n", result);
  return 0;
}
