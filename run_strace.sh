#!/bin/bash

strace -f -o trace.txt -s 999 cargo build