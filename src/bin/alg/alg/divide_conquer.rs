

/*
递归模板
func recursion(level int,parm1,parm2...) parm{
  // terminator 终止条件
  if xxx {
    return XXX
  }

  // process 本层要处理的东西
  xxxxxxx

  // trill down 往下一层输入的信息
  recursion(level+1,parmNew1,parmNew2)

  // clear states  可选，递归返回的时候要处理的东西
  xxxxxxx
}

分治模板
func DivideConquer(problem,parm) parm  {
  // 判断问题是否结束
  if problem == nil {
    return result
  }
  // 开始准备数据，拆分数据
  data := prepareProblem(problem,parm)
  sonProblem := splitProblem(problem,data)
  // 开始执行往下一层的递归任务
  p1  := DivideConquer(sonProblem[0])
  p2  := DivideConquer(sonProblem[1])
  p3  := DivideConquer(sonProblem[2])
  p4  := DivideConquer(sonProblem[3])
}
// 开始合并这几个子问题
return merge(p1,p2,p3,p4)
*/

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


/*
50. Pow(x, n)
实现 pow(x, n) ，即计算 x 的 n 次幂函数。

示例 1:

输入: 2.00000, 10
输出: 1024.00000
示例 2:

输入: 2.10000, 3
输出: 9.26100
示例 3:

输入: 2.00000, -2
输出: 0.25000
解释: 2-2 = 1/22 = 1/4 = 0.25
说明:

-100.0 < x < 100.0
n 是 32 位有符号整数，其数值范围是 [−231, 231 − 1] 。
*/

fn my_pow(x: f64, n: i32) -> f64 {
    let (mut x, mut n) = if n<0{
        (1f64/x, -n)
    } else { (x,n) };
    let mut pow =1f64;
    if n==0{
        return 1f64;
    }
    if n%2!=0{
        return x*my_pow(x, n-1)
    }
    my_pow(x*x, n/2)
}

pub fn test2() {
    println!("{}", my_pow(2.00,10));
    println!("{}", my_pow(2.10,3));
    println!("{}", my_pow(2.00,-2));
}
