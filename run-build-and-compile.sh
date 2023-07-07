#!/bin/sh
cargo build
docker run --rm -v "$(pwd)":/code -v "$(basename "$(pwd)")_cache":/code/target -v registry_cache:/usr/local/cargo/registry cosmwasm/workspace-optimizer:0.12.10