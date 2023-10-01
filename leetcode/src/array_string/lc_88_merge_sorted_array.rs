struct Solution;

/// `nums1` & `nums2` sorted in non-decreasing order.
/// `m` & `n` represent length of `nums1` & `nums2` respectively.
/// Result should be stored in `nums1`.
/// `nums1` has length of `m` + `n`:
///     `m` elements should be merged.
///     `n` elements should be ignored and are 0.
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);
        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // Given
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;

        // When
        Solution::merge(&mut nums1, m, &mut nums2, n);

        // Then
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_2() {
        // Given
        let mut nums1 = vec![1];
        let m = 1;
        let mut nums2 = vec![];
        let n = 0;

        // When
        Solution::merge(&mut nums1, m, &mut nums2, n);

        // Then
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_3() {
        // Given
        let mut nums1 = vec![0];
        let m = 0;
        let mut nums2 = vec![1];
        let n = 1;

        // When
        Solution::merge(&mut nums1, m, &mut nums2, n);

        // Then
        assert_eq!(nums1, vec![1]);
    }
}
