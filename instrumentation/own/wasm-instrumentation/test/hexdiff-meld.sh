#!/bin/bash
# uses bash process substitution
meld <(hexdump -C $1) <(hexdump -C $2)
