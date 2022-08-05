#include <stdint.h>

/*
 * Sets handler for signals that should quit the program. Returns 0 on success
 * or a negative value to indicate error.
 */
int8_t signals_listen(void (*handler)(int));
