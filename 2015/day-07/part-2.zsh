#!/bin/zsh

{cat input.txt; (echo "\n" `cat input.txt | python part-1.py` "-> b")} | python3 part-1.py
