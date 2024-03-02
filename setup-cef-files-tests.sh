#!/bin/sh

cp -r "$CEF_PATH/Resources/"* .
cp -r "$CEF_PATH/Release/"* .

sudo chown root:root ./chrome-sandbox
sudo chmod 4755 ./chrome-sandbox
