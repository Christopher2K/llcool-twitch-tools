#!/bin/bash
if [[ $DATABASE_URL == "" ]]; then
	echo "DATABASE_URL is not defined"
	exit 1
fi

diesel migration run

cargo run --release

