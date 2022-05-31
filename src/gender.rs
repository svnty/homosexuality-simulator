use rand::Rng;

#[derive(Copy, Clone)]
pub enum Gender {
  M,
  F
}

impl Gender {
  pub fn random() -> Self {
    let genders = [Gender::M, Gender::F];
    let gender_type = &genders[rand::thread_rng().gen_range(0..genders.len())];
    return match gender_type {
      Gender::M => Gender::M,
      Gender::F => Gender::F
    }
  }
}