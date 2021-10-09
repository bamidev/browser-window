#include "../cookie.h"
#include "../cef/util.hpp"

#include <string>
#include <include/cef_cookie.h>

#define CEF_COOKIE_MANAGER(COOKIE_JAR) \
	(*(CefRefPtr<CefCookieManager>*)(COOKIE_JAR)->impl.handle_ptr)



class BwCookieVisitor : public CefCookieVisitor {
public:
	std::vector<CefCookie> cookies;
	bool finished;

	BwCookieVisitor() : finished(false) {}

	bool Visit(const CefCookie& cookie, int count, int total, bool& deleteCookie) override {
		this->cookies.push_back(cookie);

		return true;
	}

protected:
	IMPLEMENT_REFCOUNTING(BwCookieVisitor);
};



void bw_Cookie_free(bw_Cookie* cookie) {
	delete (CefCookie*)cookie->impl.handle_ptr;
	free(cookie);
}

bw_Cookie* bw_Cookie_new(bw_CStrSlice name, bw_CStrSlice value) {
	CefRefPtr<CefCookie>* cef_cookie = new CefRefPtr<CefCookie>();

	CefString(&(*cef_cookie)->name).FromString(std::string(name.data, name.len));
	CefString(&(*cef_cookie)->value).FromString(std::string(value.data, value.len));

	bw_Cookie* cookie = (bw_Cookie*)malloc(sizeof(bw_Cookie));
	cookie->impl.handle_ptr = (void*)cef_cookie;
	return cookie;
}

unsigned long bw_Cookie_getCreationTime(const bw_Cookie* cookie) {
	CefTime time(((CefCookie*)cookie->impl.handle_ptr)->creation);
	return time.GetDoubleT() * 1000;
}

void bw_Cookie_setCreationTime(bw_Cookie* cookie, unsigned long time) {
	CefTime cef_time(((CefCookie*)cookie->impl.handle_ptr)->creation);
	cef_time.SetDoubleT((double)time / 1000);
}

BOOL bw_Cookie_getDomain(const bw_Cookie* cookie, bw_StrSlice* domain) {
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->domain);
	*domain = bw_cef_copyToStrSlice(string);
	return TRUE;
}

void bw_Cookie_setDomain(bw_Cookie* cookie, bw_CStrSlice domain) {
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->domain);
	string.FromString(std::string(domain.data, domain.len));
}

unsigned long bw_Cookie_getExpires(const bw_Cookie* cookie) {
	CefCookie* cef_cookie = (CefCookie*)cookie->impl.handle_ptr;

	if (cef_cookie->has_expires)
		return 0;

	CefTime time(cef_cookie->expires);
	return time.GetDoubleT() * 1000;
}

void bw_Cookie_setExpires(bw_Cookie* cookie, unsigned long time) {
	CefCookie* cef_cookie = (CefCookie*)cookie->impl.handle_ptr;

	cef_cookie->has_expires = 1;
	CefTime(cef_cookie->expires).SetDoubleT((double)time / 1000);
}

void bw_Cookie_setName(bw_Cookie* cookie, bw_CStrSlice name) {
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->name);
	string.FromString(std::string(name.data, name.len));
}

BOOL bw_Cookie_getPath(const bw_Cookie* cookie, bw_StrSlice* path) {
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->path);
	*path = bw_cef_copyToStrSlice(string);
	return TRUE;
}

void bw_Cookie_setPath(bw_Cookie* cookie, bw_CStrSlice path) {
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->path);
	string.FromString(std::string(path.data, path.len));
}

void bw_Cookie_setValue(bw_Cookie* cookie, bw_CStrSlice value) {
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->value);
	string.FromString(std::string(value.data, value.len));
}

BOOL bw_Cookie_isHttpOnly(const bw_Cookie* cookie) {
	return ((CefCookie*)cookie->impl.handle_ptr)->httponly;
}
void bw_Cookie_makeHttpOnly(bw_Cookie* cookie) {
	((CefCookie*)cookie->impl.handle_ptr)->httponly = 1;
}
BOOL bw_Cookie_isSecure(const bw_Cookie* cookie) {
	return ((CefCookie*)cookie->impl.handle_ptr)->secure;
}
void bw_Cookie_makeSecure(bw_Cookie* cookie) {
	((CefCookie*)cookie->impl.handle_ptr)->secure = 1;
}

BOOL bw_Cookie_getName(const bw_Cookie* cookie, bw_StrSlice* name) {
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->name);
	*name = bw_cef_copyToStrSlice(string);
	return TRUE;
}

BOOL bw_Cookie_getValue(const bw_Cookie* cookie, bw_StrSlice* value) {
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->name);
	*value = bw_cef_copyToStrSlice(string);
	return TRUE;
}

void bw_CookieJar_free(bw_CookieJar* jar) {
	delete (CefRefPtr<BwCookieVisitor>*)jar->impl.handle_ptr;
	free(jar);
}

void bw_CookieJar_iterator(bw_CookieJar* jar, bw_CookieIterator** iterator, BOOL include_http_only, bw_CStrSlice _url) {
	CefString url = bw_cef_copyFromStrSlice(_url);
	
	CefRefPtr<BwCookieVisitor>* visitor = new CefRefPtr<BwCookieVisitor>(new BwCookieVisitor());
	CEF_COOKIE_MANAGER(jar)->VisitUrlCookies(url, include_http_only, *visitor);
	(*visitor)->finished = true;

	*iterator = (bw_CookieIterator*)malloc(sizeof(bw_CookieIterator));
	(*iterator)->impl.index = 0;
	(*iterator)->impl.visitor_ptr = (void*)visitor;
}

bw_CookieJar* bw_CookieJar_newGlobal() {

	CefRefPtr<CefCookieManager>* mgr = new CefRefPtr<CefCookieManager>(CefCookieManager::GetGlobalManager(0));

	bw_CookieJar* cj = (bw_CookieJar*)malloc(sizeof(bw_CookieJar));
	cj->impl.handle_ptr = mgr;

	return cj;
}

void bw_CookieJar_store(bw_CookieJar* jar, const bw_Cookie* cookie) {
	CefRefPtr<CefCookieManager> mgr = *(CefRefPtr<CefCookieManager>*)jar->impl.handle_ptr;
	CefCookie cef_cookie = *(CefCookie*)cookie->impl.handle_ptr;

	std::string url = CefString(&cef_cookie.domain).ToString();
	url += CefString(&cef_cookie.path).ToString();
	CefString cef_url;
	cef_url.FromString(url);

	mgr->SetCookie(cef_url, cef_cookie, nullptr);
}

void bw_CookieIterator_free(bw_CookieIterator* iterator) {
	delete (CefRefPtr<BwCookieVisitor>*)iterator->impl.visitor_ptr;
	free(iterator);
}

BOOL bw_CookieIterator_next(bw_CookieIterator* iterator, bw_Cookie** cookie_out) {
	CefRefPtr<BwCookieVisitor> visitor = *(CefRefPtr<BwCookieVisitor>*)iterator->impl.visitor_ptr;

	if (visitor->finished)
		return false;
	
	size_t index = iterator->impl.index;
	CefCookie* cef_cookie = new CefCookie(visitor->cookies[index]);
	
	bw_Cookie* cookie = (bw_Cookie*)malloc(sizeof(bw_Cookie));
	cookie->impl.handle_ptr = cef_cookie;
	*cookie_out = cookie;

	return true;
}