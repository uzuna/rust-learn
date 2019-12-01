use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
  // 疑似乱数生成
  let mut rng = Pcg64Mcg::from_seed([0; 16]);

  // usgin `sample_iter` = infinity random generate iterator
  // `take(n)` is limit collect count
  // `collect()` transform iter -> Vec
  rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
  x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
  x.windows(2).all(|pair| pair[0] >= pair[1])
}
