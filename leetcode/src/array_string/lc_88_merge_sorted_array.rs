/// # Merge Sorted Array
///
/// ## Problem
///
/// - `nums1` & `nums2` sorted in non-decreasing order.
/// - `m` & `n` represent length of `nums1` & `nums2` respectively.
/// - Result should be stored in `nums1`.
/// - `nums1` has length of `m` + `n`:
///     `m` elements should be merged.
///     `n` elements should be ignored and are 0.
///
/// ## Explanation
///
/// - Fill `nums1` from the end with the largest candidate from `nums1` or `nums2`:
///     - Candidate is current selection from the right of each array.
///     - Move selection 1 to the left each time a candidate is selected.
///     - Continue until all values from `nums2` are accounted for.
/// - `nums1` & `nums2` are already sorted so:
///     - Once `nums2` are dealt with, i.e: `n` = 0, rest of `nums1` sorted.
///     - No need to iterate through rest of `nums1`.
///
/// ## Example
///
/// - Given:
///     - nums1 = [1, 2, 3, 0, 0, 0]
///     - nums2 = [2, 5, 6]
/// - m = 3, n = 3:
///     - nums1[m - 1] < nums2[n - 1]
///     - nums1[m + n - 1] = nums2[n - 1]:
///         - nums1 = [1, 2, 3, 0, 0, 6]
///         - nums2 = [2, 5, 6]
///     - n -= 1
/// - m = 3, n = 2:
///     - nums1[m - 1] < nums2[n - 1]
///     - nums1[m + n - 1] = nums2[n - 1]:
///         - nums1 = [1, 2, 3, 0, 5, 6]
///         - nums2 = [2, 5, 6]
///     - n -= 1
/// - m = 3, n = 1:
///     - nums1[m - 1] > nums2[n - 1]
///     - nums1[m + n - 1] = nums1[m - 1]:
///         - nums1 = [1, 2, 3, 3, 5, 6]
///         - nums2 = [2, 5, 6]
///     - m -= 1
/// - m = 2, n = 1:
///     - nums1[m - 1] == nums2[n - 1]
///         - In the event of a tie, take value from nums2.
///     - nums1[m + n - 1] = nums2[n - 1]:
///         - nums1 = [1, 2, 2, 3, 5, 6]
///         - nums2 = [2, 5, 6]
///     - n -= 1
/// - m = 2, n = 0:
///     - END
/// - Result: [1, 2, 2, 3, 5, 6]
pub struct Solution;

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
