#!/bin/bash

# CLI installation
cargo install diesel_cli

# DB migrations
if [[ $DATABASE_URL == "" ]]; then
	echo "DATABASE_URL is not defined"
	exit 1
fi

diesel migration run

# Build
cargo build --release

