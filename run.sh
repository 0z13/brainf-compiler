#!/bin/bash
gcc -c entry.s -o entry.o
gcc -Wno-implicit-function-declaration entry.o main.c
./a.out
