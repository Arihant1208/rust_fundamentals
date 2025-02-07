use std::io;

use generics::genreicsImpl;
use hashmap::{hashmaps, wordCount};
use leetcode::Tuple_1726::solve;
use rand::Rng;
use crate::garden::vegitables::Greenes;
use crate::inital_stuff::inital_stuff;
use crate::vectors::vectors;

pub mod inital_stuff;
pub mod guess_game;
pub mod garden;
pub mod vectors;
pub mod hashmap;
pub mod leetcode;
pub mod generics;

fn main() {
    // let result = first_word();
    // println!("ans is  = {result}");
    // let str = String::from("hello world");
    // let res = first_word_2(&str);
    // println!("{}", &str[..res]);
    // println!("{res}");

    
    // let x = 10;
    // let y = x;
    // println!("{x}");
    // println!("{y}");

    // let str1 = String::from("hello world");
    // let str2 = &str1;
    // println!("{str1}");
    // println!("{str2}");

    // enumTes();

    // modularity();
    // garden::print_hey();
    
    // vectors();

    // let hello = "Здравствуйте";
    // let answer = &hello[0];
    // println!("{answer}")

    // hashmaps();
    // wordCount();

    let ans = solve();
    println!("{ans}");

    // genreicsImpl();
}

fn modularity () {
    let beans = Greenes {} ;
    println!("heyyyyy eat {beans:?}");
}