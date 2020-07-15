use crate::prelude::*;

pub mod prelude;
pub mod alg;


type TreeNode<T> = Option<Box<T>>;


fn main(){
    println!("{}",strStr(String::from("hello world"), String::from("world")));
    test_insert();
    test();
    quick();
    // let a = [1,5,9,2,4];
    // let mut b:Vec<i32> =Vec::new();
    // for i in 0..a.len() {
    //    b.push(a[i]);
    // }
    // for i in 0..a.len() {
    //     let c = b.get(i);
    //     if let Some(v) = c {
    //         println!("{:?}, {:?}", a[i], v);
    //     }
    // }
}


