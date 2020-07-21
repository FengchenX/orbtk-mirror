
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