#!/bin/sh

mkdir -p ./target/debug


cp -r "$CEF_PATH/Resources/"* ./target/debug
cp -r "$CEF_PATH/Release/"* ./target/debug


chown root:root ./target/debug/chrome-sandbox
chmod 4755 ./target/debug/chrome-sandbox
