
/*
result = []
def backtrack(路径, 选择列表):
    if 满足结束条件:
        result.add(路径)
        return

    for 选择 in 选择列表:
        做选择
        backtrack(路径, 选择列表)
        撤销选择
*/

/*
22. 括号生成
数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
示例：
输入：n = 3
输出：[
       "((()))",
       "(()())",
       "(())()",
       "()(())",
       "()()()"
     ]
*/

use orbtk::prelude::HashMap;
use std::fs::copy;

fn generate_parenthesis(n: i32) ->Vec<String> {
    let mut result = vec![];
    back_trace(String::new(), n as usize, 0,0, &mut result);
    result
}

fn back_trace(s: String, n: usize, right: usize, left: usize, result: &mut Vec<String>) {
    if s.len() == 2*n{
        result.push(s);
        return;
    }
    if left < n { // 此处就是使用的剪枝
        let mut s = s.to_owned();
        s.push('(');
        back_trace(s, n, right, left+1, result);
    }
    if left>0 && left > right && right<n { // 此处就是使用的剪枝 因为不是所有的条件都去递归，而是又选择性质的去递归，这种又选择的方式就是剪枝
        let mut s = s.to_owned();
        s.push(')');
        back_trace(s, n, right+1, left, result);
    }
}

pub fn test() {
    println!("{:?}", generate_parenthesis(3));
}


/*
37. 解数独
编写一个程序，通过已填充的空格来解决数独问题。

一个数独的解法需遵循如下规则：

数字 1-9 在每一行只能出现一次。
数字 1-9 在每一列只能出现一次。
数字 1-9 在每一个以粗实线分隔的 3x3 宫内只能出现一次。
空白格用 '.' 表示。

一个数独。
答案被标成红色。
Note:
给定的数独序列只包含数字 1-9 和字符 '.' 。
你可以假设给定的数独只有唯一解。
给定数独永远是 9x9 形式的。
*/

fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut row = vec![vec![false;9];9];
    let mut col = vec![vec![false;9];9];
    let mut block = vec![vec![false;9];9];
    let mut rest = vec![];
    for i in 0..9{
        for j in 0..9{
           match board[i][j] {
               '.' => rest.push((i,j)),
               _ =>{
                   let n = (board[i][j] as u8 - b'1') as usize;
                   row[i][n] = true;
                   col[j][n] = true;
                   block[i/3*3+j/3][n] = true;
               }
           }
        }
    }
    dfs(board, &rest, &mut row, &mut col, &mut block);
}

fn dfs(
    board: &mut Vec<Vec<char>>,
    rest: &[(usize,usize)],
    row: &mut Vec<Vec<bool>>,
    col: &mut Vec<Vec<bool>>,
    block: &mut Vec<Vec<bool>>
) -> bool {
    if let Some((i,j)) = rest.first() {
        let (i, j) = (*i,*j);
        for x in 0..9 {
            let n_block = i/3*3+j/3;
            if !row[i][x] && !col[j][x]&&!block[n_block][x] {
                row[i][x] = true;
                col[j][x] = true;
                block[n_block][x]=true;

                board[i][j] = (x as u8 + b'1') as char;
                if dfs(board, &rest[1..], row, col, block) {
                    return true;
                }
                row[i][x] = false;
                col[j][x] = false;
                block[n_block][x] = false;
            }
        }
        false
    } else {
        true
    }
}


pub fn  test2() {
    let mut board2 = vec![
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

    solve_sudoku(&mut board2);

    println!("{:?}", board2);
}


/*
51. N皇后
n 皇后问题研究的是如何将 n 个皇后放置在 n×n 的棋盘上，并且使皇后彼此之间不能相互攻击。
上图为 8 皇后问题的一种解法。
给定一个整数 n，返回所有不同的 n 皇后问题的解决方案。
每一种解法包含一个明确的 n 皇后问题的棋子放置方案，该方案中 'Q' 和 '.' 分别代表了皇后和空位。
示例:
输入: 4
输出: [
 [".Q..",  // 解法 1
  "...Q",
  "Q...",
  "..Q."],

 ["..Q.",  // 解法 2
  "Q...",
  "...Q",
  ".Q.."]
]
解释: 4 皇后问题存在两个不同的解法。
提示：
皇后，是国际象棋中的棋子，意味着国王的妻子。皇后只做一件事，那就是“吃子”。当她遇见可以吃的棋子时，就迅速冲上去吃掉棋子。当然，她横、竖、斜都可走一到七步，可进可退。（引用自 百度百科 - 皇后 ）
*/


/*
queens(0) = [[]]
queens(n) = [ [new_queen, ...sol] | sol in queens(n-1),
                                    new_queen in [0..size],
                                    check(new_queen, sol)]
*/
fn solve_n_queens(n: i32) -> Vec<Vec<Vec<char>>> {
    let n = n as usize;
    let mut mPie = HashMap::new();
    let mut mNa = HashMap::new();
    let mut mCol = HashMap::new();
    let mut result =  vec![];
    let re = vec![];
    mo(&mut mPie, &mut mNa, &mut mCol, &mut result, re, n, 0);
    result
}

fn mo(
    mPie: &mut HashMap<i32, usize>,
    mNa: &mut HashMap<i32, usize>,
    mCol: &mut HashMap<i32,usize>,
    result: &mut Vec<Vec<Vec<char>>>,
    re: Vec<Vec<char>>,
    n: usize,
    row: usize
) {
    let row = row as i32;
    let n = n as i32;
    if row>n-1 {
        result.push(re);
        return
    }
    for i in 0..n{
        if mPie.contains_key(&(row-i)) && mPie[&(row-i)] !=0 {
            continue;
        }
        if mNa.contains_key(&(row+i)) && mNa[&(row+i)] !=0 {
            continue;
        }
        if mCol.contains_key(&i) && mCol[&i] !=0 {
            continue;
        }
        let x = mPie.entry(row - i).or_insert(0);
        *x +=1;
        let x1 = mNa.entry(row + i).or_insert(0);
        *x1+=1;
        let x2 = mCol.entry(i).or_insert(0);
        *x2 +=1;
        let mut b = re.clone();
        b.push(ddtt(i as usize,n as usize));
        mo(mPie,mNa,mCol,result, b,n as usize,(row+1) as usize);
        mPie.insert(row-i,0);
        mNa.insert(row+i,0);
        mCol.insert(i,0);
    }
}

fn ddtt(i:usize,n:usize) ->Vec<char> {
    let mut res = vec![];
    for j in 0..n{
        if j != i {
            res.push('.')
        } else {
            res.push('Q')
        }
    }
    res
}

pub fn test3(){
    println!("{:?}",solve_n_queens(4));
}

