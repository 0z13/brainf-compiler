#!/bin/bash
gcc -c entry.s -o entry.o
gcc entry.o main.c
./a.out
