#!/usr/bin/env python3

import sys
import os
import argparse
import regex
import code

quiet = False

four_num = regex.compile("\d\d\d\d")
two_num  = regex.compile("\d\d")

def progress(msg):
    if not quiet: 
        print(msg, end="")

def token(line):
    if line.startswith("# "):
        return "H" # Header
    elif line.startswith("##"):
        return "S" # Secondary
    elif line.startswith("````"):
        return "L" # Long Code
    elif line.startswith("```"):
        return "C" # Code
    elif regex.match(r"\p{L}+:", line):
        return "T" # Topic
    else:
        return "O" # Other

# Blocks of code detection
def semantics(tokens):
    block = 'O'  # in an other block to start with
    def trans(token):
        nonlocal block
        if block == 'O':
            if token == 'C':
                block = 'C'
                return 'C'
            elif token == 'L':
                block = 'L'
                return 'C'
            elif token == 'H':
                block = 'H'
                return 'H'
            else:
                return 'O'
        elif block == 'H':
            if token == 'C':
                block = 'C'
                return 'C'
            elif token == 'L':
                block = 'L'
                return 'C'
            elif token == 'S':
                block = 'O'
                return 'O'
            elif (token == 'T' or token == 'H'):
                return token
            else:
                return 'O'
        elif block == 'C':
            if token == 'C':
                block = 'O'
            return 'C'
        elif block == 'L':
            if  token == 'L':
                block = 'O'
            return 'C'
        else:
            raise Exception("Parser Malformed Table")
    return list(map(trans, tokens)) 

def categorial(lines, topic):
    first =  [ lines[i].split(':') for i in topic ] 
    return [ [l[0], [k.strip() for k in l[1].split(',')] ] for l in first ]

def scan_file(file, date):
    con = open(file, 'r')
    lines = con.readlines()
    con.close()
    tokens = semantics([token(line) for line in lines])
    headers = list(filter(lambda x:tokens[x]=='H', range(0, len(tokens)-1)))
    topics  = list(filter(lambda x:tokens[x]=='T', range(0, len(tokens)-1)))
    split   = [list(filter(lambda x: x>i and (x < j), topics))
               for (i,j) in zip(headers, headers[1:]+[len(tokens)]) ]
    return [[file, lines[hdr][2:-1],date,categorial(lines, topic)] for (hdr,topic) in zip(headers,split)]

#return [[file,lines[hdr][2:-1],date] for hdr in hdr1]

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

#code.interact(local=locals())

progress(f"Starting Indexing at {args.path}\n")

index = scan(args.path)

print("\n")
print(index)
