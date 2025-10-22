#!/bin/sh
mkdir -p ./target/debug
mkdir -p ./target/release

cp -r "$CEF_PATH/Resources/"* ./target/debug
cp -r "$CEF_PATH/Release/"* ./target/debug
cp -r "$CEF_PATH/Resources/"* ./target/release
cp -r "$CEF_PATH/Release/"* ./target/release

#chown root:root ./target/debug/chrome-sandbox
#chmod 4755 ./target/debug/chrome-sandbox
