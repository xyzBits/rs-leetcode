use crate::Solution;

impl Solution {
    pub fn merge_1(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = 0usize;
        while i != n as usize {
            nums1[m as usize + i] = nums2[i];
            i += 1;
        }

        nums1.sort_unstable();
    }

    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m = m as usize;
        let n = n as usize;
        let (mut p1, mut p2) = (0, 0);
        let mut sorted = vec![0; m + n];
        let mut cur = 0;

        while p1 < m || p2 < n {
            if p1 == m {
                cur = nums2[p2];
                p2 += 1;
            } else if p2 == n {
                cur = nums1[p1];
                p1 += 1;
            } else if nums1[p1] < nums2[p2] {
                cur = nums1[p1];
                p1 += 1;
            } else {
                cur = nums2[p2];
                p2 += 1;
            }
            sorted[p1 + p2 - 1] = cur;
        }

        let mut i = 0;
        while i != m + n {
            nums1[i] = sorted[i];
            i += 1;
        }
    }
}
