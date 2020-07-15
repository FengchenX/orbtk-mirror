

use std::cmp::PartialOrd;

fn quick_sort<T:PartialOrd+Copy>(arr: &mut [T], start: usize, end:usize) {
    if start < end {
        let base = arr[start];
        let (mut left,mut right)=(start,end);
        while left<right {
            while left<right&&arr[right]>=base {
               right-=1;
            }
            if left<right{
                arr[left]=arr[right];
                left+=1;
            }
            while left < right && arr[left]<=base {
               left+=1;
            }
            if left<right{
                arr[right]=arr[left];
                right-=1;
            }
        }
        arr[left]=base;
        quick_sort(arr,start,left-1);
        quick_sort(arr, left+1, end);
    }
}

fn merge_sort<T:PartialOrd+Copy+std::fmt::Debug>(arr: &mut [T]) -> &mut [T] {
    if arr.len()<2 {
        return arr;
    }
    let i = arr.len()/2;
    let mut a1:Vec<T> = Vec::new();
    let mut a2:Vec<T> =Vec::new();
    for k in 0..i{
        a1.push(arr[k]);
    }
    for k in i..arr.len() {
        a2.push(arr[k]);
    }

    let mut left = merge_sort(a1.as_mut_slice());
    let mut right = merge_sort(a2.as_mut_slice());
    let mut result: Vec<T> = Vec::new();
    merge(&mut left,&mut right,&mut result);
    for k in 0..arr.len(){
        if let Some(v) = result.get(k){
            arr[k]=*v;
        }
    }

    arr
}

fn merge<T:PartialOrd+Copy+std::fmt::Debug>(left: &mut [T], right: &mut [T], result: &mut Vec<T>){
    let (mut m,mut n)=(0,0);
    let (l,r)=(left.len(),right.len());
    while m<l&&n<r {
        if left[m]>right[n] {
           result.push(right[n]);
            n=n+1;
        } else {
            result.push(left[m]);
            m=m+1;
        }
    }
    result.append(&mut right[n..].to_vec());
    result.append(&mut left[m..].to_vec());
}


pub fn test() {
    let mut a = [9,1,6,5,4,2];
    let b = a.len()-1;
    quick_sort(&mut a, 0, b);
    println!("{:?}",a);
    println!("#########{:?}", merge_sort(&mut a));
}

