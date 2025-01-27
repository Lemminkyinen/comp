pub use python_comp::iter_comp;
pub use python_comp::set_comp;
pub use python_comp::vec_comp;

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{self, *};
    use std::collections::HashSet;

    #[template]
    #[rstest]
    #[case(vec![1,2,3])]
    #[case(vec![1,2,3].into_iter())]
    #[case(1..=3)]
    fn iters(#[case] input: IntoIterator) {}

    #[apply(iters)]
    fn test_list_comprehension<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_vec = vec_comp![x * 2 for x in input];
        assert_eq!(my_vec, vec![2, 4, 6]);
    }

    #[apply(iters)]
    fn test_list_comprehension_2<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_vec = vec_comp![x + 2 for x in input];
        assert_eq!(my_vec, vec![3, 4, 5]);
    }

    #[apply(iters)]
    fn test_list_comprehension_3<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_vec = vec_comp![x for x in input];
        assert_eq!(my_vec, vec![1, 2, 3]);
    }

    #[apply(iters)]
    fn test_set_comprehension<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_set = set_comp![x * 2 for x in input];
        assert_eq!(my_set, HashSet::from([2, 4, 6]));
    }

    #[apply(iters)]
    fn test_set_comprehension_2<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_set = set_comp![x + 2 for x in input];
        assert_eq!(my_set, HashSet::from([3, 4, 5]));
    }

    #[apply(iters)]
    fn test_set_comprehension_3<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_set = set_comp![x for x in input];
        assert_eq!(my_set, HashSet::from([1, 2, 3]));
    }

    #[apply(iters)]
    fn test_iter_comprehension<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_iter = iter_comp![x for x in input];
        assert!(Iterator::eq(my_iter, vec![1, 2, 3]));
    }

    #[apply(iters)]
    fn test_if_comprehension_1<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_iter = iter_comp![x for x in input if x != 2];
        assert!(Iterator::eq(my_iter, vec![1, 3]));
    }

    #[apply(iters)]
    fn test_if_comprehension_2<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_iter = iter_comp![x for x in input if x != 2 && x != 3];
        assert!(Iterator::eq(my_iter, vec![1]));
    }

    #[apply(iters)]
    fn test_if_comprehension_3<T>(#[case] input: T)
    where
        T: IntoIterator<Item = i32>,
    {
        let my_iter = iter_comp![x for x in input if x != 2 if x != 1];
        assert!(Iterator::eq(my_iter, vec![3]));
    }

    #[rstest]
    fn test_nested_comprehension_1() {
        let input = vec![
            vec![vec![1, 2, 3], vec![1, 2, 3]],
            vec![vec![3, 2, 1], vec![3, 2, 1]],
        ];
        let my_iter = iter_comp![x for outer in input for inner in outer for x in inner];
        assert!(Iterator::eq(
            my_iter,
            vec![1, 2, 3, 1, 2, 3, 3, 2, 1, 3, 2, 1]
        ));
    }

    #[rstest]
    fn test_nested_comprehension_2() {
        let input = vec![
            vec![vec![1, 2, 3], vec![1, 2, 3]],
            vec![vec![3, 2, 1], vec![3, 2, 1]],
        ];
        let my_iter = iter_comp![x for outer in input for inner in outer for x in inner if x !=3];
        assert!(Iterator::eq(my_iter, vec![1, 2, 1, 2, 2, 1, 2, 1]));
    }

    #[rstest]
    fn test_nested_comprehension_3() {
        let input = vec![
            vec![vec![1, 2, 3], vec![1, 2, 3, 4]],
            vec![vec![3, 2, 1], vec![3, 2, 1, 0]],
        ];
        let my_iter =
            iter_comp![x for outer in input for inner in outer if inner.len() == 3 for x in inner];
        assert!(Iterator::eq(my_iter, vec![1, 2, 3, 3, 2, 1]));
    }
}
