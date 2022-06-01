use rand::Rng;
use std::fmt;

pub enum Gender {
  M,
  F
}

impl fmt::Display for Gender {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Gender::M => write!(f, "M"),
      Gender::F => write!(f, "F"),
    }
  }
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
