#[allow(dead_code)]
pub fn checked_add(nums: &[u32]) -> Option<u32> {
    let mut max: u32 = u32::MAX;
    let mut sum: u32 = 0;

    for num in nums {
        if max < *num {
            return None
        }
        sum += *num;
        max -= *num;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checked_add_success() {
        let nums = [1, 2, 3, 4, 5];
        assert_eq!(checked_add(&nums), Some(15));
    }

    #[test]
    fn test_checked_add_overflow() {
        let nums = [u32::MAX, 2];
        assert_eq!(checked_add(&nums), None);
    }
}