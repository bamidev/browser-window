#include "../cookie.h"
#include "../cef/util.hpp"
#include "../common.h"

#include <string>
#include <include/cef_cookie.h>

#define CEF_COOKIE_MANAGER(COOKIE_JAR) \
	(*(CefRefPtr<CefCookieManager>*)(COOKIE_JAR)->impl.handle_ptr)



bw_Cookie* bw_Cookie_fromCef(const CefCookie& cef_cookie);

class BwCookieVisitor : public CefCookieVisitor {
public:
	bw_CookieIterator* iterator;
	std::vector<CefCookie> cookies;
	bool finished;
	bw_CookieIteratorNextCallbackFn next_cb;
	void* cb_data;

	BwCookieVisitor(bw_CookieIterator* iterator) : iterator(iterator), finished(false), next_cb(0) {}

	bool Visit(const CefCookie& cookie, int count, int total, bool& delete_cookie) override {
		UNUSED(delete_cookie);

		this->cookies.push_back(cookie);

		if (this->next_cb != 0) {
			this->next_cb(this->iterator, this->cb_data, bw_Cookie_fromCef(cookie));
			this->next_cb = 0;
		}

		if ((count+1) == total)
			this->finished = true;

		return true;
	}

protected:
	IMPLEMENT_REFCOUNTING(BwCookieVisitor);
};

class BwSetCookieCallback : public CefSetCookieCallback {
public:
	bw_CookieJar* cookie_jar;
	bw_CookieJarStorageCallbackFn cb;
	void* cb_data;

	BwSetCookieCallback(bw_CookieJar* cookie_jar, bw_CookieJarStorageCallbackFn cb, void* cb_data) : CefSetCookieCallback(),
		cookie_jar(cookie_jar), cb(cb), cb_data(cb_data)
	{}

	void OnComplete(bool success) override {

		if (cb != 0) {
			if (success) {
				BW_ERR_DECLARE_SUCCESS(success);
				this->cb(this->cookie_jar, this->cb_data, success);
			}
			else {
				bw_Err error = bw_Err_new_with_msg(1, "unable to set cookie");
				this->cb(this->cookie_jar, this->cb_data, error);
			}
		}
	}

protected:
	IMPLEMENT_REFCOUNTING(BwSetCookieCallback);
};



void bw_Cookie_free(bw_Cookie* cookie) {
	delete (CefCookie*)cookie->impl.handle_ptr;
	free(cookie);
}

bw_Cookie* bw_Cookie_fromCef(const CefCookie& cef_cookie) {
	CefCookie* cef_ptr = new CefCookie(cef_cookie);
	
	bw_Cookie* cookie = (bw_Cookie*)malloc(sizeof(bw_Cookie));
	cookie->impl.handle_ptr = (void*)cef_ptr;

	return cookie;
}

bw_Cookie* bw_Cookie_new(bw_CStrSlice name, bw_CStrSlice value) {
	CefCookie* cef_cookie = new CefCookie();
	cef_cookie->has_expires = 0;
	
	CefString(&cef_cookie->name).FromString(std::string(name.data, name.len));
	CefString(&cef_cookie->value).FromString(std::string(value.data, value.len));

	bw_Cookie* cookie = (bw_Cookie*)malloc(sizeof(bw_Cookie));
	cookie->impl.handle_ptr = (void*)cef_cookie;
	return cookie;
}

uint64_t bw_Cookie_getCreationTime(const bw_Cookie* cookie) {
	CefTime time(((CefCookie*)cookie->impl.handle_ptr)->creation);
	return time.GetDoubleT() * 1000;
}

