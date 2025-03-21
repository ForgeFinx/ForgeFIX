#!/bin/sh
xsltproc fields.xslt FIX42.xml > ../src/fix/fields/generated.rs
cd ..
cargo fmt
