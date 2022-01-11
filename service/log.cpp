#include "log.h"
#include <cstdlib>
#include <string>

namespace log_utils {
    static bool allEnabled = ("all" == std::getenv("G_MESSAGES_DEBUG"));

    bool isEnabled(GLogLevelFlags level) {
        return allEnabled;
    }
}
