#!/usr/bin/env bash
set -e

# Check processor architecture to download the correction version
LINUX_ARCH=`uname -m`
if [ "$LINUX_ARCH" = "x86_64" ]; then
	CEF_ARCH="64"
elif [ "$LINUX_ARCH" = "arm" ]; then
	CEF_ARCH="arm"
elif [ "$LINUX_ARCH" = "aarch64" ]; then
	CEF_ARCH="arm64"
elif [ "$LINUX_ARCH" = "aarch64_be" ]; then
	CEF_ARCH="arm64"
elif [ "$LINUX_ARCH" = "armv8b" ]; then
	CEF_ARCH="arm64"
elif [ "$LINUX_ARCH" = "armv8l" ]; then
	CEF_ARCH="arm64"
else
	echo "Your system has a processor architecture that is unsupported by CEF: \"$LINUX_ARCH\""
	exit 1
fi

# Download CEF archive
CEF_ARCHIVE="cef_binary_141.0.11+g7e73ac4+chromium-141.0.7390.123_linux${CEF_ARCH}_minimal"
if [ ! -f /tmp/cef.tar.bz2 ]; then
	curl -o /tmp/cef.tar.bz2.part "https://cef-builds.spotifycdn.com/$CEF_ARCHIVE.tar.bz2"
	mv /tmp/cef.tar.bz2.part /tmp/cef.tar.bz2
fi
mkdir -p cef
tar -xvf /tmp/cef.tar.bz2 -C cef

export CEF_PATH="$PWD/cef/$CEF_ARCHIVE"

# Build CEF
(
	cd "$CEF_PATH"

	# Add compilation definitions to the top of the CMakeLists.txt file
	mv CMakeLists.txt CMakeLists.txt.old
	echo "add_compile_definitions(DCHECK_ALWAYS_ON=1)" > CMakeLists.txt
	cat CMakeLists.txt.old >> CMakeLists.txt

	# Build
	cmake .
	cmake --build .
)

echo "CEF is ready, please put the following line somewhere to set the environment variable, e.g. in .profile:"
echo "export CEF_PATH=\"$CEF_PATH\""
