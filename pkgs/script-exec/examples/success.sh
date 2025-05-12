#!/usr/bin/env bash

# Generate a random number between 2 and 5 (inclusive) and sleep for that duration
sleep $(( ( RANDOM % 4 ) + 2 ))
echo "all good"
echo "this should never be printed"

exit 0