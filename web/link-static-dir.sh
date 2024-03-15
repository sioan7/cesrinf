#!/bin/bash

# Execute this script from the "web" directory only.

unlink cesrinf-web-docker/static
unlink cesrinf-web-spin/static
unlink cesrinf-web-wasmer/static

cd cesrinf-web-docker
ln -svw  ../static static
cd ../cesrinf-web-spin
ln -svw  ../static static
cd ../cesrinf-web-wasmer
ln -svw  ../static static
