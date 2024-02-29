#include "signal.h"
#include "stddef.h"
#include <stdbool.h>

bool signals_listen(void (*handler)(int)) {
  // The functions defined here should never fail with current usage according
  // to the man pages.
  struct sigaction signal_action = {.sa_handler = handler, .sa_flags = 0};

  if (sigemptyset(&signal_action.sa_mask) == -1 ||
      sigaction(SIGHUP, &signal_action, NULL) == -1 ||
      sigaction(SIGINT, &signal_action, NULL) == -1 ||
      sigaction(SIGQUIT, &signal_action, NULL) == -1 ||
      sigaction(SIGTERM, &signal_action, NULL) == -1) {
    return false;
  }

  return true;
}
