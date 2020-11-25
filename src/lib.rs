mod list {
    // 整数のリストが与えられ、ベクタを使って
    // - mean(平均値)
    // - median(ソートされた時に真ん中に来る値)
    // - mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)
    // を返してください。

    use std::collections::HashMap;

    #[allow(dead_code)]
    fn mean(numbers: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for n in numbers {
            sum += n;
        }
        sum / numbers.len() as i32
    }

    #[allow(dead_code)]
    fn median(numbers: &Vec<i32>) -> i32 {
        let mut numbers = numbers.clone();
        numbers.sort();
        numbers[numbers.len() / 2]
    }

    #[allow(dead_code)]
    fn mode(numbers: &Vec<i32>) -> i32 {
        let mut counts = HashMap::new();
        for n in numbers {
            let count = counts.entry(n).or_insert(0);
            *count += 1;
        }

        let mut counts: Vec<_> = counts.iter().collect();
        counts.sort_by(|a, b| a.1.cmp(b.1));
        let (n, _) = counts[counts.len() - 1];
        *n.clone()
    }

    #[cfg(test)]
    mod tests {
        use crate::list::*;

        #[test]
        fn get_mean() {
            assert_eq!(mean(&vec![1, 10, 2, 5]), 4);
        }

        #[test]
        fn get_median() {
            assert_eq!(median(&vec![5, 2, 4, 1, 3]), 3);
        }

        #[test]
        fn get_nearly_median() {
            assert_eq!(median(&vec![2, 4, 1, 3]), 3);
        }

        #[test]
        fn get_mode() {
            assert_eq!(mode(&vec![2, 2, 1, 3, 2, 3]), 2);
        }
    }
}
