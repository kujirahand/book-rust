#include<stdio.h>

// Rustで定義した関数を定義
int rust_mul(int a, int b);

int main() {
  // Rustで定義した関数を実行
  printf("%d\n", rust_mul(10, 8));
  printf("%d\n", rust_mul(9, 9));
  return 0;
}

