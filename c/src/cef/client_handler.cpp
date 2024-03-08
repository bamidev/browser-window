#include "client_handler.hpp"


void ClientHandler::externalInvocationHandlerFunc( bw_Application* app, void* _data ) {
	auto data = (ExternalInvocationHandlerData*)_data;

	// Slice of the command string
	bw_CStrSlice cmd_str_slice = {
		data->cmd.length(),
		data->cmd.c_str()
	};

	// Slices of the arguments
	std::vector<bw_CStrSlice> params_slices; params_slices.reserve( data->params.capacity() );
	for ( size_t i = 0; i < data->params.size(); i++ ) {
		std::string& param = data->params[i];

		// Convert the stored param into a bw_CStrSlice
		bw_CStrSlice param_str_slice = {
			param.length(),
			param.c_str()
		};
		params_slices.push_back( param_str_slice );
	}

	// Fire!
	bw_BrowserWindowMessageArgs args = {
		cmd_str_slice,
		params_slices.size(),
		&params_slices[0]
	};
	bw_Event_fire(&data->bw->events.on_message, (void*)&args);
}