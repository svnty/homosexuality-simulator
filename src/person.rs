use super::gender;
use super::config;
use super::genome;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Person {
  gender: gender::Gender,
  homosexual: bool,
  dead: bool,
  age: u8,
  #[allow(dead_code)]
  birth_year: u32,
  donor: bool,
  genome: genome::Genome
}

impl Person {
  pub fn new(parent_1: &Person, parent_2: &Person, generation: u32) -> Self {
    let genome = genome::Genome::get_genome(&parent_1, &parent_2);
    let donor = rand::random::<bool>();
    let gender: gender::Gender;
    #[allow(unused_parens)]
    if (rand::thread_rng().gen::<f32>() <= 0.5) {
      gender = gender::Gender::M;
    } else {
      gender = gender::Gender::F;
    }
    let homosexual = genome::Genome::get_homosexual(&gender, &genome);

    return Self {
      gender,
      dead: false,
      homosexual,
      age: 0,
      birth_year: generation,
      donor,
      genome
    }
  }

  pub fn generate_random() -> Self {
    let age = rand::thread_rng().gen_range(0..config::BREED_RANGE[0]);
    let donor = rand::random::<bool>();
    let genome = genome::Genome::generate_random_genome();
    let gender: gender::Gender;
    #[allow(unused_parens)]
    if (rand::thread_rng().gen::<f32>() <= 0.5) {
      gender = gender::Gender::M;
    } else {
      gender = gender::Gender::F;
    }
    let homosexual = genome::Genome::get_homosexual(&gender, &genome);

    return Self {
      age,
      birth_year: 0,
      donor,
      dead: false,
      gender,
      homosexual,
      genome
    }
  }

  pub fn fake() -> Self {
    let age = 99;
    let donor = false;
    let genome = genome::Genome::generate_random_genome();
    let gender = gender::Gender::M;
    let homosexual = true;
    let dead = true;

    return Self {
      age,
      birth_year: 0,
      donor,
      dead,
      gender,
      homosexual,
      genome
    }
  }

  pub fn get_genome(&self) -> genome::Genome {
    return self.genome;
  }

  pub fn get_gender(&self) -> gender::Gender {
    #[allow(unused_parens)]
    if (self.gender == gender::Gender::M) {
      return gender::Gender::M;
    }
    return gender::Gender::F;
  }

  pub fn get_donor_status(&self) -> bool {
    #[allow(unused_parens)]
    if (self.dead != true) {
      if (self.get_in_breed_range() == true) {
        if (self.donor == true) {
          return true;
        }
      }
    }
    return false;
  }

  pub fn get_donor_eligible(&self, parent_1: &Person) -> bool {
    #[allow(unused_parens)]
    if (self.gender == parent_1.gender) {
      return false;
    }
    return self.get_donor_status();
  }

  pub fn increment_age(&mut self) {
    #[allow(unused_parens)]
    if (self.dead != true) {
      self.age = self.age + 1;
      let mut rng = rand::thread_rng();
      let dead_age = rng.gen_range(config::DEATH_RANGE[1]..config::DEATH_RANGE[0]);
      if (self.age > dead_age) {
        self.dead = true;
      }
    }
  }

  pub fn get_homosexual_status(&self) -> bool {
    #[allow(unused_parens)]
    if (self.homosexual == true) {
      return true
    }
    return false;
  }

  pub fn get_dead_status(&self) -> bool {
    #[allow(unused_parens)]
    if (self.dead == true) {
      return true;
    }
    return false;
  }

  pub fn get_in_breed_range(&self) -> bool {
    #[allow(unused_parens)]
    if (self.age >= config::BREED_RANGE[0]) {
      if (self.age <= config::BREED_RANGE[1]) {
        return true;
      }
    }
    return false;
  }
}
