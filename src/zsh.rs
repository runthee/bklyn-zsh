#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate futures;

use std::vec::Vec;
use segments::*;
use self::futures::future::*;

const all_reset: &'static str = "\u{1b}[0m";
const fg_reset: &'static str = "\u{1b}[39m"; // or is it 38?
const bg_reset: &'static str = "\u{1b}[49m"; //  or is it 48?

// set foreground color
// TODO: cache values
fn fg(color: u32) -> String {
  let b = color & 0xff;
  let g = (color >> 8) & 0xff;
  let r = (color >> 16) & 0xff;
  format!("\u{1b}[38;2;{};{};{}m", r, g, b) 
}

// set background color
// TODO: cache values
fn bg(color: u32) -> String {
  let b = color & 0xff;
  let g = (color >> 8) & 0xff;
  let r = (color >> 16) & 0xff;
  format!("\u{1b}[48;2;{};{};{}m", r, g, b) 
}

// seps
fn sep_codes() -> [[&'static str; 4]; 4] {
  [
    // angles
    ["\u{e0b0}", "\u{e0b1}", "\u{e0b2}", "\u{e0b3}"],
    // curves
    ["\u{e0b4}", "\u{e0b5}", "\u{e0b6}", "\u{e0b7}"],
    // flames
    ["\u{e0c0}", "\u{e0c1}", "\u{e0c2}", "\u{e0c3}"],
    // digital
    ["\u{e0c4}", "\u{e0c5}", "\u{e0c6}", "\u{e0c7}"]
  ]
}

fn left_sep(result: &mut String, seps: [[&'static str; 4]; 4], lastBg: u32, currBg: u32) {
  if currBg == lastBg {
    //result.push_str(seps[1][1]);
  }
  else {
    result.push_str(&fg(lastBg));
    result.push_str(&bg(currBg));
    result.push_str(seps[1][0]);
    result.push_str(" ");
  }
}

// combine texts for left prompt
fn left_fold(texts: Vec<Vec<Part>>) -> String {
  let seps = sep_codes();
  let mut result = String::with_capacity(1000);
  let mut hasLast = false;
  let mut lastBg = 0;
  for text in texts {
    let currBg = match text.first() {
      Some(&Part::Bg(bg)) => bg,
      _ => panic!["Segment without bg??"]
    };
    if hasLast {
      left_sep(&mut result, seps, lastBg, currBg);
    }
    // is there a separater here
    for part in text {
      match part {
        Part::Text(string) => result.push_str(&string),
        Part::Fg(color) => result.push_str(&fg(color)), 
        Part::Bg(color) => result.push_str(&bg(color)),
        Part::FgReset{} => result.push_str(fg_reset), 
        Part::BgReset{} => result.push_str(bg_reset),
        Part::Ignore{} => {}
      };
    }
    hasLast = true;
    lastBg = currBg;
  };
  left_sep(&mut result, seps, lastBg, 0);
  result.push_str(all_reset);
  result
}

// generate zsh left prompt
pub fn left(segments: Vec<String>) { 
  let futs: Vec<BoxFuture<Vec<Part>, ()>> = segments.iter()
    .map(|string| string.as_str())
    .flat_map(|str| segment_of(str))
    .collect();
  let fut = join_all(futs)
    .then(|result| {
      match result {
        Ok(texts) => {
          let string = left_fold(texts);
          print!("{}", string.as_str());
          Ok(string)
        },
        Err(e) => Err(e)
      }
    });
  let _ = fut.wait();
  ()
}

// generate zsh right prompt
#[allow(unused_variables)]
pub fn right(segments: Vec<String>) {
  unimplemented!();
}


