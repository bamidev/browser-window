#!/bin/sh

mkdir -p ./target/debug


cp -r "$CEF_PATH/Resources/"* ./target/debug
cp -r "$CEF_PATH/Release/"* ./target/debug

echo The script needs root permissions to change permissions. >&2
sudo chown root:root ./target/debug/chrome-sandbox
sudo chmod 4755 ./target/debug/chrome-sandbox
