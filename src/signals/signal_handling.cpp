#include "signal_handling.hpp"
#include <atomic>
#include <csignal>

static std::atomic_bool received(false);

static void signal_handler(int) {
  received.store(true, std::memory_order_relaxed);
}

static constexpr struct sigaction construct_signal_action() {
  struct sigaction signal_action {};
  signal_action.sa_handler = signal_handler;
  return signal_action;
}

int8_t signals::listen() {
  struct sigaction signal_action = construct_signal_action();

  return sigaction(SIGHUP, &signal_action, nullptr) +
         sigaction(SIGINT, &signal_action, nullptr) +
         sigaction(SIGQUIT, &signal_action, nullptr) +
         sigaction(SIGTERM, &signal_action, nullptr);
}

bool signals::should_close() {
  return received.load(std::memory_order_relaxed);
}
