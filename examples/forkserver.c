#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <sys/time.h>

int main() {
    static struct itimerval it;
    int st_pipe[2], ctl_pipe[2];
    int status;
    int rlen;
    int forksv_pid;
    int fsrv_st_fd;

    if (pipe(st_pipe) || pipe(ctl_pipe)) {
        printf("pip() failed!\n");
        return 1;
    }

    forksv_pid = fork();
    printf("forksv_pid is %d\n", forksv_pid);

    if (forksv_pid < 0) {
        printf("fork() failed!\n");
        return 1;
    }

    if (!forksv_pid) {
        // setsid();
        if (dup2(ctl_pipe[0], 198) < 0) {
            printf("dup2() failed\n");
            return 1;
        }
        if (dup2(st_pipe[1], 199) < 0) {
            printf("dup2() failed\n");
            return 1;
        }
        close(ctl_pipe[0]);
        close(ctl_pipe[1]);
        close(st_pipe[0]);
        close(st_pipe[1]);
        char* path = "/home/lyk/rs-snippets/examples/main";
        char* argv[] = {"/home/lyk/rs-snippets/examples/main", NULL};
        execv(path, argv);
        printf("execv failed! path: %s, Err: %s\n", path, strerror(errno));
        return 1;
    }
    close(ctl_pipe[0]);
    close(st_pipe[1]);



    fsrv_st_fd = st_pipe[0];


    it.it_value.tv_sec = 0;
    it.it_value.tv_usec = 500000;

    setitimer(ITIMER_REAL, &it, NULL);

    rlen = read(fsrv_st_fd, &status, 4);

    if (rlen == 4) {
        printf("forkserver set up ok!\n");
        return 0;
    }
    else {
        printf("forkserver set up failed!, rlen is %d\n", rlen);
        return 1;
    }


}