#!/bin/bash

# CLI installation
cargo install diesel_cli

# DB migrations
diesel migration

# Build
cargo build --release

