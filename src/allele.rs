use rand::Rng;

#[derive(Copy, Clone)]
pub enum Allele {
  Recessive,
  Dominant
}

impl Allele {
  pub fn random() -> Self {
    let alleles = [Allele::Recessive, Allele::Dominant];
    let allele_type = &alleles[rand::thread_rng().gen_range(0..alleles.len())];
    return match allele_type {
      Allele::Recessive => Allele::Recessive,
      Allele::Dominant => Allele::Dominant
    }
  }
}