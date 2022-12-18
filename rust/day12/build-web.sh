#!/usr/bin/env bash

wasm-pack build --target web
rm -rf web/pkg
mv pkg web/

