#!/usr/bin/env python3
import os
import sys

def greet(name):
    print(f"Hello, {name}!")

def main():
    greet("world")
    path = os.getcwd()
    print(f"Running from {path}")
    return 0

if __name__ == "__main__":
    sys.exit(main())
