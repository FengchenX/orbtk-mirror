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


/*
36. 有效的数独
判断一个 9x9 的数独是否有效。只需要根据以下规则，验证已经填入的数字是否有效即可。
数字 1-9 在每一行只能出现一次。
数字 1-9 在每一列只能出现一次。
数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。

上图是一个部分填充的有效的数独。

数独部分空格内已填入了数字，空白格用 '.' 表示。

示例 1:

输入:
[
  ['5','3','.','.','7','.','.','.','.'],
  ['6','.','.','1','9','5','.','.','.'],
  ['.','9','8','.','.','.','.','6','.'],
  ['8','.','.','.','6','.','.','.','3'],
  ['4','.','.','8','.','3','.','.','1'],
  ['7','.','.','.','2','.','.','.','6'],
  ['.','6','.','.','.','.','2','8','.'],
  ['.','.','.','4','1','9','.','.','5'],
  ['.','.','.','.','8','.','.','7','9']
]
输出: true
示例 2:

输入:
[
  ['8','3','.','.','7','.','.','.','.'],
  ['6','.','.','1','9','5','.','.','.'],
  ['.','9','8','.','.','.','.','6','.'],
  ['8','.','.','.','6','.','.','.','3'],
  ['4','.','.','8','.','3','.','.','1'],
  ['7','.','.','.','2','.','.','.','6'],
  ['.','6','.','.','.','.','2','8','.'],
  ['.','.','.','4','1','9','.','.','5'],
  ['.','.','.','.','8','.','.','7','9']
]
输出: false
解释: 除了第一行的第一个数字从 5 改为 8 以外，空格内其他数字均与 示例1 相同。
     但由于位于左上角的 3x3 宫内有两个 8 存在, 因此这个数独是无效的。
说明:

一个有效的数独（部分已被填充）不一定是可解的。
只需要根据以上规则，验证已经填入的数字是否有效即可。
给定数独序列只包含数字 1-9 和字符 '.' 。
给定数独永远是 9x9 形式的。
*/
fn is_valid_sudoku(board: Vec<Vec<char>>) ->bool{
    let mut raw: Vec<Vec<i32>> = vec![vec![0; 9]; 9];
    let mut col: Vec<Vec<i32>> = vec![vec![0; 9]; 9];
    let mut block: Vec<Vec<i32>> = vec![vec![0; 9]; 9];
    let n: usize = 9;
    for i in 0..n {
        for j in 0..n {
            if board[i][j] == '.' { continue; }
            let number: usize = (board[i][j].to_digit(10).unwrap() - 1) as usize;
            let n_block: usize = (i / 3 * 3 + j / 3) as usize;
            raw[i][number] += 1;
            col[j][number] += 1;
            block[n_block][number] += 1;
            if raw[i][number] > 1 ||
                col[j][number] > 1 ||
                block[n_block][number] > 1 {
                return false;
            }
        }
    }
    true
}


pub fn test2() {
    let board = vec![
        vec!['8','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];

    let board2 = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9']
    ];

    // println!("{}", is_valid_sudoku(board));
    println!("{}", is_valid_sudoku2(board2))
}

fn is_valid_sudoku2(board: Vec<Vec<char>>) ->bool{
    let mut raw:Vec<HashMap<usize, usize>> = vec![HashMap::new();9];
    let mut col:Vec<HashMap<usize, usize>> = vec![HashMap::new();9];
    let mut block:Vec<HashMap<usize, usize>> = vec![HashMap::new();9];
    let n:usize = 9;
    for i in 0..n{
        for j in 0..n{
            if board[i][j] == '.' { continue; }
            let number = (board[i][j].to_digit(10).unwrap()-1) as usize;
            let count = raw[i].entry(number).or_insert(0);
            *count +=1;
            let x = col[j].entry(number).or_insert(0);
            *x+=1;
            let n_block = i / 3 * 3 + j / 3;
            let x1 = block[n_block].entry(number).or_insert(0);
            *x1+=1;
            if *raw[i].get(&number).unwrap() > 1 ||
                *col[j].get(&number).unwrap()>1 ||
                *block[n_block].get(&number).unwrap()>1 {
                return false
            }
        }
    }
    true
}

