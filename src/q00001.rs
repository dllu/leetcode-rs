use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut mapping = HashMap::new();
    for (i, x) in nums.iter().enumerate() {
        mapping.insert(x, i);
    }
    for (i, x) in nums.iter().enumerate() {
        if let Some(&j) = mapping.get(&(target - x)) {
            if i != j {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn q00001() {
        assert_eq!(super::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(super::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(super::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
