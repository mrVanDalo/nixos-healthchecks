#!/usr/bin/env bash

cargo run --bin script-exec -- ./examples/success.sh
cargo run --bin script-exec -- ./examples/failing.sh
cargo run --bin script-exec -- ./examples/doesnot-exist.sh

cargo run --bin script-exec -- ./examples/success.sh --emoji
cargo run --bin script-exec -- ./examples/failing.sh --emoji
cargo run --bin script-exec -- ./examples/doesnot-exist.sh --emoji

cargo run --bin script-exec -- ./examples/success.sh --emoji --time
cargo run --bin script-exec -- ./examples/failing.sh --emoji --time
cargo run --bin script-exec -- ./examples/doesnot-exist.sh --emoji --time
