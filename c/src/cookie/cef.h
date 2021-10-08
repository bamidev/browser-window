#ifndef BW_COOKIE_CEF_H
#define BW_COOKIE_CEF_H

#include <stdint.h>



struct bw_CookieImpl {
	void* handle_ptr;
};

struct bw_CookieJarImpl {
	void* handle_ptr;
};

struct bw_CookieIteratorImpl {
	size_t index;
	void* visitor_ptr;
};



#endif//BW_COOKIE_CEF_H