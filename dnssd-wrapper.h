// https://developer.apple.com/documentation/dnssd
// https://developer.apple.com/documentation/dnssd/dns_service_discovery_c
#include <dns_sd.h>

fd_set new_fd_set() {
    fd_set set;
    FD_ZERO(&set);
    return set;
}
void bonjour_fd_clr(int fd, fd_set* set) {
    FD_CLR(fd, set);
}
void bonjour_fd_isset(int fd, fd_set* set) {
    FD_ISSET(fd, set);
}
void bonjour_fd_set(int fd, fd_set* set) {
    FD_SET(fd, set);
}
void bonjour_fd_zero(fd_set* set) {
    FD_ZERO(set);
}