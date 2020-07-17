
/*
给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串，判断字符串是否有效。

有效字符串需满足：

左括号必须用相同类型的右括号闭合。
左括号必须以正确的顺序闭合。
注意空字符串可被认为是有效字符串。

示例 1:

输入: "()"
输出: true
示例 2:

输入: "()[]{}"
输出: true
示例 3:

输入: "(]"
输出: false
示例 4:

输入: "([)]"
输出: false
示例 5:

输入: "{[]}"
输出: true
*/

use orbtk::prelude::HashMap;

fn is_vaild(s: String) ->bool{
    let mut m = HashMap::new(); //辅助map
    m.insert('(',')');
    m.insert('{','}');
    m.insert('[', ']');
    let mut ret = Vec::new();
    for char in s.chars() {
        if m.contains_key(&char) {
            ret.push(char);
        } else {
            if ret.len() == 0 {
                return false;
            }
            let top = ret.pop();
            if let Some(c) = top {
                if char != m[&c] {
                   return false
                }
            }
        }
    }
    true
}

pub fn test() {
    println!("{}", is_vaild(String::from("()")));
    println!("{}", is_vaild(String::from("()[]{}")));
    println!("{}", is_vaild(String::from("(]")));
    println!("{}", is_vaild(String::from("([)]")));
    println!("{}", is_vaild(String::from("{[]}")));
}
