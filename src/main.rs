#![feature(iter_arith)]
extern crate num;

pub mod common;
pub mod prob1;
pub mod prob2;
pub mod prob8;

use common::Problem;
use prob2::*;
use prob8::*;

fn main() {
  let a = find_answer(4);
  println!("{}", a);
}