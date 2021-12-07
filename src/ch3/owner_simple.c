#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main() {
  // メモリを確保して文字列をコピー --- (*1)
  char *g1 = (char *)malloc(100);
  strcpy(g1, "穏やかな心は体に良い");
  // 変数g2にg1を代入 --- (*2)
  char *g2 = g1;
  // g2の内容を表示 --- (*3)
  printf("%s\n", g2);
  // メモリの破棄 --- (*4)
  free(g2);
  // うっかり以下を実行するとメモリの二重解放
  // free(g1); 
  return 0;
}

