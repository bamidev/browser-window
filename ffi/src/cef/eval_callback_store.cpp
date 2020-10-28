#include "eval_callback_store.hpp"


namespace bw {

	EvalCallbackStore eval_callback_store;

	EvalCallbackStore::EvalCallbackStore() : next_key(0) {}

	unsigned int EvalCallbackStore::store( bw_BrowserWindow* bw, bw_BrowserWindowJsCallbackFn callback, void* user_data ) {

		EvalCallbackData cb_data = {
			bw,
			callback,
			user_data
		};

		return this->store( cb_data );
	}

	unsigned int EvalCallbackStore::store( EvalCallbackData data ) {

		this->mutex.lock();
			unsigned int key = this->next_key;
			this->next_key++;

			this->cb_store[ key ] = data;
		this->mutex.unlock();

		return key;
	}

	bool EvalCallbackStore::invoke( unsigned int callback_id, bool success, const EvalCallbackResult& _result ) {

		this->mutex.lock();
			auto cb_iterator = this->cb_store.find( callback_id );

			// If the ID is not found, stop and return false
			if ( cb_iterator == this->cb_store.end() ) {
				this->mutex.unlock();
				return false;
			}

			// Remove the callback from the store
			this->cb_store.erase( cb_iterator );
		this->mutex.unlock();

		EvalCallbackData& cb_data = (*cb_iterator).second;

		// Pass the result string or the error allong with the callback, depending on whether or not the evaluation succeeded.
		if ( success ) {
			std::string result = _result.result.ToString();
			cb_data.callback( cb_data.bw, cb_data.user_data, result.c_str(), 0 );
		}
		else {
			cb_data.callback( cb_data.bw, cb_data.user_data, 0, &_result.error );
		}

		return true;
	}
}
