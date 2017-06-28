#include  <stdio.h>
#include <sys/types.h>
#include <unistd.h>
#include <sys/ipc.h>
#include  <sys/shm.h>
#include  <string.h>
#include <errno.h>

#define MAX_MEM 4096

int main()

{

  int shmid;
  int ret;
  void* mem;

  shmid=shmget(0x12367,MAX_MEM,0);
  mem=shmat(shmid,(const void*)0,0);
  //printf("%s\n",(char*)mem);
  if(mem==(void *) -1)
  {
    fprintf(stderr,"shmat return NULL ,errno(%d:%s)\n",errno,strerror(errno));
    return 2;
  }
  printf("%s\n",(char*)mem);

  shmdt(mem);

  return 0;
}
