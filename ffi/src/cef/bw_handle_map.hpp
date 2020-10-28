#ifndef BW_CEF_BW_HANDLE_MAP
#define BW_CEF_BW_HANDLE_MAP

#include "../browser_window.h"

#include <include/cef_browser.h>
#include <optional>
#include <map>
#include <mutex>



namespace bw {

	// A thread safe class that links CEF browser handles to our browser window handdles.
	// This makes it possible to get a bw_BrowserWindow* from a CefRefPtr<CefBrowser>, from any thread.
	class BwHandleMap {
		// The CefBrowser's GetIdentifier output is used as the key to identify CEF browser handles
		std::map<int, bw_BrowserWindow*> map;
		std::mutex mutex;

	public:
		// The only constructor is the default constructor
		BwHandleMap() {}

		// Remove a link
		void drop( CefRefPtr<CefBrowser> cef_handle ) {
			this->mutex.lock();
				this->map.erase( cef_handle->GetIdentifier() );
			this->mutex.unlock();
		}

		// Stores a link
		void store( CefRefPtr<CefBrowser> cef_handle, bw_BrowserWindow* our_handle ) {
			this->mutex.lock();
				this->map[ cef_handle->GetIdentifier() ] = our_handle;
			this->mutex.unlock();
		}

		// Fetches a bw_BrowserWindow handle from a cef handle.
		// Returns an optional bw_BrowserWindow pointer.
		std::optional<bw_BrowserWindow*> fetch( CefRefPtr<CefBrowser> cef_handle ) {
			this->mutex.lock();
				auto it = this->map.find( cef_handle->GetIdentifier() );

				// If not found return nothing
				if ( it == this->map.end() )
					return std::optional<bw_BrowserWindow*>();

				// If found:
				std::optional<bw_BrowserWindow*> result( (*it).second );
			this->mutex.unlock();

			return result;
		}
	};

	// A global instance
	extern BwHandleMap bw_handle_map;
}



#endif//BW_CEF_BW_HANDLE_MAP
