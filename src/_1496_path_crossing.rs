struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut position: (i32, i32) = (0, 0);
        let mut h_set: HashSet<(i32, i32)> = HashSet::new();
        h_set.insert((0, 0));
        for h_move in path.chars() {
            match h_move {
                'N' => position.1 -= 1,
                'S' => position.1 += 1,
                'E' => position.0 += 1,
                'W' => position.0 -= 1,
                _ => {}
            }
            if !h_set.insert(position) {
                return true;
            }
        }
        return false;
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_path_crossing("NNSWWEWSSESSWENNW".to_string()),
        true
    );
    assert_eq!(Solution::is_path_crossing("NES".to_string()), false);
    assert_eq!(Solution::is_path_crossing("NESWW".to_string()), true);
}
