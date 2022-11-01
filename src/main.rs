use std::env;
use std::fs::File;
use std::path::Path;
use std::fmt;
use regex::Regex;
use std::io::{self, BufRead};
use chrono::{NaiveDate}; //, Datelike};

  /////////////////////////////////////////////////////////////////////////////
 //
// Declare Structures
//
#[derive(PartialEq)]
enum Token 
{
    H, // Header
    S, // Secondary header
    L, // Long code
    C, // Code
    T, // Topic
    O, // Other
}

impl fmt::Display for Token
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self
        {
            Token::H => write!(f, "H"),
            Token::S => write!(f, "S"),
            Token::L => write!(f, "L"),
            Token::C => write!(f, "C"),
            Token::T => write!(f, "T"),
            Token::O => write!(f, "O"),
        }
    }
}

#[derive(Debug)]
struct Topic 
{
  name:    String,
  entries: Vec<String>
}

#[derive(Debug)]
struct Note
{
  title:   String,
  path:    Box<Path>,
  // https://docs.rs/chrono/0.4.22/chrono/naive/struct.NaiveDate.html#calendar-date  
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
    //else if line.contains(":")       { Token::T }
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

fn parse(path: &Path, date: NaiveDate) -> Vec<Note>
{
    let notes : Vec<Note> = Vec::new();
    let mut state : Token = Token::O;
    let mut cur   : Token = Token::O;

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
                  (state,  cur) = tokenizer(state, &ip);
                  println!("{}", cur);
                }
            } 
        }
    }

    notes
}


fn main()
{
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let note1 = Note 
    {
        title:String::from("First Note"),
        path:Box::from(Path::new("README.md")),
        date:NaiveDate::from_ymd(2022, 10, 31),
        topics:Vec::new()
    };
    dbg!(note1);

    let stuff = parse(Path::new(&args[1]), NaiveDate::from_ymd(2022, 10, 31));
    dbg!(stuff);

}
 
