//
// Created by 今村秀明 on 2018/04/21.
//

#include <stdio.h>
#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <unistd.h>
#include <vector>

const int N = 1000000;

void encryption(char *buf, int n) {
    for (int i = 0; i < n; ++i) {
        buf[i] += 1;
    }
}

int main() {
    int sock0;
    struct sockaddr_in addr;
    struct sockaddr_in client;
    socklen_t len;
    int sock;
    char buf[N];
    ssize_t rd;

    sock0 = socket(AF_INET, SOCK_STREAM, 0);

    addr.sin_family = AF_INET;
    addr.sin_port = htons(12344);
    addr.sin_addr.s_addr = INADDR_ANY;

    bind(sock0, (struct sockaddr *) &addr, sizeof(addr));

    listen(sock0, 5);
    len = sizeof(client);
    sock = accept(sock0, (struct sockaddr *) &client, &len);
    while (1) {
        rd = read(sock, buf, N);
        printf("%ld\n", rd / sizeof(char));
        if (rd <= 1) break;
        encryption(buf, int(rd) - 1);
        write(sock, buf, rd);
    }
    close(sock);
    close(sock0);
    return 0;
}
