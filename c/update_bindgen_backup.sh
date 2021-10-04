#!/bin/sh
#
# This invokes the build script in a way that regenerates the bindgen file and places it in ./bindgen_backup.rs
# This file will be then be present in docs.rs' build environment.

env UPDATE_BINDGEN_BACKUP=1 cargo build