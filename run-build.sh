#!/bin/sh
rustup target add wasm32-unknown-unknown
cd contracts/lease-management-system
cargo wasm