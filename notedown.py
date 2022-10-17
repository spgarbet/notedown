#!/usr/bin/env python3

import sys
import os
import argparse
import re

quiet = False

four_num = re.compile("\d\d\d\d")
two_num  = re.compile("\d\d")
md_re    = re.compile(".*\.md")

def progress(msg):
    if not quiet: 
        print(msg, end="")

def scan_nd(path, yyyy, mm, dd):
    path = os.path.join(path, yyyy, mm, dd)
    list(filter(md_re.fullmatch, os.listdir(path)))

def scan_dd(path, yyyy, mm):
    progress(" "+mm) 
    days = list(filter(two_num.fullmatch, os.listdir(os.path.join(path, yyyy, mm))))
    [scan_nd(path, yyyy, mm, dd) for dd in days]

def scan_mm(path, yyyy):
    progress("Found " + yyyy + "\n")
    months = list(filter(two_num.fullmatch, os.listdir(os.path.join(path, yyyy))))
    [scan_dd(path, yyyy, mm) for mm in months] 

def scan(path):
    years = list(filter(four_num.fullmatch, os.listdir(path)))
    results = [scan_mm(path, yyyy) for yyyy in years]

INVALID_PATH_MSG = "Error: Invalid path name. '%s' is not a valid path."

arg_parser = argparse.ArgumentParser(description = 'Notedown indexer.')

arg_parser.add_argument('path', metavar='path', type=str, help='file path for notedown directory')
arg_parser.add_argument('-q', "--quiet", metavar='quiet', action=argparse.BooleanOptionalAction, help="Suppress STDOUT progress")

args = arg_parser.parse_args()

if not os.path.isdir(args.path):
  print(INVALID_PATH_MSG%(args.path))

quiet = args.quiet

progress(f"Starting Indexing at {args.path}\n")

index = scan(args.path)

print(index)
