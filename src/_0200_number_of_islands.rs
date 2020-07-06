struct Solution;

struct UnionFind {
    parents: Vec<usize>,
    groups: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        let parents = (0..size).collect();
        UnionFind {
            parents,
            groups: size,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.find(self.parents[x]);
            self.parents[x]
        } else {
            x
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let x_parent = self.find(x);
        let y_parent = self.find(y);
        if x_parent != y_parent {
            self.parents[x_parent] = y_parent;
            self.groups -= 1;
        }
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        if n == 0 {
            return 0;
        };
        let m = grid[0].len();
        let mut unionset = UnionFind::new(n * m + 1);

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' {
                    if j > 0 && grid[i][j - 1] == '1' {
                        unionset.union(m * i + (j - 1), m * i + j);
                    }
                    if i > 0 && grid[i - 1][j] == '1' {
                        unionset.union(m * (i - 1) + j, m * i + j);
                    }
                } else {
                    unionset.union(m * i + j, n * m);
                }
            }
        }

        (unionset.groups - 1) as i32
    }
}

#[test]
fn test() {
    let grid: Vec<Vec<char>> = vec_vec_char![
        ['1', '1', '1', '1', '0'],
        ['1', '1', '0', '1', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '0', '0', '0']
    ];
    assert_eq!(Solution::num_islands(grid), 1);
    let grid: Vec<Vec<char>> = vec_vec_char![
        ['1', '1', '0', '0', '0'],
        ['1', '1', '0', '0', '0'],
        ['0', '0', '1', '0', '0'],
        ['0', '0', '0', '1', '1']
    ];
    assert_eq!(Solution::num_islands(grid), 3);
}
