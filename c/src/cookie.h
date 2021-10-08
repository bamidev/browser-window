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
bw_StrSlice bw_Cookie_getName(const bw_Cookie* cookie);
bw_StrSlice bw_Cookie_getValue(const bw_Cookie* cookie);

void bw_CookieJar_free(bw_CookieJar* jar);
void bw_CookieJar_iterator(bw_CookieJar* jar, bw_CookieIterator** iterator, BOOL include_http_only, bw_CStrSlice url);
bw_CookieJar* bw_CookieJar_newGlobal();
void bw_CookieJar_setCookie(bw_CookieJar* jar, bw_CStrSlice name, bw_CStrSlice value);

void bw_CookieIterator_free(bw_CookieIterator* iterator);
BOOL bw_CookieIterator_next(bw_CookieIterator* iterator, bw_Cookie** cookie);



#ifdef __cplusplus
}
#endif

#endif//BW_COOKIE_H