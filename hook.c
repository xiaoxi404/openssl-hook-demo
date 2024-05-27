#include <dlfcn.h>
#include <openssl/ssl.h>

extern void rust_ssl_write(const char *buffer);

int SSL_write(SSL *context, const void *buffer, int bytes)
{
    int (*new_ssl_write)(SSL *context, const void *buffer, int bytes);
    new_ssl_write = dlsym(RTLD_NEXT, "SSL_write");

    // call rust ssl_write
    rust_ssl_write((char *)buffer);
    
    return new_ssl_write(context, buffer, bytes);
}