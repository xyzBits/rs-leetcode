use crate::Solution;

impl Solution {
    pub fn shopping_offers(price: Vec<i32>, special: Vec<Vec<i32>>, mut needs: Vec<i32>) -> i32 {
        Self::back_tracing(&price, &special, &mut needs)
    }

    fn back_tracing(price: &Vec<i32>, special: &Vec<Vec<i32>>, needs: &mut Vec<i32>) -> i32 {
        // 1.
        let mut result = 0;
        for (&p, &count) in price.iter().zip(needs.iter()) {
            result += p * count;
        }

        // 2.
        let n = price.len();
        for vals in special.iter() {
            if !Self::check_638(price, vals, needs) {
                continue;
            }

            for i in 0..n {
                needs[i] -= vals[i];
            }

            result = i32::min(result, vals[n] + Self::back_tracing(price, special, needs));

            for i in 0..n {
                needs[i] += vals[i];
            }
        }

        // 3.
        result
    }

    // 大礼包中的物品数量必须 <= needs中的数量，并且大礼包的价格必须 < 单独买的价格
    fn check_638(price: &Vec<i32>, counts: &Vec<i32>, needs: &Vec<i32>) -> bool {
        let n = price.len();
        let mut sum = 0;

        for i in 0..n {
            if counts[i] > needs[i] {
                return false;
            }
            sum += price[i] * counts[i];
        }

        sum > counts[n]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test() {
        assert_eq!(
            Solution::shopping_offers(vec![2, 5], vec![vec![3, 0, 5], vec![1, 2, 10]], vec![3, 2],),
            14
        );

        assert_eq!(
            Solution::shopping_offers(
                vec![2, 3, 4],
                vec![vec![1, 1, 0, 4], vec![2, 2, 1, 9]],
                vec![1, 2, 1],
            ),
            11
        );
    }
}
