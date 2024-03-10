# Download CEF archive
$CEF_ARCHIVE = "cef_binary_122.1.12+g6e69d20+chromium-122.0.6261.112_windows64_minimal"


$ErrorActionPreference = "Stop"


mkdir -f hoi
if (!(Test-Path "cef\$CEF_ARCHIVE.tar.bz2")) {
	"Downloading CEF..."
	#$client = new-object System.Net.WebClient
	#$client.DownloadFile("https://cef-builds.spotifycdn.com/$CEF_ARCHIVE.tar.bz2", "cef\$CEF_ARCHIVE.tar.bz2")
	curl -o "cef\$CEF_ARCHIVE.tar.bz2.part" "https://cef-builds.spotifycdn.com/$CEF_ARCHIVE.tar.bz2"
	mv "cef\$CEF_ARCHIVE.tar.bz2.part" "cef\$CEF_ARCHIVE.tar.bz2"
}

"Unpacking CEF..."
tar -xvf "cef\$CEF_ARCHIVE.tar.bz2" -C cef

"Compiling CEF..."
try {
	cd "cef\$CEF_ARCHIVE"

	# Add compilation definitions to the top of the CMakeLists.txt file
	mv CMakeLists.txt CMakeLists.txt.old
	[IO.File]::WriteAllLines("CMakeLists.txt", "add_compile_definitions(DCHECK_ALWAYS_ON=1)")
	$FROM = Get-Content -Path "CMakeLists.txt.old"
	Add-Content -Path "CMakeLists.txt" -Value $FROM

	cmake .
	cmake --build .
}
finally {
	cd ..\..
}
`
"CEF is ready. Add the following path to with name CEF_PATH to your environment variables:"
"$PWD\cef\$CEF_ARCHIVE"

$Env:CEF_PATH = "$PWD\cef\$CEF_ARCHIVE"
