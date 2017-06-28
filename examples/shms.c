#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ipc.h>
#include <sys/shm.h>
#include <sys/types.h>
#include <time.h>

int SIZE = 20;

int get_id() {
  int key = rand();
  int shm_id = shmget(key, SIZE, IPC_CREAT | 0666);
  printf("%d, %d\n", key, shm_id);
  if (shm_id < 0) {
    perror("shmget");
    exit(1);
  }
  return shm_id;
}

void remove_shm(int shm_id) { shmctl(shm_id, IPC_RMID, NULL); }

int main(void) {
  srand(time(NULL));
  puts("1");
  int shm_id = get_id();
  puts("2");
  // char* pa = shmat(shm_id, (const void*)0, 0);
  char* pa = shmat(shm_id, NULL, 0);
  if (pa == (char*)(-1)) {
    perror("shmat");
    exit(1);
  }
  puts("3");
  char const* ss = "ss";
  puts("4");
  strcpy(pa, ss);
  puts("5");
  printf("%s\n", pa);
  remove_shm(shm_id);
}
