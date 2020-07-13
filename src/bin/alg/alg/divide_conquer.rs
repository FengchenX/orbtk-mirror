

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

fn merge_sort<T:PartialOrd+Copy>(arr: &[T]) -> &[T] {
    if arr.len()<2 {
        return arr;
    }
    let i = arr.len()/2;
    let mut left = merge_sort(&arr[0..i]);
    let mut right = merge_sort(&arr [i..]);
    merge(&mut left,&mut right)
}
fn merge<'a,T:PartialOrd+Copy>(left: &'a mut [T], right: &'a mut [T]) ->&'a [T]{
    let mut result = Vec::new();
    let (mut m,mut n)=(0,0);
    let (l,r)=(left.len(),right.len());
    while m<l&&n<r {
        if left[m]>right[n] {
           result.push(right[n]);
            n+=1;
            continue;
        }
        result.push(left[m]);
        m+=1;
    }
    result.append(&mut right[n..].to_vec());
    result.append(&mut left[m..].to_vec());
    result.as_slice()
}


pub fn test() {
    let mut a = [9,1,6,5,4,2];
    let b = a.len()-1;
    quick_sort(&mut a, 0, b);
    println!("{:?}",a);
    println!("{:?}", merge_sort(&mut a));
}

