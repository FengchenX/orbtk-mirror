
/*
15. 三数之和
给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。
注意：答案中不可以包含重复的三元组。
示例：
给定数组 nums = [-1, 0, 1, 2, -1, -4]，
满足要求的三元组集合为：
[
  [-1, 0, 1],
  [-1, -1, 2]
]
*/



pub fn test(){
    let mut nums =[-1, 0, 1, 2, -1, -4];
    let mut result: Vec<Vec<i32>> = Vec::new();
    three_sum(&mut nums,&mut result);
    println!("{:?}", result);
}

fn three_sum(nums: &mut [i32], result: &mut Vec<Vec<i32>>) {
    if nums.len() == 0 {
        return;
    }
    // 排序
    nums.sort();
    for k in 0..nums.len() {
        let v = nums[k];
        if k==nums.len()-2 {
            break;
        }
        if k>0 && nums[k] == nums[k-1] {
            continue
        }
        let (mut l, mut r) = (k+1, nums.len() -1);
        while l<r {
           let sum = v + nums[l] + nums[r];
            if sum<0{
               //l++
                l+=1;
            } else if sum>0 {
               r-=1;
            } else {
                let mut ret = Vec::new();
                ret.push(v);
                ret.push(nums[l]);
                ret.push(nums[r]);
               result.push(ret);
                while nums[l]==nums[l+1] && l < r {
                    l+=1;
                }
                while  nums[r] == nums[r-1] && l < r{
                   r+=1;
                }
                l+=1;
                r-=1;
            }
        }
    }
}