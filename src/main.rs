mod config;
mod gender;
mod person;
mod genome;

use rand::Rng;
use std::fs::File;
use std::io::Write;

fn main() {
  // Human vector
  let mut humans_vector: Vec<person::Person> = Vec::new();
  // Generation counter
  let mut generation: u32 = 0;
  let mut generation_offspring: u64 = 0;
  let mut generation_offspring_counter: u64 = 0;
  // let mut generation_offspring_percent: f32 = 0.0;
  // Person counter
  let mut total_person_counter: u64 = 0;
  let mut alive_person_counter: u64 = 0;
  let mut alive_homosexual_counter: u32 = 0;
  // Random events
  let mut generation_random_events_triggered: u32 = 0;
  let mut generation_random_events_success: u32 = 0;
  // let mut generation_random_event_expected: u32 = 0;
  let mut generation_random_event_offspring: u32 = 0;
  let mut generation_random_event_homosexual_offspring: u32 = 0;
  // Run off counter
  let mut run_off_counter: u16 = 0;
  // Homosexual extinct checker
  // let mut are_homosexuals_extinct: bool = false;
  let mut are_homosexual_breeders_extinct: bool = false;

  // Start application
  init_population(&mut humans_vector);
  init_text();
  #[allow(unused_parens)]
  while (check_homosexuals_extinct(true, &mut are_homosexual_breeders_extinct, &mut alive_homosexual_counter, &mut alive_person_counter, &mut humans_vector, &mut run_off_counter, &mut total_person_counter) == false || run_off_counter < config::RUN_OFF_GENERATIONS) {
    generation = generation + 1;
    generation_offspring_counter = 0;
    generation_offspring = 0;
    if (alive_person_counter > config::MAX_POPULATION) {
      break;
    }
    generation_offspring = set_children_in_generation(&alive_person_counter);

    println!(
"{{
  generation: {},
  alive_person_counter: {},
  generation_offspring: {},
  total_person_coutner: {},
  alive_homosexuals: {},
  human_vectors_len: {}
}}", 
generation, alive_person_counter, generation_offspring, total_person_counter, alive_homosexual_counter, humans_vector.len()
);

    let mut parent_1 = person::Person::fake();
    let mut parent_2 = person::Person::fake();
    while generation_offspring_counter < generation_offspring {
      let mut reverse_walker = humans_vector.len() - 1;
      while (check_mate(&parent_1, &parent_2) != true) {
        let index = rand::thread_rng().gen_range(0..(humans_vector.len()-1) as usize);
        // Is this dereferencing the object?
        parent_1 = humans_vector[index];
        parent_2 = humans_vector[reverse_walker];
        reverse_walker = reverse_walker - 1;
        if (reverse_walker == 0) {
          reverse_walker = humans_vector.len() - 1;
        }
      }
      mate(
        &parent_1, 
        &parent_2, 
        &mut humans_vector, 
        &generation, 
        &mut generation_random_events_triggered, 
        &mut generation_random_events_success, 
        &mut generation_random_event_offspring, 
        &mut generation_random_event_homosexual_offspring, 
        &mut generation_offspring_counter
      ); 
    }
    write_generation();
  }
  println!("process done");
}

fn init_text() {
  let mut ofile = File::create("result.txt").expect("unable to create file");
  let output = "Generation,Expected Offspring,Total Offspring,Homosexual Offspring,Random Events Expected,Random Events Triggered,Random Event Offspring (Homosexual Parent) [IVF/Surrogacy/Bi-Sexual],Alive Homosexuals,Homosexuals as percent of population,Alive Population,Total Population (Alive + Dead)";
  ofile.write_all(output.as_bytes()).expect("unable to write");
}

fn init_population(humans_vector: &mut Vec<person::Person>) {
  while (humans_vector.len() as u16) < config::STARTING_POPULATION {
    let new_human = person::Person::generate_random();
    humans_vector.push(new_human);
  }
}

fn write_generation() {

}

fn get_donor(parent_1: &person::Person, humans_vector: &Vec<person::Person>) -> Option<person::Person> {
  for person in humans_vector.iter().rev() {
    #[allow(unused_parens)]
    if (person.get_donor_eligible(&parent_1) == true) {
      return Option::Some(person.clone());
    }
  }
  return Option::None;
}

