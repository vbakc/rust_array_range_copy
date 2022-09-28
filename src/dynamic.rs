pub trait CopyArray<T, const N: usize, const NS: usize>
where
    T: Copy,
{
    fn copy_from_array(&mut self, start_idx: usize, source: [T; NS]) -> &mut Self;
}

impl<T, const N: usize, const NS: usize> CopyArray<T, N, NS> for [T; N]
where
    T: Copy,
{
    fn copy_from_array(&mut self, start_idx: usize, source: [T; NS]) -> &mut Self {
        for (i, j) in (start_idx..start_idx + NS).zip(0..NS) {
            self[i] = source[j];
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::CopyArray;

    #[test]
    fn it_works() {
        let mut arr = [0; 20];
        let arr1 = [1; 10];

        arr.copy_from_array(1, arr1);

        assert_eq!(
            arr,
            [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
