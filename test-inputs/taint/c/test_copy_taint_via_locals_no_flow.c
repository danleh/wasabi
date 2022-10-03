#include <stdio.h>

int markSource(int i) {
  return i; 
}

int sink(int i) {
  return i;
}

int main(int argc, char const *argv[]) {
  printf("Hello, world!\n");
  int a = 23;
  markSource(a);
  int b = 5;
  sink(b);
  printf("%d\n", b);
  return 0;
}

