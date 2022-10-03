#include <stdio.h>

int markSource(int i) {
  return i; 
}

int sink(int i) {
  return i;
}

int foo(int in) {
  return in << 4;
}

int main(int argc, char const *argv[]) {
  printf("Hello, world!\n");
  int a = 23;
  markSource(a);
  int b = foo(a);
  sink(b);
  printf("%d\n", b);
  return 0;
}

