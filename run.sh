#!/bin/bash
clang -c -masm=intel entry.s -o entry.o
clang -Wno-implicit-function-declaration entry.o main.c
./a.out
