#ifndef BW_COOKIE_H
#define BW_COOKIE_H

#ifdef __cplusplus
extern "C" {
#endif

#include "bool.h"
#include "err.h"
#include "string.h"

#include <stdint.h>

#ifdef BW_CEF
#include "cookie/cef.h"
#else
struct bw_CookieImpl { void* _; };
struct bw_CookieIteratorImpl { void* _; };
struct bw_CookieJarImpl { void* _; };
#endif



typedef struct {
	struct bw_CookieImpl impl;
} bw_Cookie;

typedef struct {
	struct bw_CookieJarImpl impl;
} bw_CookieJar;

typedef struct {
	struct bw_CookieIteratorImpl impl;
} bw_CookieIterator;

typedef void (*bw_CookieJarStorageCallbackFn)( bw_CookieJar* cj, void* data, bw_Err error );
typedef void (*bw_CookieIteratorNextCallbackFn)(bw_CookieIterator* ci, void* data, bw_Cookie* cookie);
typedef void (*bw_CookieJarDeleteCallbackFn)(bw_CookieJar* cj, void* data, unsigned int deleted);



void bw_Cookie_free(bw_Cookie* cookie);
bw_Cookie* bw_Cookie_new(bw_CStrSlice name, bw_CStrSlice value);
uint64_t bw_Cookie_getCreationTime(const bw_Cookie* cookie);
void bw_Cookie_setCreationTime(bw_Cookie* cookie, uint64_t time);
BOOL bw_Cookie_getDomain(const bw_Cookie* cookie, bw_StrSlice* domain);
void bw_Cookie_setDomain(bw_Cookie* cookie, bw_CStrSlice domain);
uint64_t bw_Cookie_getExpires(const bw_Cookie* cookie);
void bw_Cookie_setExpires(bw_Cookie* cookie, uint64_t time);
BOOL bw_Cookie_getName(const bw_Cookie* cookie, bw_StrSlice* name);
void bw_Cookie_setName(bw_Cookie* cookie, bw_CStrSlice name);
BOOL bw_Cookie_getPath(const bw_Cookie* cookie, bw_StrSlice* path);
void bw_Cookie_setPath(bw_Cookie* cookie, bw_CStrSlice path);
BOOL bw_Cookie_getValue(const bw_Cookie* cookie, bw_StrSlice* value);
void bw_Cookie_setValue(bw_Cookie* cookie, bw_CStrSlice value);
BOOL bw_Cookie_isHttpOnly(const bw_Cookie* cookie);
void bw_Cookie_makeHttpOnly(bw_Cookie* cookie);
BOOL bw_Cookie_isSecure(const bw_Cookie* cookie);
void bw_Cookie_makeSecure(bw_Cookie* cookie);

void bw_CookieJar_delete(bw_CookieJar* jar, bw_CStrSlice url, bw_CStrSlice name, bw_CookieJarDeleteCallbackFn cb, void* cb_data);
void bw_CookieJar_free(bw_CookieJar* jar);
void bw_CookieJar_iterator(bw_CookieJar* jar, bw_CookieIterator** iterator, BOOL include_http_only, bw_CStrSlice url);
void bw_CookieJar_iteratorAll(bw_CookieJar* jar, bw_CookieIterator** iterator);
bw_CookieJar* bw_CookieJar_newGlobal();
bw_Err bw_CookieJar_store(bw_CookieJar* jar, bw_CStrSlice url, const bw_Cookie* cookie, bw_CookieJarStorageCallbackFn cb, void* cb_data);

void bw_CookieIterator_free(bw_CookieIterator* iterator);
BOOL bw_CookieIterator_next(bw_CookieIterator* iterator, bw_CookieIteratorNextCallbackFn on_next, void* cb_data);



#ifdef __cplusplus
}
#endif

#endif//BW_COOKIE_H