#!/usr/bin/env python3

import sys
import os
import argparse
import re
import code

quiet = False

four_num = re.compile("\d\d\d\d")
two_num  = re.compile("\d\d")

def progress(msg):
    if not quiet: 
        print(msg, end="")

def scan_file(file, date):
    con = open(file, 'r')
    lines = con.readlines()
    con.close()
    hdr1 = [i for i in range(0, len(lines)-1) if lines[i].startswith("# ")]
    ends = [i-1 for i in hdr1]
    return [[file,lines[hdr][2:-1],date] for hdr in hdr1]

def scan_nd(path, yyyy, mm, dd):
    path = os.path.join(path, yyyy, mm, dd)
    files =  [file for file in os.listdir(path) if file.endswith(".md")]
    results = [scan_file(os.path.join(path,file), "/".join([yyyy, mm, dd])) for file in files]  
    return([item for sublist in results for item in sublist])

def scan_dd(path, yyyy, mm):
    days = list(filter(two_num.fullmatch, os.listdir(os.path.join(path, yyyy, mm))))
    days.sort()
    results = [scan_nd(path, yyyy, mm, dd) for dd in days]
    results = [item for sublist in results for item in sublist]
    progress(" "+mm+": "+",".join(days)+f"---{len(results)} entries\n") 
    return(results)

def scan_mm(path, yyyy):
    progress("Found " + yyyy+"\n")
    months = list(filter(two_num.fullmatch, os.listdir(os.path.join(path, yyyy))))
    months.sort()
    results = [scan_dd(path, yyyy, mm) for mm in months] 
    return [item for sublist in results for item in sublist]

def scan(path):
    years = list(filter(four_num.fullmatch, os.listdir(path)))
    years.sort()
    results = [scan_mm(path, yyyy) for yyyy in years]
    return [item for sublist in results for item in sublist]

INVALID_PATH_MSG = "Error: Invalid path name. '%s' is not a valid path."

arg_parser = argparse.ArgumentParser(description = 'Notedown indexer.')

arg_parser.add_argument('path', metavar='path', type=str, help='file path for notedown directory')
arg_parser.add_argument('-q', "--quiet", metavar='quiet', action=argparse.BooleanOptionalAction, help="Suppress STDOUT progress")

args = arg_parser.parse_args()

if not os.path.isdir(args.path):
  print(INVALID_PATH_MSG%(args.path))

quiet = not (args.quiet == None)

progress(f"Starting Indexing at {args.path}\n")

index = scan(args.path)

print("\n")
print(index)
