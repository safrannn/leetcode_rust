struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut s_to_t = vec![None; 256];
        let mut t_seen = vec![false; 256];

        for i in 0..s.len() {
            if s_to_t[s[i] as usize].is_none() {
                if t_seen[t[i] as usize] {
                    return false;
                } else {
                    t_seen[t[i] as usize] = true;
                }
                s_to_t[s[i] as usize] = Some(t[i]);
            } else if s_to_t[s[i] as usize] != Some(t[i]) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_isomorphic("egg".to_string(), "add".to_string()),
        true
    );
    assert_eq!(
        Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
        false
    );
    assert_eq!(
        Solution::is_isomorphic("paper".to_string(), "title".to_string()),
        true
    );
}
