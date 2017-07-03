#include <stdio.h>
#include <mqueue.h>
#include <errno.h>
#include <string.h>
#include <stdlib.h>
#include <stddef.h>
#include <sys/stat.h>

int main(int argc, char** argv)
{
    int message_size = argc > 1 ? atoi(argv[1]) : 6;
    int queue_size = argc > 2 ? atoi(argv[2]) : 1000;

    printf("Trying to open queue with msgsize: %d, and maxmsg: %d\n", message_size, queue_size);

    struct mq_attr initial_attributes = (struct mq_attr){
            .mq_msgsize = message_size,
            .mq_maxmsg = queue_size
    };

    int open_flags = O_RDWR | O_CREAT | O_EXCL;
    int permissions = S_IWUSR | S_IRUSR;

    const char* name = "/message_queue_name;";
    mqd_t queue = mq_open(name, open_flags, permissions, &initial_attributes);

    if(queue == -1)
    {
        printf("Cannot open message queue\n");
        printf("Errno: %d [%s]\n", errno, strerror(errno));
        return 1;
    }
    else
    {
        printf("Queue has been opened successfully. Closing...\n");
        mq_close(queue);
        mq_unlink(name);
    }

    return 0;
}
