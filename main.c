#include <stdio.h>

extern int entry() asm("start");

int main(int argc, char** argv) { 
  char result = start();
  printf("res: %c \n", result);
  return 0;
}
