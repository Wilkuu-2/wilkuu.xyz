#!/bin/sh
# Import all .env stuff 
set -e 

SERVER_NAME=$1 
RUST_RELEASE=$2

# Build in rust 
build_arg=$( [ -z $RUST_RELEASE ] && echo "" || echo "--release" )
cargo build --locked --bin=$SERVER_NAME $build_arg

#Copy the server file
TARGET_FOLDER=$([-z $RUST_RELEASE] && echo "debug" || echo "release" )
cp ./target/$TARGET_FOLDER/$SERVER_NAME /bin/$SERVER_NAME 
