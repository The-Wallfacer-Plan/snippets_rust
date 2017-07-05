#include <unistd.h>

int main(void) {
    char* bin = "/usr/bin/printenv";
    char*argv[] = {bin, "MY_VAR", NULL};
    char*envp[] = {"MY_VAR=lol", NULL};
    execve(bin, argv, envp);
    return 0;
}
