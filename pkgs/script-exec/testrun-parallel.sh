#!/usr/bin/env bash

cargo run --bin script-exec -- --emoji --time \
a=./examples/success-a.sh \
b=./examples/success-b.sh \
c=./examples/success-c.sh \
d=./examples/doesnot-exist.sh \
e=./examples/success-d.sh \
f=./examples/success-e.sh \
g=./examples/doesnot-exist.sh \
h=./examples/success-f.sh \
i=./examples/failing-a.sh \
j=./examples/doesnot-exist.sh \
k=./examples/failing-b.sh \
l=./examples/failing-c.sh \
m=./examples/failing-d.sh \
n=./examples/doesnot-exist.sh \
i=./examples/failing-e.sh \
o=./examples/failing-f.sh \
p=./examples/doesnot-exist.sh \
