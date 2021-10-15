#!/bin/sh

cp -r "$CEF_PATH/Resources/"* .
cp -r "$CEF_PATH/Release/"* .

echo The script needs root permissions to change permissions. >&2
sudo chown root:root ./chrome-sandbox
sudo chmod 4755 ./chrome-sandbox
