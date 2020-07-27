
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
                   row[j][n] = true;
                   col[j][n] = true;
                   col[i/3*3+j/3][n] = true;
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
        let n_block = i/3*3+j/3;
        for x in 0..9 {
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
