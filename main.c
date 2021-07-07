#include <stdio.h>

extern int entry() asm("start");

int main(int argc, char** argv) { 
  long result = start();
  printf("res: %ld \n", result);
  return 0;
}
