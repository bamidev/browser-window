#ifndef BW_CEF_EVAL_CALLBACK_STORE
#define BW_CEF_EVAL_CALLBACK_STORE

#include "../browser_window.h"
#include "../err.h"

#include <map>
#include <mutex>
#include <include/cef_base.h>



namespace bw {

	union EvalCallbackResult {
		CefString result;	// The eval result
		bw_Err error;	// An error if it occurred

		EvalCallbackResult() : result( CefString() ) {}
		EvalCallbackResult( CefString result ) : result(result) {}
		EvalCallbackResult( bw_Err error ) : error(error) {}
		~EvalCallbackResult() {}
	};

	// All data that is necessary to call the callback
	struct EvalCallbackData {
		bw_BrowserWindow* bw;
		bw_BrowserWindowJsCallbackFn callback;
		void* user_data;
	};

	// A thread safe store of eval js callbacks.
	class EvalCallbackStore {
		std::map<unsigned int, EvalCallbackData> cb_store;
		unsigned int next_key;
		std::mutex mutex;

	public:
		// The only constructor is the default contructor
		EvalCallbackStore();

		// Stores callback data in this store.
		// An identifier will be returned with which the callback can be invoked.
		unsigned int store( bw_BrowserWindow* bw, bw_BrowserWindowJsCallbackFn callback, void* user_data );
		// Stores callback data in this store.
		// An identifier will be returned with which the callback can be invoked.
		unsigned int store( EvalCallbackData data );

		// Invokes a stored callback, only when the callback ID exists in this store.
		// Returns whether or not the callback exists in the store.
		// When actually invoked, the callback will disappear from this store.
		bool invoke( unsigned int callback_id, bool success, const EvalCallbackResult& result );
	};

	// A global instance
	extern EvalCallbackStore eval_callback_store;
}



#endif//BW_CEF_EVAL_CALLBACK_STORE
