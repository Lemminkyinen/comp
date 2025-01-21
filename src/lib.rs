pub use python_comp::list_comp;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{self, *};

    #[template]
    #[rstest]
    #[case(vec![1,2,3])]
    #[case(vec![1,2,3].into_iter())]
    #[case(1..=3)]
    fn iters(#[case] input: T)
    where
        T: IntoIterator,
        T: IntoIterator::Item<i32>,
    {
    }

    #[apply(iters)]
    fn test_list_comprehension<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_vec = list_comp![x * 2 for x in input];
        assert_eq!(my_vec, vec![2, 4, 6]);
    }

    #[apply(iters)]
    fn test_list_comprehension_2<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_vec = list_comp![x + 2 for x in input];
        assert_eq!(my_vec, vec![3, 4, 5]);
    }

    #[apply(iters)]
    fn test_list_comprehension_3<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_vec = list_comp![x for x in input];
        assert_eq!(my_vec, vec![1, 2, 3]);
    }
}
