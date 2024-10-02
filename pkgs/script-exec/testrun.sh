#!/usr/bin/env bash

cargo run --bin script-exec -- ./examples/success.sh --title "it should work"
cargo run --bin script-exec -- ./examples/success.sh
cargo run --bin script-exec -- ./examples/failing.sh --title "should fail"
cargo run --bin script-exec -- ./examples/failing.sh
cargo run --bin script-exec -- ./examples/doesnot-exist.sh --title "does not exist"
cargo run --bin script-exec -- ./examples/doesnot-exist.sh

cargo run --bin script-exec -- ./examples/success.sh --title "it should work" --emoji
cargo run --bin script-exec -- ./examples/success.sh --emoji
cargo run --bin script-exec -- ./examples/failing.sh --title "should fail" --emoji
cargo run --bin script-exec -- ./examples/failing.sh --emoji
cargo run --bin script-exec -- ./examples/doesnot-exist.sh --title "does not exist" --emoji
cargo run --bin script-exec -- ./examples/doesnot-exist.sh --emoji
