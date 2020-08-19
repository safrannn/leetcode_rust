struct Solution;

impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut l: usize = 0;
        let mut r: usize = nums.len() - 1;
        let k: usize = nums.len() - k as usize;
        while l < r {
            let pivot: usize = Self::partition(&mut nums, l, r);
            // println!("{},{},{}",l,r,pivot);
            if pivot == k {
                break;
            } else if pivot < k {
                l = pivot + 1;
            } else {
                r = pivot - 1
            }
        }
        nums[k]
    }

    fn partition(nums: &mut Vec<i32>, l: usize, r: usize) -> usize {
        nums.swap((l + r) / 2, r);
        let mut j = l;
        let pivot_value = nums[r];
        for i in l..r {
            if nums[i] <= pivot_value {
                nums.swap(i, j);
                j += 1;
            }
        }
        nums.swap(j, r);
        j
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    assert_eq!(
        Solution::find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4),
        4
    );
}
