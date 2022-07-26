#include "signal_handling.hpp"
#include <csignal>

static bool received = false;

static void signal_handler(int) { received = true; }

int8_t signals::listen() {
  static struct sigaction signal_action;
  sigemptyset(&signal_action.sa_mask);
  signal_action.sa_flags = 0;
  signal_action.sa_handler = signal_handler;

  return sigaction(SIGHUP, &signal_action, nullptr) +
         sigaction(SIGINT, &signal_action, nullptr) +
         sigaction(SIGQUIT, &signal_action, nullptr) +
         sigaction(SIGTERM, &signal_action, nullptr);
}

bool signals::should_close() { return received; }
