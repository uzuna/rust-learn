pub fn inspection_positive(tpf: &f32, fpf: &f32, pir: &f32) -> f32 {
  // tpf: true_positive_fraction: 感度
  // fpf: false_positive_fraction: 偽陽性
  // pir: pre_incident_rate: 事前確率
  // P(H1|D)=P(D|H1)P(H1)/(P(D|H1)P(H1)+P(D|H2)P(H2))
  (tpf * pir) / ((tpf * pir) + (fpf * (1.0 - pir)))
}

mod tests {
  use crate::inspection_positive;
  #[test]
  fn test_inspection_positive() {
    let mut r = 0.03;
    for i in 0..3 {
      r = inspection_positive(&0.98, &0.05, &r);
      println!("{}: r={}", i, r);
    }
  }
}
