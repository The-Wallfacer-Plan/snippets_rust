struct StructA {
  int i_field;
  float f_field;
  int array_field[10];
  struct StructA *p_field;
};

typedef struct {
  int field_b;
  struct StructA array_a[3];
} StructBTy;

struct StructC {};

StructBTy func1(void);

float func2(int i) { return (float)i; }
