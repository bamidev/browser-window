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

bw_StrSlice bw_Cookie_getName(const bw_Cookie* cookie) {
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->name);
	return bw_cef_copyToStrSlice(string);
}

bw_StrSlice bw_Cookie_getValue(const bw_Cookie* cookie) {
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->value);
	return bw_cef_copyToStrSlice(string);
}

void bw_Cookie_setValue(bw_Cookie* cookie, bw_CStrSlice value) {
	std::string temp(value.data, value.len);
	CefString(&((CefCookie*)cookie->impl.handle_ptr)->value).FromString(temp);
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