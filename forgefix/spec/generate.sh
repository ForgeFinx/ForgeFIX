#!/bin/sh
xsltproc fields.xslt FIX42.xml > ../src/fix/generated/fields.rs
cd ..
cargo fmt
