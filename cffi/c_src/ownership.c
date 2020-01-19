#include <stdlib.h>
#include <stdio.h>

void take_ownership(int *i, void(*dtor)(int *)) {
  printf("got %d\n",*i);

  // Cのコードでメモリを開放するが
  // デストラクタ処理はRustから渡してもらう
  dtor(i);
}

int *make_memory() {
  int *i;
  i = malloc(sizeof(int));
  *i=2;
  return i;
}