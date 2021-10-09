#ifndef BW_COOKIE_H
#define BW_COOKIE_H

#ifdef __cplusplus
extern "C" {
#endif

#include "bool.h"
#include "string.h"

#if defined(BW_CEF)
#include "cookie/cef.h"
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



void bw_Cookie_free(bw_Cookie* cookie);
bw_Cookie* bw_Cookie_new(bw_CStrSlice name, bw_CStrSlice value);
unsigned long bw_Cookie_getCreationTime(const bw_Cookie* cookie);
void bw_Cookie_setCreationTime(bw_Cookie* cookie, unsigned long time);
BOOL bw_Cookie_getDomain(const bw_Cookie* cookie, bw_StrSlice* domain);
void bw_Cookie_setDomain(bw_Cookie* cookie, bw_CStrSlice domain);
unsigned long bw_Cookie_getExpires(const bw_Cookie* cookie);
void bw_Cookie_setExpires(bw_Cookie* cookie, unsigned long time);
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

void bw_CookieJar_free(bw_CookieJar* jar);
void bw_CookieJar_iterator(bw_CookieJar* jar, bw_CookieIterator** iterator, BOOL include_http_only, bw_CStrSlice url);
bw_CookieJar* bw_CookieJar_newGlobal();
void bw_CookieJar_store(bw_CookieJar* jar, const bw_Cookie* cookie);

void bw_CookieIterator_free(bw_CookieIterator* iterator);
BOOL bw_CookieIterator_next(bw_CookieIterator* iterator, bw_Cookie** cookie);



#ifdef __cplusplus
}
#endif

#endif//BW_COOKIE_H