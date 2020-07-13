

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

fn merge_sort<T:PartialOrd>(arr: &[T]) -> &[T] {
    if arr.len()<2 {
        return arr;
    }
    let i = arr.len()/2;
    arr
}


pub fn test() {
    let mut a = [9,1,6,5,4,2];
    let b = a.len()-1;
    quick_sort(&mut a, 0, b);
    println!("{:?}",a);
}

