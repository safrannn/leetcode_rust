struct Solution;

impl Solution {
    pub fn count_good_rectangles(rectangles: Vec<Vec<i32>>) -> i32 {
        let mut max_square_side = 0;
        let mut max_square_count = 0;

        for v in rectangles {
            let side = v[0].min(v[1]);
            if side > max_square_side {
                max_square_side = side;
                max_square_count = 1;
            } else if side == max_square_side {
                max_square_count += 1;
            }
        }

        max_square_count
    }
}

use std::collections::HashMap;
impl Solution {
    pub fn tuple_same_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        let mut products: HashMap<i32, i32> = HashMap::new();
        for i in 0..n - 1 {
            for j in i + 1..n {
                *products.entry(nums[i] * nums[j]).or_default() += 1;
            }
        }

        for (_, v) in products {
            result += v * (v - 1) / 2;
        }
        result * 8
    }
}

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut result: i32 = 0;
        let n = matrix.len();
        let m = matrix[0].len();
        if n == 1 {
            return matrix[0].clone().into_iter().sum();
        }
        for j in 0..m {
            let mut ix = 0;
            let mut iy = 0;
            while iy < n {
                if matrix[iy][j] == 0 {
                    ix = iy + 1;
                } else {
                    let mut right_count = 0;
                    for j_test in j..m {
                        let mut all_one = true;
                        for i_test in ix..=iy {
                            if matrix[i_test][j_test] == 0 {
                                all_one = false;
                                break;
                            }
                        }
                        if all_one {
                            right_count += 1;
                        }
                    }
                    println!("{:?},{:?},{:?}", ix, iy, right_count);
                    result = result.max((iy - ix + 1) as i32 * right_count);
                }
                iy += 1;
            }
        }
        result
    }
}
