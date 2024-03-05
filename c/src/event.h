#ifndef BW_EVENT_H
#define BW_EVENT_H

#ifdef __cplusplus
extern "C" {
#endif

#include "bool.h"


typedef BOOL (*bw_EventCallbackFn)(void* arg, void* event_data);

typedef struct {
    bw_EventCallbackFn callback;
    void* data;
} bw_Event;


BOOL bw_Event_fire(bw_Event* event, void* data);


#ifdef __cplusplus
}
#endif

#endif//BW_EVENT_H