void bw_Cookie_setCreationTime(bw_Cookie* cookie, uint64_t time) {
	CefTime cef_time;
	cef_time.SetDoubleT((double)time / 1000);
	((CefCookie*)cookie->impl.handle_ptr)->creation = cef_time;
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

uint64_t bw_Cookie_getExpires(const bw_Cookie* cookie) {
	CefCookie* cef_cookie = (CefCookie*)cookie->impl.handle_ptr;

	if (!cef_cookie->has_expires)
		return 0;
		
	CefTime time(cef_cookie->expires);
	return time.GetDoubleT() * 1000;
}

void bw_Cookie_setExpires(bw_Cookie* cookie, uint64_t time) {
	CefCookie* cef_cookie = (CefCookie*)cookie->impl.handle_ptr;

	cef_cookie->has_expires = 1;
	CefTime temp;	temp.SetDoubleT((double)time / 1000);
	cef_cookie->expires = temp;
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
	CefString string(&((CefCookie*)cookie->impl.handle_ptr)->value);
	*value = bw_cef_copyToStrSlice(string);
	return TRUE;
}

void bw_CookieJar_free(bw_CookieJar* jar) {
	delete (CefRefPtr<CefCookieManager>*)jar->impl.handle_ptr;
	free(jar);
}

void bw_CookieJar_iterator(bw_CookieJar* jar, bw_CookieIterator** iterator, BOOL include_http_only, bw_CStrSlice _url) {
	CefString url = bw_cef_copyFromStrSlice(_url);

	*iterator = (bw_CookieIterator*)malloc(sizeof(bw_CookieIterator));
	(*iterator)->impl.index = 0;
	CefRefPtr<BwCookieVisitor>* visitor = new CefRefPtr<BwCookieVisitor>(new BwCookieVisitor(*iterator));
	(*iterator)->impl.visitor_ptr = (void*)visitor;
	
	bool not_empty = CEF_COOKIE_MANAGER(jar)->VisitUrlCookies(url, include_http_only, *visitor);
	if (!not_empty)
		(*visitor)->finished = true;
}

void bw_CookieJar_iteratorAll(bw_CookieJar* jar, bw_CookieIterator** iterator) {

	*iterator = (bw_CookieIterator*)malloc(sizeof(bw_CookieIterator));
	(*iterator)->impl.index = 0;
	CefRefPtr<BwCookieVisitor>* visitor = new CefRefPtr<BwCookieVisitor>(new BwCookieVisitor(*iterator));
	(*iterator)->impl.visitor_ptr = (void*)visitor;
	
	bool not_empty = CEF_COOKIE_MANAGER(jar)->VisitAllCookies(*visitor);
	if (!not_empty)
		(*visitor)->finished = true;
}

bw_CookieJar* bw_CookieJar_newGlobal() {

	CefRefPtr<CefCookieManager>* mgr = new CefRefPtr<CefCookieManager>(CefCookieManager::GetGlobalManager(0));

	bw_CookieJar* cj = (bw_CookieJar*)malloc(sizeof(bw_CookieJar));
	cj->impl.handle_ptr = mgr;

	return cj;
}

bw_Err bw_CookieJar_store(bw_CookieJar* jar, bw_CStrSlice url, const bw_Cookie* cookie, bw_CookieJarStorageCallbackFn cb, void* cb_data) {
	CefCookie cef_cookie = *(CefCookie*)cookie->impl.handle_ptr;
	CefString cef_url = bw_cef_copyFromStrSlice(url);

	CefRefPtr<BwSetCookieCallback> cef_cb(new BwSetCookieCallback(jar, cb, cb_data));

	if (!CEF_COOKIE_MANAGER(jar)->SetCookie(cef_url, cef_cookie, cef_cb))
		return bw_Err_new_with_msg(1, "invalid characters in cookie or invalid url");

	BW_ERR_RETURN_SUCCESS;
}

void bw_CookieIterator_free(bw_CookieIterator* iterator) {
	delete (CefRefPtr<BwCookieVisitor>*)iterator->impl.visitor_ptr;
	free(iterator);
}

BOOL bw_CookieIterator_next(bw_CookieIterator* iterator, bw_CookieIteratorNextCallbackFn on_next, void* cb_data) {
	CefRefPtr<BwCookieVisitor> visitor = *(CefRefPtr<BwCookieVisitor>*)iterator->impl.visitor_ptr;

	size_t& index = iterator->impl.index;

	if (index >= visitor->cookies.size()) {
		
		// If finished, return false
		if (visitor->finished)
			return FALSE;
		// If not yet finished, wait on the next cookie
		else {
			// If next_cb already set, we can't do anything.
			BW_ASSERT(visitor->next_cb == 0, "cookie iterator is already waiting on next item");
			
			visitor->next_cb = on_next;
			visitor->cb_data = cb_data;
		}
	}
	// If cookie already available, return it immediately
	else {
		bw_Cookie* cookie = bw_Cookie_fromCef(visitor->cookies[index]);
		on_next(iterator, cb_data, cookie);
	}

	index += 1;

	return true;
}