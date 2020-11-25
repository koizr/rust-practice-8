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

    #[cfg(test)]
    mod tests {
        use crate::list::mean;

        #[test]
        fn get_mean() {
            assert_eq!(mean(&vec![1, 10, 2, 5]), 4);
        }
    }
}
