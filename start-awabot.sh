#!/bin/sh

set -a
. ./.env
set +a

cargo run --release
