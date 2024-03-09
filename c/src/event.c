#include "event.h"

#include <stddef.h>


BOOL bw_Event_fire(bw_Event* event, void* arg) {
    if (event->callback != NULL) {
        return event->callback(event->data, arg);
    }
    return FALSE;
}