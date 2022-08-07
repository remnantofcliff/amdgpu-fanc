/*
 * Sets handler for signals that should quit the program. Returns 0 on success
 * or a negative value to indicate error.
 */
int signals_listen(void (*handler)(int));
