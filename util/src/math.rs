const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }
pub fn log_2(x: usize) -> usize {
    assert!(x > 0);
    num_bits::<usize>() as usize - x.leading_zeros() as usize - 1
}