
/*
54. 螺旋矩阵
给定一个包含 m x n 个元素的矩阵（m 行, n 列），请按照顺时针螺旋顺序，返回矩阵中的所有元素。

示例 1:

输入:
[
 [ 1, 2, 3 ],
 [ 4, 5, 6 ],
 [ 7, 8, 9 ]
]
输出: [1,2,3,6,9,8,7,4,5]
示例 2:

输入:
[
  [1, 2, 3, 4],
  [5, 6, 7, 8],
  [9,10,11,12]
]
输出: [1,2,3,4,8,12,11,10,9,5,6,7]
*/

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut res = vec![];
    if matrix.is_empty() {
        return res;
    }
    if matrix[0].is_empty() {
        return res;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut left = 0;
    let mut right = cols;
    let mut top = 0;
    let mut down = rows;
    while res.len() < cols * rows {
        for j in left..right {
            res.push(matrix[top][j]);
        }
        top += 1;
        for i in top..down {
            res.push(matrix[i][right-1]);
        }
        right -= 1;
        if down <= top {
            break;
        }
        for j in (left..right).rev() {
            res.push(matrix[down-1][j]);
        }
        down -= 1;
        if left >= right {
            break;
        }
        for i in (top..down).rev() {
            res.push(matrix[i][left]);
        }
        left += 1;
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_spiral_order() {
        let a = vec![
            vec![ 1, 2, 3 ],
            vec![ 4, 5, 6 ],
            vec![ 7, 8, 9 ]
        ];
        assert_eq!(
            spiral_order(a),
            [1,2,3,6,9,8,7,4,5].to_vec()
        );
        let b = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9,10,11,12]
        ];
        assert_eq!(
            spiral_order(b),
            [1,2,3,4,8,12,11,10,9,5,6,7].to_vec()
        );
    }
}

