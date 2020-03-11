#include "os_uart.h"
#include "uart.h"

int usend(
    unsigned char    *data,
    int    len
)
{
    int rv = 0;
    rv = uart_write(data,len);
    if(rv>0)
        return 0;
    else
        return -1;
}

int uget(
    unsigned char    *data,
    int    len
)
{

    int rv = 0;
    rv = uart_read(data,len);
    if(rv>0)
        return 0;
    else
        return -1;
}