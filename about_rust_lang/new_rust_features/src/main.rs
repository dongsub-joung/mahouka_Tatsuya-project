mod eg_trait;
mod eg_generictraits;

use crate::eg_trait::*;
use crate::eg_generictraits::*;


static A: i32 =1; // "located in Rust's Data Segment" Google Gemini notice

fn get_one<T:eg_trait::Example>()-> i32{
    T::CONST_NO_DEFAULT
}

fn main(){
    // For Explicit Declaration, Its same thing parse::<usize>()
    let status_one= get_one::<eg_trait::EgStruct>();
}
