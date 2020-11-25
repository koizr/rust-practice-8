mod list {
    // 整数のリストが与えられ、ベクタを使って
    // - mean(平均値)
    // - median(ソートされた時に真ん中に来る値)
    // - mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)
    // を返してください。

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
    }
}
