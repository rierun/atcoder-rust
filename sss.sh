#! /bin/bash

cargo compete new $1
cd $1
code .
cargo build
