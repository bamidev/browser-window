#ifndef BW_CEF_BW_HANDLE_MAP
#define BW_CEF_BW_HANDLE_MAP

#include "../browser_window.h"

#include <include/cef_browser.h>
#include <optional>
#include <map>
#include <mutex>



namespace bw {

	struct OnCreateCallback {
		bw_BrowserWindowCreationCallbackFn callback;
		void* data;
	};

	struct BrowserInfo {
		bw_BrowserWindow* handle;
		std::optional<OnCreateCallback> callback;
	};

	// A thread safe class that links CEF browser handles to our browser window handdles.
	// This makes it possible to get a bw_BrowserWindow* from a CefRefPtr<CefBrowser>, from any thread.
	class HandleMap {
		// The CefBrowser's GetIdentifier output is used as the key to identify CEF browser handles
		std::map<int, BrowserInfo> map;
		std::mutex mutex;

	public:
		// The only constructor is the default constructor
		HandleMap() {}

		// Remove a link
		void drop( CefRefPtr<CefBrowser> cef_handle ) {
			this->mutex.lock();
			this->map.erase(cef_handle->GetIdentifier());
			this->mutex.unlock();
		}

		// Stores a link
		void store(CefRefPtr<CefBrowser> cef_handle, bw_BrowserWindow* our_handle, bw_BrowserWindowCreationCallbackFn callback, void* callback_data) {
			BrowserInfo& bw_info = this->map[cef_handle->GetIdentifier()];
			bw_info.handle = our_handle;
			bw_info.callback = std::optional(OnCreateCallback {
				callback,
				data: callback_data
			});
			this->mutex.unlock();
		}

		// Fetches a bw_BrowserWindow handle from a cef handle.
		// Returns an optional bw_BrowserWindow pointer.
		std::optional<BrowserInfo> fetch( CefRefPtr<CefBrowser> cef_handle ) {
			this->mutex.lock();
				auto it = this->map.find( cef_handle->GetIdentifier() );

				// If not found return nothing
				if ( it == this->map.end() )
					return std::optional<BrowserInfo>();

				// If found:
				std::optional<BrowserInfo> result( (*it).second );
			this->mutex.unlock();

			return result;
		}
	};

	// A global instance
	extern HandleMap bw_handle_map;
}



#endif//BW_CEF_BW_HANDLE_MAP
