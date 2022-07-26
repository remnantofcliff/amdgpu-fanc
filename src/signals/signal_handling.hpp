#include <cstdint>

namespace signals {

/*
 * Sets the callbacks for listening to signals.
 *
 * Returns 0 if successful or a negative value to indicate error (between 0
 * and -4)
 */
int8_t listen();

/*
 * Returns true if at least one signal was handled. Otherwise returns false.
 *
 * Will always return false if signal_listen() was never called.
 */
bool should_close();

}; // namespace signals
