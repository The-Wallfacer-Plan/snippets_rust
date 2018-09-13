#ifndef _STRUCTS_H_
#define _STRUCTS_H_

#include <stdlib.h>

struct A {
  int a;
  float b;
  int ca[10];
  struct A *p;
};

typedef struct {
  int index;
  struct A aa[3];
} B;

B func1(void);

float func2(int i) {
  return float(i);
}

#endif
