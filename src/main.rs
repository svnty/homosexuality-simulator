mod config;

mod person;
mod gender;
mod genome;
mod gene;
mod allele;

use rand::Rng;
use std::fs::File;
use std::io::Write;

fn main() {
  // HOUSEKEEPING
  let mut generation: u32 = 0;
  let mut humans_vector: Vec<person::Person> = Vec::new();
  let mut total_person_counter: u64 = 0;
  let mut alive_person_counter: u64 = 0;
  let mut alive_homosexual_counter: u64 = 0;
  // OFFSPRING
  let mut generation_offspring_expected: u64 = 0;
  let mut generation_offspring_counter: u64 = 0;
  let mut generation_offspring_homosexual: u64 = 0;
  // Counterdown starts once breeders are extinct
  let mut run_off_counter: u16 = 0;
  let mut are_homosexual_breeders_extinct: bool = false;

  init_population(&mut humans_vector);
  init_text();

  // Program loop
  while (
    check_homosexuals_extinct(
      &mut humans_vector, 
      &mut total_person_counter, 
      &mut alive_person_counter, 
      &mut alive_homosexual_counter, 
      &mut run_off_counter, 
      &mut are_homosexual_breeders_extinct
  ) || run_off_counter < config::RUN_OFF_GENERATIONS) {
    // Set variables
    generation = generation + 1;
    generation_offspring_expected = set_children_in_generation(&alive_person_counter);
    generation_offspring_counter = 0;
    generation_offspring_homosexual = 0;
    
    // Break if population 10 billion
    if (alive_person_counter >= config::MAX_POPULATION) {
      break;
    }

    // debug
    println!(
"{{
  generation: {},
  alive_person_counter: {},
  generation_offspring_expected: {},
  total_person_coutner: {},
  alive_homosexuals: {},
  human_vectors_len: {}
}}", 
      generation, 
      alive_person_counter, 
      generation_offspring_expected, 
      total_person_counter, 
      alive_homosexual_counter, 
      humans_vector.len()
    );

    // Start mate
    let mut parent_1 = person::Person::dummy();
    let mut parent_2 = person::Person::dummy();

    while generation_offspring_counter < generation_offspring_expected {
      let mut reverse_walker = humans_vector.len() - 1;
      while check_parents_can_mate(&parent_1, &parent_2) != true {
        let index = rand::thread_rng().gen_range(0..(humans_vector.len()-1) as usize);
        parent_1 = *humans_vector.get(index).unwrap();
        parent_2 = *humans_vector.get(reverse_walker).unwrap();
        reverse_walker = reverse_walker - 1;
        if (reverse_walker == 0) {
          reverse_walker = humans_vector.len() - 1;
        }
      }
      let new_human = mate(
        &parent_1, 
        &parent_2, 
        &humans_vector,
        &generation, 
        &mut generation_offspring_counter,
        &mut generation_offspring_homosexual
      );

      match new_human {
        Some(person) => {
          humans_vector.push(person);
        }
        None => ()
      }
    }
  }

  fn mate(
    parent_1: &person::Person, 
    parent_2: &person::Person,
    humans_vector: &Vec<person::Person>,
    generation: &u32, 
    generation_offspring_counter: &mut u64,
    generation_offspring_homosexual: &mut u64
  ) -> Option<person::Person> {

    fn create_new_human(
      parent_1: &person::Person, 
      parent_2: &person::Person, 
      generation: &u32, 
      generation_offspring_counter: &mut u64, 
      generation_offspring_homosexual: &mut u64
    ) -> Option<person::Person> {
      *generation_offspring_counter = *generation_offspring_counter + 1;
      let new_human = person::Person::new(&parent_1, &parent_2, *generation);
      if new_human.get_homosexual() {
        *generation_offspring_homosexual = *generation_offspring_homosexual + 1;
      }
      return Some(new_human);
    }

    fn create_new_human_with_donor(
      parent_1: &person::Person, 
      humans_vector: &Vec<person::Person>, 
      generation: &u32, 
      generation_offspring_counter: &mut u64, 
      generation_offspring_homosexual: &mut u64
    ) -> Option<person::Person> {
      fn get_donor(
        parent_1: &person::Person, 
        humans_vector: &Vec<person::Person>, 
        generation: &u32, 
        generation_offspring_counter: &mut u64, 
        generation_offspring_homosexual: &mut u64
      ) -> Option<person::Person> {
        for person in humans_vector.iter().rev() {
          #[allow(unused_parens)]
          if (person.check_donor_eligible(&parent_1) == true) {
            return create_new_human(
              &parent_1, 
              &person, 
              &generation, 
              generation_offspring_counter, 
              generation_offspring_homosexual
            );
          }
        }
        return None;
      }

      match get_donor(
        &parent_1, 
        &humans_vector, 
        &generation, 
        generation_offspring_counter, 
        generation_offspring_homosexual
      ) {
        Some(donor) => return create_new_human(
          &parent_1, 
          &donor, 
          &generation, 
          generation_offspring_counter, 
          generation_offspring_homosexual
        ),
        None => return None
      }
    }

    fn random_event() -> bool {
      #[allow(unused_parens)]
      if (rand::thread_rng().gen::<f32>() <= config::RANDOM_EVENT_CHANCE) {
        return true;
      }
      return false;
    }

    if parent_1.get_homosexual() || parent_2.get_homosexual() {
      if random_event() {
        match parent_1.get_gender() {
          gender::Gender::M => {
            match parent_2.get_gender() {
              gender::Gender::M => {
                return create_new_human_with_donor(
                  &parent_1, 
                  &humans_vector, 
                  &generation, 
                  generation_offspring_counter, 
                  generation_offspring_homosexual
                );
              },
              _ => {
                return create_new_human(
                  &parent_1, 
                  &parent_2, 
                  &generation, 
                  generation_offspring_counter, 
                  generation_offspring_homosexual
                );
              }
            }
          },
          gender::Gender::F => {
            match parent_2.get_gender() {
              gender::Gender::F => {
                return create_new_human_with_donor(
                  &parent_1, 
                  &humans_vector,
                  &generation, 
                  generation_offspring_counter, 
                  generation_offspring_homosexual
                );
              },
              _ => {
                return create_new_human(
                  &parent_1, 
                  &parent_2, 
                  &generation, 
                  generation_offspring_counter,
                  generation_offspring_homosexual
                );
              }
            }
          }
        }
      }
    } else {
      return create_new_human(
        &parent_1, 
        &parent_2, 
        &generation, 
        generation_offspring_counter, 
        generation_offspring_homosexual
      );
    }
    return None;
  }

  fn check_parents_can_mate(parent_1: &person::Person, parent_2: &person::Person) -> bool {
    if parent_1.get_can_breed() {
      if parent_2.get_can_breed() {
        return true;
      }
    }
    return false;
  }

  fn set_children_in_generation(alive_person_counter: &u64) -> u64 {
    let upper_percent = config::OFFSPRING_PERCENT[0];
    let upper_num = *alive_person_counter as f64 * upper_percent as f64;
    let lower_percent = config::OFFSPRING_PERCENT[1];
    let lower_num = *alive_person_counter as f64 * lower_percent as f64;
    let offspring_total: f64 = rand::thread_rng().gen_range(lower_num..upper_num);
    return offspring_total as u64 + 1;
  }

  fn init_population(humans_vector: &mut Vec<person::Person>) {
    while (humans_vector.len() as u16) < config::STARTING_POPULATION {
      let new_human = person::Person::random();
      humans_vector.push(new_human);
    }
  }

  fn init_text() {
    let mut ofile = File::create("result.txt").expect("unable to create file");
    let output = "Generation,Expected Offspring,Total Offspring,Homosexual Offspring,Random Events Expected,Random Events Triggered,Random Event Offspring (Homosexual Parent) [IVF/Surrogacy/Bi-Sexual],Alive Homosexuals,Homosexuals as percent of population,Alive Population,Total Population (Alive + Dead)";
    ofile.write_all(output.as_bytes()).expect("unable to write");
  }

  fn check_homosexuals_extinct(
    humans_vector: &mut Vec<person::Person>,
    total_person_counter: &mut u64,
    alive_person_counter: &mut u64,
    alive_homosexual_counter: &mut u64, 
    run_off_counter: &mut u16,
    are_homosexual_breeders_extinct: &mut bool
  ) -> bool {
    *total_person_counter = 0;
    *alive_person_counter = 0;
    *alive_homosexual_counter = 0;
    *are_homosexual_breeders_extinct = false;

    for person in humans_vector.iter_mut() {
      person.increment_age();
      if (person.get_dead() == false) {
        *alive_person_counter = *alive_person_counter + 1;
        if (person.get_homosexual() == true) {
          *alive_homosexual_counter = *alive_homosexual_counter + 1;
          if (person.get_in_breed_range() == true) {
            *are_homosexual_breeders_extinct = false;
          }
        }
      }
    }
    if (*are_homosexual_breeders_extinct == true) {
      *run_off_counter = *run_off_counter + 1;
    }
    return *are_homosexual_breeders_extinct;
  }

  println!("Process exiting");
}