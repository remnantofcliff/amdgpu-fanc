#include "signal_handling.h"
#include "signal.h"
#include "stddef.h"

int8_t signals_listen(void (*handler)(int)) {
  struct sigaction signal_action;
  signal_action.sa_handler = handler;
  signal_action.sa_flags = 0;
  sigemptyset(&signal_action.sa_mask);

  return sigaction(SIGHUP, &signal_action, NULL) +
         sigaction(SIGINT, &signal_action, NULL) +
         sigaction(SIGQUIT, &signal_action, NULL) +
         sigaction(SIGTERM, &signal_action, NULL);
}