fn mate(parent_1: &person::Person, parent_2: &person::Person, humans_vector: &mut Vec<person::Person>, generation: &u32, generation_random_events_triggered: &mut u32, generation_random_events_success: &mut u32, generation_random_event_offspring: &mut u32, generation_random_event_homosexual_offspring: &mut u32, generation_offspring_counter: &mut u64) {
  #[allow(unused_parens)]
  // New Human
  fn new_human(parent_1: &person::Person, parent_2: &person::Person, humans_vector: &mut Vec<person::Person>, generation: &u32, generation_random_event_homosexual_offspring: &mut u32, generation_offspring_counter: &mut u64) {
    let offspring = person::Person::new(&parent_1, &parent_2, *generation);
    *generation_offspring_counter = *generation_offspring_counter + 1;
    if (offspring.get_homosexual_status() == true) {
      *generation_random_event_homosexual_offspring = *generation_random_event_homosexual_offspring + 1;
    }
    humans_vector.push(offspring);
  }

  fn new_human_with_donor(parent_1: &person::Person, donor: &person::Person, humans_vector: &mut Vec<person::Person>, generation: &u32, generation_random_event_offspring: &mut u32, generation_random_event_homosexual_offspring: &mut u32, generation_offspring_counter: &mut u64) {
    let offspring = person::Person::new(&parent_1, &donor, *generation);
    *generation_offspring_counter = *generation_offspring_counter + 1;
    *generation_random_event_offspring = *generation_random_event_offspring + 1;
    #[allow(unused_parens)]
    if (offspring.get_homosexual_status() == true) {
      *generation_random_event_homosexual_offspring = *generation_random_event_homosexual_offspring + 1;
    }
    humans_vector.push(offspring);
  }
  // End New Human

  // Homosexual parent(s)
  #[allow(unused_parens)]
  if (parent_1.get_homosexual_status() == true || parent_2.get_homosexual_status() == true) {
    *generation_random_events_triggered = *generation_random_events_triggered + (1 as u32);
    if (random_event() == true) {
      *generation_random_events_success = *generation_random_events_success + (1 as u32);
      // Homo-sexual
      if (parent_1.get_gender() == parent_2.get_gender()) {
        let donor = get_donor(&parent_1, &humans_vector);
        if (donor.is_none() != true) {
          new_human_with_donor(&parent_1, &donor.unwrap(), humans_vector, &generation, generation_random_event_offspring, generation_random_event_homosexual_offspring, generation_offspring_counter);
        }
      // Bi-sexual
      } else if (parent_1.get_gender() != parent_2.get_gender()) {
        new_human(&parent_1, &parent_2, humans_vector, &generation, generation_random_event_homosexual_offspring, generation_offspring_counter);
      }
    }
  // Heterosexual parents
  } else if (parent_1.get_homosexual_status() != true && parent_2.get_homosexual_status() != true) {
    new_human(&parent_1, &parent_2, humans_vector, &generation, generation_random_event_homosexual_offspring, generation_offspring_counter);
  }
}

fn check_mate(parent_1: &person::Person, parent_2: &person::Person) -> bool {
  #[allow(unused_parens)]
  if (std::ptr::eq(parent_1, parent_2) != true) {
    if (parent_1.get_in_breed_range() == true) {
      if (parent_2.get_in_breed_range() == true) {
        return true;
      }
    }
  }
  return false;
}

fn check_homosexuals_extinct(increment_age: bool, are_homosexual_breeders_extinct: &mut bool, alive_homosexual_counter: &mut u32, alive_person_counter: &mut u64, human_vector: &mut Vec<person::Person>, run_off_counter: &mut u16, total_person_counter: &mut u64) -> bool {
  *alive_person_counter = 0;
  *alive_homosexual_counter = 0;
  *are_homosexual_breeders_extinct = true;
  *total_person_counter = 0;
  for person in human_vector.iter_mut() {
    *total_person_counter = *total_person_counter+1;
    #[allow(unused_parens)]
    if (increment_age) {
      person.increment_age();
    }
    #[allow(unused_parens)]
    if (person.get_dead_status() != true) {
      *alive_person_counter = *alive_person_counter + 1;
      if (person.get_homosexual_status() == true) {
        *alive_homosexual_counter = *alive_homosexual_counter + 1;
        if (person.get_in_breed_range() == true) {
          *are_homosexual_breeders_extinct = false;
        }
      }
    }
  }
  #[allow(unused_parens)]
  if (*are_homosexual_breeders_extinct == true) {
    *run_off_counter = *run_off_counter + 1;
  }
  return *are_homosexual_breeders_extinct;
}

fn set_children_in_generation(alive_person_counter: &u64) -> u64 {
  let upper_percent = config::OFFSPRING_PERCENT[0];
  #[allow(unused_parens)]
  let upper_num = *alive_person_counter as f64 * upper_percent as f64;
  let lower_percent = config::OFFSPRING_PERCENT[1];
  #[allow(unused_parens)]
  let lower_num = *alive_person_counter as f64 * lower_percent as f64;
  let offspring_total: f64 = rand::thread_rng().gen_range(lower_num..upper_num);
  return offspring_total as u64 + 1;
}

fn random_event() -> bool {
  #[allow(unused_parens)]
  if (rand::thread_rng().gen::<f32>() <= config::RANDOM_EVENT_CHANCE) {
    return true;
  }
  return false;
}
