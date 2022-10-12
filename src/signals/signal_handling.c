#include "signal.h"
#include "stddef.h"

void signals_listen(void (*handler)(int)) {
  // The functions defined here should never fail with current usage according
  // to the man pages.
  struct sigaction signal_action = {.sa_handler = handler, .sa_flags = 0};

  sigemptyset(&signal_action.sa_mask);

  sigaction(SIGHUP, &signal_action, NULL);
  sigaction(SIGINT, &signal_action, NULL);
  sigaction(SIGQUIT, &signal_action, NULL);
  sigaction(SIGTERM, &signal_action, NULL);
}
