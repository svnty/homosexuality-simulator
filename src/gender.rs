use std::fmt;

#[derive(Copy, Clone)]
#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Gender {
  M,
  F
}

impl fmt::Display for Gender {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Gender::M => write!(f, "Male"),
      Gender::F => write!(f, "Female"),
    }
  }
}
