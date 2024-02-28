#include "../cookie.h"

#include "../common.h"


void bw_Cookie_free(bw_Cookie* cookie) {}

bw_Cookie* bw_Cookie_new(bw_CStrSlice name, bw_CStrSlice value) { return NULL; }

uint64_t bw_Cookie_getCreationTime(const bw_Cookie* cookie) { return 0; }

void bw_Cookie_setCreationTime(bw_Cookie* cookie, uint64_t time) {}

BOOL bw_Cookie_getDomain(const bw_Cookie* cookie, bw_StrSlice* domain) { return FALSE; }

void bw_Cookie_setDomain(bw_Cookie* cookie, bw_CStrSlice domain) {}

uint64_t bw_Cookie_getExpires(const bw_Cookie* cookie) { return 0; }

void bw_Cookie_setExpires(bw_Cookie* cookie, uint64_t time) {}

void bw_Cookie_setName(bw_Cookie* cookie, bw_CStrSlice name) {}

BOOL bw_Cookie_getPath(const bw_Cookie* cookie, bw_StrSlice* path) { return FALSE; }

void bw_Cookie_setPath(bw_Cookie* cookie, bw_CStrSlice path) {}

void bw_Cookie_setValue(bw_Cookie* cookie, bw_CStrSlice value) {}

BOOL bw_Cookie_isHttpOnly(const bw_Cookie* cookie) { return FALSE; }

void bw_Cookie_makeHttpOnly(bw_Cookie* cookie) {}

BOOL bw_Cookie_isSecure(const bw_Cookie* cookie) { return FALSE; }

void bw_Cookie_makeSecure(bw_Cookie* cookie) {}

BOOL bw_Cookie_getName(const bw_Cookie* cookie, bw_StrSlice* name) { return FALSE; }

BOOL bw_Cookie_getValue(const bw_Cookie* cookie, bw_StrSlice* value) { return FALSE; }

void bw_CookieJar_delete(bw_CookieJar* jar, bw_CStrSlice _url, bw_CStrSlice _name, bw_CookieJarDeleteCallbackFn cb, void* cb_data) {}

void bw_CookieJar_free(bw_CookieJar* jar) {}

void bw_CookieJar_iterator(bw_CookieJar* jar, bw_CookieIterator** iterator, BOOL include_http_only, bw_CStrSlice _url) {}

void bw_CookieJar_iteratorAll(bw_CookieJar* jar, bw_CookieIterator** iterator) {}

bw_CookieJar* bw_CookieJar_newGlobal() { return NULL; }

bw_Err bw_CookieJar_store(bw_CookieJar* jar, bw_CStrSlice url, const bw_Cookie* cookie, bw_CookieJarStorageCallbackFn cb, void* cb_data) {
	BW_ERR_RETURN_SUCCESS;
}

void bw_CookieIterator_free(bw_CookieIterator* iterator) {}

extern "C" BOOL bw_CookieIterator_next(bw_CookieIterator* iterator, bw_CookieIteratorNextCallbackFn on_next, void* cb_data) { return FALSE; }
