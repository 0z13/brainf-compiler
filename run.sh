#!/bin/bash
clang -c entry.s -o entry.o
#idk use gcc?
clang -Wno-implicit-function-declaration entry.o main.c
./a.out
