# Download CEF archive
$CEF_ARCHIVE = "cef_binary_122.1.12+g6e69d20+chromium-122.0.6261.112_windows64_minimal"

$ErrorActionPreference = "Stop"


mkdir -f cef
if (!(Test-Path "cef\$CEF_ARCHIVE.tar.bz2")) {
	"Downloading CEF..."
	curl -o "cef\$CEF_ARCHIVE.tar.bz2.part" "https://cef-builds.spotifycdn.com/$CEF_ARCHIVE.tar.bz2"
	mv "cef\$CEF_ARCHIVE.tar.bz2.part" "cef\$CEF_ARCHIVE.tar.bz2"
}

if (!(Test-Path "cef\$CEF_ARCHIVE")) {
	"Unpacking CEF..."
	tar -xvf "cef\$CEF_ARCHIVE.tar.bz2" -C cef
}

"Compiling CEF..."
try {
	cd "cef\$CEF_ARCHIVE"

	# Add compilation definitions to the top of the CMakeLists.txt file
	if (!(Test-Path "CMakeLists.txt.def")) {
		mv CMakeLists.txt CMakeLists.txt.old
		Set-Content -Path "CMakeLists.txt.def" -Value "add_compile_definitions(NDEBUG=1 DCHECK_ALWAYS_ON=1)"
		Get-Content CMakeLists.txt.def, CMakeLists.txt.old | Set-Content -Path "CMakeLists.txt"
	}

	cmake .
	cmake --build . --config Release
	cmake --build libcef_dll_wrapper --config Release --target libcef_dll_wrapper
}
finally {
	cd ..\..
}
`
"CEF is ready. Add the following path to with name CEF_PATH to your environment variables:"
"$PWD\cef\$CEF_ARCHIVE"

$Env:CEF_PATH = "$PWD\cef\$CEF_ARCHIVE"
