pub trait CopyArrayConst<T, const N: usize, const NS: usize>
where
    T: Copy,
{
    fn copy_array_to<const S: usize>(&mut self, source: [T; NS]) -> &mut Self;
}

impl<T, const N: usize, const NS: usize> CopyArrayConst<T, N, NS> for [T; N]
where
    T: Copy,
{
    fn copy_array_to<const S: usize>(&mut self, source: [T; NS]) -> &mut Self {
        for (i, j) in (S..S + NS).zip(0..NS) {
            self[i] = source[j];
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::CopyArrayConst;

    #[test]
    fn it_works() {
        let mut arr = [0; 20];
        let arr1 = [1; 10];

        arr.copy_array_to::<1>(arr1);

        assert_eq!(
            arr,
            [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}

