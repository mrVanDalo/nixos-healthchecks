#!/usr/bin/env bash

cargo run --bin script-exec -- --emoji --time \
./examples/success-a.sh \
./examples/success-b.sh \
./examples/success-c.sh \
./examples/doesnot-exist.sh \
./examples/success-d.sh \
./examples/success-e.sh \
./examples/doesnot-exist.sh \
./examples/success-f.sh \
./examples/failing-a.sh \
./examples/doesnot-exist.sh \
./examples/failing-b.sh \
./examples/failing-c.sh \
./examples/failing-d.sh \
./examples/doesnot-exist.sh \
./examples/failing-e.sh \
./examples/failing-f.sh \
./examples/doesnot-exist.sh \
