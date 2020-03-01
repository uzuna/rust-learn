pub fn inspection_positive(tpf: &f32, fpf: &f32, pir: &f32) -> f32 {
  // tpf: true_positive_fraction: 感度
  // fpf: false_positive_fraction: 偽陽性
  // pir: pre_incident_rate: 事前確率
  // P(H1|D)=P(D|H1)P(H1)/(P(D|H1)P(H1)+P(D|H2)P(H2))
  (tpf * pir) / ((tpf * pir) + (fpf * (1.0 - pir)))
}

pub fn inspection_false_positive(tpf: &f32, fpf: &f32, pir: &f32) -> f32 {
  // tpf: true_positive_fraction: 感度
  // fpf: false_positive_fraction: 偽陽性
  // pir: pre_incident_rate: 事前確率
  // P(H2|D)=P(D|H2)P(H2)|P(D|H1)P(H1)+P(D|H2)P(H2)
  (fpf * pir) / ((tpf * (1.0 - pir)) + (fpf * pir))
}

pub fn inspection_negative(tpf: &f32, fpf: &f32, pir: &f32) -> f32 {
  // tpf: true_positive_fraction: 感度
  // fpf: false_positive_fraction: 偽陽性
  // pir: pre_incident_rate: 事前確率
  // P(H2|!D)=P(!D|H2)P(H2)/(P(!D|H1)P(H1)+P(!D|H2)P(H2))
  let nd = 1.0 - tpf;
  let fff = 1.0 - fpf;
  (fff * pir) / ((fff * pir) + (nd * (1.0 - pir)))
}

mod tests {
  use crate::*;
  #[test]
  fn test_inspection() {
    // P(H1)罹患している確率は0.03
    // 罹患していて、検査でn買い続けてpositiveとなる確率=H1である確率
    let mut r = 0.03;
    for i in 0..3 {
      r = inspection_positive(&0.98, &0.05, &r);
      println!("{}: r={}", i, r);
    }
    // P(H2)罹患していない確率は1-0.03
    // 罹患してなくて、検査がNegativeになり続ける確率
    r = 0.97;
    for i in 0..3 {
      r = inspection_negative(&0.98, &0.05, &r);
      println!("{}: r={}", i, r);
    }
    // 罹患しているように見えたが0.03
    // 検査がnegativeとなってH2っぽいという確率
    r = 0.03;
    for i in 0..3 {
      r = inspection_negative(&0.98, &0.05, &r);
      println!("{}: r={}", i, r);
    }

    // P(H2)罹患していない確率は1-0.03
    // 罹患してなくて、検査がPositiveになり続ける確率
    r = 0.97;
    for i in 0..3 {
      r = inspection_false_positive(&0.98, &0.05, &r);
      println!("{}: r={}", i, r);
    }
  }
}
