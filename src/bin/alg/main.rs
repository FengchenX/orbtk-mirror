use crate::prelude::*;
use std::borrow::BorrowMut;

pub mod prelude;
pub mod alg;


type TreeNode<T> = Option<Box<T>>;



fn main(){
    println!("{}",strStr(String::from("hello world"), String::from("world")));
    test_insert();
    test();
}


