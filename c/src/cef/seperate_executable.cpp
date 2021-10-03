#include "app_handler.hpp"

#include <include/cef_app.h>
#include <include/cef_base.h>



int main(int argc, char* argv[]) {
#ifdef BW_MACOS
	// Initialize the macOS sandbox for this helper process.
	CefScopedSandboxContext sandbox_context;
	if (!sandbox_context.Initialize(argc, argv))
	return 1;

	// Load the CEF framework library at runtime instead of linking directly
	// as required by the macOS sandbox implementation.
	CefScopedLibraryLoader library_loader;
	if (!library_loader.LoadInHelper())
	return 1;
#endif

	// Structure for passing command-line arguments.
	// The definition of this structure is platform-specific.
	CefMainArgs main_args(argc, argv);

	// Optional implementation of the CefApp interface.
	CefRefPtr<CefApp> app( new AppHandler( 0 ) );

	// Execute the sub-process logic. This will block until the sub-process should exit.
	return CefExecuteProcess(main_args, app, 0);
}