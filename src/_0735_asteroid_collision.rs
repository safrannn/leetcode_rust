struct Solution;

impl Solution {
    fn collide(asteroids: &mut Vec<i32>, asteroid: i32) {
        if let Some(&top) = asteroids.last() {
            if top < 0 {
                asteroids.push(asteroid);
            } else {
                if top + asteroid == 0 {
                    asteroids.pop();
                } else if top + asteroid < 0 {
                    asteroids.pop();
                    Self::collide(asteroids, asteroid);
                }
            }
        } else {
            asteroids.push(asteroid);
        }
    }
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        for v in asteroids {
            if v > 0 {
                result.push(v);
            } else {
                Self::collide(&mut result, v);
            }
        }
        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::asteroid_collision(vec![5, 10, -5]), vec![5, 10]);
    assert_eq!(Solution::asteroid_collision(vec![8, -8]), vec![]);
    assert_eq!(Solution::asteroid_collision(vec![10, 2, -5]), vec![10]);
    assert_eq!(
        Solution::asteroid_collision(vec![-2, -1, 1, 2]),
        vec![-2, -1, 1, 2]
    );
}
