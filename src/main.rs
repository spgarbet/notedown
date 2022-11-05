use std::env;
use std::fs::File;
use std::path::Path;
use regex::Regex;
use std::io::{self, BufRead};
use chrono::NaiveDate; //, Datelike};
use glob::glob;
use serde::Serialize;
use std::hash;

#[macro_use]
extern crate enum_display_derive;
use std::fmt::Display;

  /////////////////////////////////////////////////////////////////////////////
 //
// Declare Structures
//
#[derive(PartialEq, Display, Serialize)]
enum Token 
{
    H, // Header
    S, // Secondary header
    L, // Long code
    C, // Code
    T, // Topic
    O, // Other
}

#[derive(Debug,Serialize)]
struct Topic 
{
  name:    String,
  entries: Vec<String>
}

#[derive(Debug,Serialize)]
struct Note
{
  title:   String,
  path:    Box<Path>,
  date:    NaiveDate,
  topics:  Vec<Topic>   
}

  /////////////////////////////////////////////////////////////////////////////
 //
// Parser
//
fn token(line: &str) -> Token
{
    let topic_re = Regex::new(r"^\p{L}+:").unwrap();

    if      line.starts_with("# ")   { Token::H }
    else if line.starts_with("##")   { Token::S }
    else if line.starts_with("````") { Token::L }
    else if line.starts_with("```")  { Token::C }
    else if topic_re.is_match(line)  { Token::T }
    else                             { Token::O }
}

fn tokenizer(state: Token, line: &str) -> (Token, Token)
{
  let token = token(line);

  if state == Token::O
  {
    if      token == Token::C { (Token::C, Token::C) }
    else if token == Token::L { (Token::L, Token::C) }
    else if token == Token::H { (Token::H, Token::H) }
    else                      { (Token::O, Token::O) }
  }
  else if state == Token::H
  {
    if      token == Token::C { (Token::C, Token::C) }
    else if token == Token::L { (Token::L, Token::C) }
    else if token == Token::S { (Token::O, Token::O) }
    else if token == Token::T { (Token::H, Token::T) }
    else if token == Token::H { (Token::H, Token::H) }
    else                      { (Token::H, Token::O) }
  }
  else if state == Token::C
  {
    if      token == Token::C { (Token::O, Token::C) }
    else                      { (Token::C, Token::C) }
  }
  else if state == Token::L
  {
    if      token == Token::L { (Token::O, Token::C) }
    else                      { (Token::L, Token::C) }
  }
  else
  { 
    panic!("Malformed parser table")
  }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;  
    Ok(io::BufReader::new(file).lines())
}

fn parse_topic(line: &str) -> Topic
{
    let mut iter = line.split(":");
    match iter.next()
    {
        None       => panic!("Malformed topic parse"),
        Some(name) => Topic {
                              name:    String::from(name.trim()),
                              entries: iter.next()
                                           .expect("Topic should have entries")
                                           .split(",")
                                           .map(|x|x.trim().to_string())
                                           .collect(),  
                            }
    }
}

fn parse(path: &Path, date: NaiveDate) -> Vec<Note>
{
    let mut notes : Vec<Note> = Vec::new();
    let mut state : Token = Token::O;

    match read_lines(path)
    { 
        Err(why)  => panic!("couldn't open {}: {}", path.display(), why),
        Ok(lines) => for line in lines
        {
            match line
            {
                Err(why) => panic!("read failure {}: {}", path.display(), why),
                Ok(ip)   => 
                {
                    let cur;
                    (state,  cur) = tokenizer(state, &ip);
                    // println!("{}", cur);
                    if cur == Token::H
                    {
                        notes.push(
                            Note {
                                title: String::from(&ip[2..]),
                                path:  Box::from(path.clone()), 
                                date:  date,
                                topics: Vec::new()
                            } 
                        )
                    }
                    else if cur == Token::T
                    {
                        if let Some(last) = notes.last_mut()
                        {
                            last.topics.push(parse_topic(&ip));
                        }
                    }
                }
            } 
        }
    }

    notes
}

fn extract_date(path: &str) -> NaiveDate
{
    let reg: Regex = Regex::new(r".*/([0-9]{4}/[0-9]{2}/[0-9]{2})/.*\.md$").unwrap();
    let date_str = reg.captures(path)
                      .ok_or(0)
                      .unwrap()
                      .get(1)
                      .map_or("", |m| m.as_str());

    NaiveDate::parse_from_str(date_str, "%Y/%m/%d").unwrap()
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    let mut stuff = Vec::new();

    for y in glob(format!("{}/[0-9][0-9][0-9][0-9]/[0-9][0-9]/[0-9][0-9]/*.md", &args[1]).as_str())
        .expect("Failed to read glob pattern")
    {
        if let Ok(x) = y
        {
            for z in parse(&x, extract_date(x.to_str().unwrap()))
            {
                stuff.push(z);
            }
        }
    }
    stuff.sort_by(|a,b| a.date.cmp(&b.date));
 
    let out = serde_json::to_string(&stuff).unwrap();
    println!("{}", out);    
}
 
