use orbtk::prelude::HashMap;

fn two_sum(nums: &[i32], target: i32 , result: &mut Vec<usize>) {
   if nums.len() == 0 {
      return;
   }
   let mut m = HashMap::new();
   for i in 0..nums.len() {
       let a = (target-nums[i]) as usize ;
       if m.contains_key(&a) {
            result.push(i);
            match m.get(&a) {
                Some(x) => {result.push(*x);}
                _ => {}
            };
        }
        m.insert(nums[i] as usize,i);
   }
}

pub fn two_sum_test() {
   let nums = [9,3,5,6,2,7];
    let target = 8;
    let mut result: Vec<usize> = Vec::new();
    two_sum(&nums, target, &mut result);
    println!("two_sum: {:?}", result);
}