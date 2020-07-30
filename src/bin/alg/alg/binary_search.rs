
/*
必须符合以下三点才可以使用二分查找

数据单调（单调递增或者单调递减）
存在上下的分界点（可以得到mid中间值）
可以通过索引获取到值（数组）

解题模板
func Search(target,value){
  left,right := 0,len(value)-1 // 设置左右的双指针索引
  for left <= right { // 只要左比右边小于等于就OK
    mid = (left + right) / 2 // 获取到中间值
    if value[mid]== target { // 判断中间值是否等于目标
      return value[mid]
    }else if value[mid] > target{ // 看目标在哪个区域就把下次寻找的数据往哪个方向找。
      right = mid -1 // 注意如果是那种不能+1 就可以跨界的情况，right = mid即可
    }else {
      left = mid+1
    }
  }
}
 */



fn search<T:std::fmt::Debug + std::cmp::PartialOrd>(target:T,value: &[T]) ->Option<&T>{
    let (mut left,mut right)=(0,value.len()-1);
    while left<=right {
       let mid = (left+right)/2;
        if value[mid] == target {
            return Some(&value[mid]);
        }else if value[mid]>target {
            right = mid -1
        }else { left=mid+1 }
    }
    None
}

pub fn test(){
    let arr = [10,20,30,40];
    println!("**************{:?}",search(70,&arr));
}

/*
69. x 的平方根
实现 int sqrt(int x) 函数。

计算并返回 x 的平方根，其中 x 是非负整数。

由于返回类型是整数，结果只保留整数的部分，小数部分将被舍去。

示例 1:

输入: 4
输出: 2
示例 2:

输入: 8
输出: 2
说明: 8 的平方根是 2.82842...,
     由于返回类型是整数，小数部分将被舍去。
*/

fn my_sqrt(x: i32) -> i32 {
   if x<2 {
       return x;
   }
    let mut l = 0;
    let mut r = x;
    let mut res = 0;
    while l<=r {
        let mid = (l+r)/2;
        if x/mid > mid {
            l = mid+1;
            res = mid;
        } else if x/mid<mid  {
            r=mid-1;
        } else {
            return mid
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_sort() {
        assert_eq!(my_sqrt(4),2);
        assert_eq!(my_sqrt(8),2);
    }
}

