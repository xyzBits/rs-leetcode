use crate::Solution;

impl Solution {
    pub fn intersect(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        nums1.sort_unstable();
        nums2.sort_unstable();

        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut intersection = vec![0; len1.min(len2)];

        let mut index1 = 0;
        let mut index2 = 0;
        let mut index = 0;
        while index1 < len1 && index2 < len2 {
            if nums1[index1] < nums2[index2] {
                index1 += 1;
            } else if nums1[index1] > nums2[index2] {
                index2 += 1;
            } else {
                intersection[index] = nums1[index1];
                index1 += 1;
                index2 += 1;
                index += 1;
            }
        }

        // (&intersection[0..index])
        //     .into_iter()
        //     .map(|num| *num)
        //     .collect();

        intersection[..index].to_vec()
    }
}
