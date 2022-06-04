use super::allele;
use super::config;
use super::gender;
use super::genome;
use rand::Rng;
use std::fmt;

pub struct Person {
  _age: u8,
  _birth_year: u64,
  _gender: gender::Gender,
  _genome: genome::Genome,
  _homosexual: Option<bool>,
  _dead: bool,
  _donor: bool,
}

impl fmt::Display for Person {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    return write!(f, "{{ Person: {{ age: {}, birth_year: {}, gender: {}, homosexual: {}, dead: {}, donor: {} }} }}", self._age, self._birth_year, self._gender, self._homosexual.unwrap(), self._dead, self._donor);
  }
}

impl Person {
  pub fn random() -> Self {
    let mut _self = Self {
      _gender: gender::Gender::random(),
      _age: rand::thread_rng().gen_range(0..config::BREED_RANGE[1]),
      _donor: rand::random::<bool>(),
      _birth_year: 0,
      _genome: genome::Genome::random(),
      _dead: false,
      _homosexual: None,
    };
    _self.set_homosexual(_self.determine_homosexual());
    return _self;
  }

  pub fn dummy() -> Self {
    let mut _self = Self {
      _gender: gender::Gender::random(),
      _age: 90,
      _donor: false,
      _birth_year: 0,
      _genome: genome::Genome::random(),
      _dead: false,
      _homosexual: Some(false),
    };
    return _self;
  }

  pub fn new(parent_1: &Person, parent_2: &Person, year: u64) -> Self {
    let mut _self = Self {
      _gender: gender::Gender::random(),
      _age: 0,
      _donor: rand::random::<bool>(),
      _birth_year: year,
      _genome: genome::Genome::new(&parent_1, &parent_2),
      _dead: false,
      _homosexual: None,
    };
    _self.set_homosexual(_self.determine_homosexual());
    return _self;
  }

  fn set_dead_status(&mut self, value: bool) {
    self._dead = value;
  }

  fn set_age(&mut self, age: u8) {
    self._age = age;
  }

  pub fn increment_age(&mut self) {
    #[allow(unused_parens)]
    if (self.get_dead() != true) {
      let new_age = self.get_age() + 1;
      self.set_age(new_age);
      let dead_age = rand::thread_rng().gen_range(config::DEATH_RANGE[1]..config::DEATH_RANGE[0]);
      if (self.get_age() > dead_age) {
        self.set_dead_status(true);
      }
    }
  }

  fn determine_homosexual(&self) -> bool {
    let gender = self.get_gender();
    let genome = self.get_genome();
    let mut series: f64 = 1.0;
    let mut series_override = true;

    // _rs11114975_12q21_31 [ALL_1]
    let _rs11114975_12q21_31 = genome.get_rs11114975_12q21_31();
    match _rs11114975_12q21_31.get_allele_1() {
      allele::Allele::Recessive => match _rs11114975_12q21_31.get_allele_2() {
        allele::Allele::Recessive => {
          series = series * (1.0 - ((0.0530 + 0.0035 + 0.0153 + 0.0146) / 3.0)) as f64;
        }
        allele::Allele::Dominant => {
          series_override = false;
        }
      },
      allele::Allele::Dominant => {
        series_override = false;
      }
    }

    // _rs10261857_7q31_2 [ALL_2]
    let _rs10261857_7q31_2 = genome.get_rs10261857_7q31_2();
    match _rs10261857_7q31_2.get_allele_1() {
      allele::Allele::Recessive => match _rs10261857_7q31_2.get_allele_2() {
        allele::Allele::Recessive => {
          series = series * (1.0 - ((0.0027 + 0.0229 + 0.0105) / 3.0)) as f64;
        }
        allele::Allele::Dominant => {
          series_override = false;
        }
      },
      allele::Allele::Dominant => {
        series_override = false;
      }
    }

    match gender {
      // GENDER [M]
      gender::Gender::M => {
        // _rs28371400_15q21_3 [M_1]
        let _rs28371400_15q21_3 = genome.get_rs28371400_15q21_3();
        match _rs28371400_15q21_3.get_allele_1() {
          allele::Allele::Recessive => match _rs28371400_15q21_3.get_allele_2() {
            allele::Allele::Recessive => {
              series = series * (1.0 - ((0.0049 + 0.0447 + 0.0180) / 3.0)) as f64;
            }
            allele::Allele::Dominant => {
              series_override = false;
            }
          },
          allele::Allele::Dominant => {}
        }
        // _rs34730029_11q12_1 [M_2]
        let _rs34730029_11q12_1 = genome.get_rs34730029_11q12_1();
        match _rs34730029_11q12_1.get_allele_1() {
          allele::Allele::Recessive => match _rs34730029_11q12_1.get_allele_2() {
            allele::Allele::Recessive => {
              series = series * (1.0 - ((0.0037 + 0.0566 + 0.0155) / 3.0)) as f64;
            }
            allele::Allele::Dominant => {
              series_override = false;
            }
          },
          allele::Allele::Dominant => {
            series_override = false;
          }
        }
      }
      // GENDER F
      gender::Gender::F => {
        let _rs13135637_4p14 = genome.get_rs13135637_4p14();
        match _rs13135637_4p14.get_allele_1() {
          allele::Allele::Recessive => match _rs13135637_4p14.get_allele_2() {
            allele::Allele::Recessive => {
              series = series * (1.0 - (-((-0.0005 + -0.0876 + -0.0143) / 3.0))) as f64;
            }
            allele::Allele::Dominant => {
              series_override = false;
            }
          },
          allele::Allele::Dominant => {
            series_override = false;
          }
        }
      }
    }

    if series_override == true {
      return true;
    }
    if series != 0.0 && rand::thread_rng().gen::<f64>() >= series {
      return true;
    }
    return false;
  }

  fn set_homosexual(&mut self, homosexual: bool) {
    self._homosexual = Some(homosexual);
  }

  pub fn get_gender(&self) -> &gender::Gender {
    return &self._gender;
  }

  pub fn get_can_breed(&self) -> bool {
    if self.get_in_breed_range() {
      if self.get_dead() != true {
        return true;
      }
    }
    return false;
  }

  pub fn get_in_breed_range(&self) -> bool {
    #[allow(unused_parens)]
    if (self.get_age() >= config::BREED_RANGE[0]) {
      if (self.get_age() <= config::BREED_RANGE[1]) {
        return true;
      }
    }
    return false;
  }

  fn get_donor(&self) -> bool {
    if self._donor == true {
      return true;
    }
    return false;
  }

  pub fn get_donor_status(&self) -> bool {
    if self.get_dead() == true {
      return false;
    }
    if self.get_in_breed_range() == false {
      return false;
    }
    if self.get_donor() == false {
      return false;
    }
    return true;
  }

  pub fn get_genome(&self) -> &genome::Genome {
    return &self._genome;
  }

  pub fn get_homosexual(&self) -> bool {
    match self._homosexual {
      Some(homosexual) => {
        if homosexual {
          return true;
        }
        return false;
      }
      None => return false,
    }
  }

  pub fn get_age(&self) -> u8 {
    return self._age;
  }

  pub fn get_dead(&self) -> bool {
    if self._dead == true {
      return true;
    }
    return false;
  }

  pub fn check_donor_eligible(&self, parent_2: &Person) -> bool {
    match self.get_gender() {
      gender::Gender::M => match parent_2.get_gender() {
        gender::Gender::M => return false,
        gender::Gender::F => return self.get_donor_status(),
      },
      gender::Gender::F => match parent_2.get_gender() {
        gender::Gender::F => return false,
        gender::Gender::M => return self.get_donor_status(),
      },
    }
  }
}
