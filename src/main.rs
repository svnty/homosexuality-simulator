mod config;

mod person;
mod gender;
mod genome;
mod gene;
mod allele;

use rand::Rng;
use std::io::Write;
use std::fs::OpenOptions;

static DEBUG: bool = false;
static STATUS: bool = false;

fn main() {
  // HOUSEKEEPING
  let mut generation: u64 = 0;
  let mut humans_vector: Vec<person::Person> = Vec::new();
  let mut total_person_counter: u64 = 0;
  let mut alive_person_counter: u64 = 0;
  let mut alive_homosexual_counter: u64 = 0;
  let mut dead_person_counter: u64 = 0;
  // OFFSPRING
  let mut generation_offspring_expected: u64 = 0;
  let mut generation_offspring_counter: u64 = 0;
  let mut generation_offspring_homosexual: u64 = 0;
  let mut generation_offspring_random_event: u64 = 0;
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
      &mut are_homosexual_breeders_extinct,
      &mut dead_person_counter
  ) || run_off_counter < config::RUN_OFF_GENERATIONS) {
    if DEBUG {
      println!("Starting generation");
    }
    // Set variables
    generation = generation + 1;
    generation_offspring_expected = set_children_in_generation(&alive_person_counter);
    generation_offspring_counter = 0;
    generation_offspring_homosexual = 0;
    generation_offspring_random_event = 0;
    
    // Break if population 10 billion
    if alive_person_counter >= config::MAX_POPULATION {
      break;
    }

    if STATUS {
      println!(
      "{{
        generation: {},
        alive_person_counter: {},
        total_person_counter: {},
        dead_person_counter: {},
        generation_offspring_expected: {},
        alive_homosexual_counter: {},
        human_vectors_len: {}
      }}", 
        generation, 
        alive_person_counter, 
        total_person_counter, 
        dead_person_counter, 
        generation_offspring_expected, 
        alive_homosexual_counter, 
        humans_vector.len()
      );
    }
    
    // Start mate
    // let mut reverse_walker = humans_vector.len();
    let mut new_human: Option<person::Person>;

    while generation_offspring_counter < generation_offspring_expected {
      let mut parent_1 = &person::Person::dummy();
      let mut parent_2 = &person::Person::dummy();

      if DEBUG {
        println!("while generation_offspring_counter < generation_offspring_expected");
      }
      while check_parents_can_mate(&parent_1, &parent_2) != true {
        if DEBUG {
          println!("while checkparents_can_mate()");
        }
        // reverse_walker = reverse_walker - 1;
        let min_index: usize = total_person_counter as usize - alive_person_counter as usize;
        let index = rand::thread_rng().gen_range(min_index..(humans_vector.len()-1));
        let index_new = rand::thread_rng().gen_range(min_index..(humans_vector.len()-1));
        // if reverse_walker <= 1 {
        //   reverse_walker = humans_vector.len();
        //   if DEBUG {
        //     println!("reverse walker: {}", reverse_walker);
        //   }
        // }

        parent_1 = &humans_vector[index];
        parent_2 = &humans_vector[index_new];
      }

      new_human = mate(
        &parent_1, 
        &parent_2, 
        &humans_vector,
        &generation, 
        &mut generation_offspring_counter,
        &mut generation_offspring_homosexual,
        &mut generation_offspring_random_event
      );

      // humans_vector.push(new_human.unwrap());
      match new_human {
        Some(person) => {
          humans_vector.push(person);
        }
        None => { 
          if DEBUG {
            println!("no human returned");
          }
        }
      }
    }

    // humans_vector.append(&mut new_vec);
    if DEBUG {
      println!("humans_vector.append(new_vec)");
    }
    write_generation(
      &generation,
      &generation_offspring_counter, 
      &generation_offspring_homosexual, 
      &generation_offspring_random_event, 
      &alive_homosexual_counter, 
      &alive_person_counter,
      &total_person_counter
    );
  }

  fn mate(parent_1: &person::Person, parent_2: &person::Person, humans_vector: &Vec<person::Person>, generation: &u64, generation_offspring_counter: &mut u64, generation_offspring_homosexual: &mut u64, generation_offspring_random_event: &mut u64) -> Option<person::Person> {
    if DEBUG {
      println!("init mate()");
    }
    let mut return_value: Option<person::Person> = None;

    fn create_new_human(parent_1: &person::Person, parent_2: &person::Person, generation: &u64, generation_offspring_counter: &mut u64, generation_offspring_homosexual: &mut u64) -> Option<person::Person> {
      if DEBUG {
        println!("init create_new_human()");
      }
      *generation_offspring_counter = *generation_offspring_counter + 1;
      let new_human = person::Person::new(&parent_1, &parent_2, *generation);
      if new_human.get_homosexual() {
        *generation_offspring_homosexual = *generation_offspring_homosexual + 1;
      }
      if DEBUG {
        println!("return create_new_human()");
      }
      return Some(new_human);
    }

    fn create_new_human_with_donor(parent_1: &person::Person, humans_vector: &Vec<person::Person>, generation: &u64, generation_offspring_counter: &mut u64, generation_offspring_homosexual: &mut u64) -> Option<person::Person> {
      if DEBUG {
        println!("init create_new_human_with_donor()");
      }
      fn get_donor(parent_1: &person::Person, humans_vector: &Vec<person::Person>, generation: &u64, generation_offspring_counter: &mut u64, generation_offspring_homosexual: &mut u64) -> Option<person::Person> {
        if DEBUG {
          println!("init get_donor()");
        }
        for person in humans_vector.iter().rev() {
          #[allow(unused_parens)]
          if (person.check_donor_eligible(&parent_1) == true) {
            if DEBUG {
              println!("return get_donor() Some");
            }
            return create_new_human(
              &parent_1, 
              &person, 
              &generation, 
              generation_offspring_counter, 
              generation_offspring_homosexual
            );
          }
        }
        if DEBUG {
          println!("return get_donor() None");
        }
        return None;
      }

      match get_donor(&parent_1, &humans_vector, &generation, generation_offspring_counter, generation_offspring_homosexual) {
        Some(donor) => {
          if DEBUG {
            println!("return create_new_human_with_donor() some");
          }
          return create_new_human(&parent_1, &donor, &generation, generation_offspring_counter, generation_offspring_homosexual);
        },
        _ => {
          if DEBUG {
            println!("return create_new_human_with_donor() none");
          }
          return None
        }
      }
    }

    fn random_event() -> bool {
      if DEBUG {
        println!("init random_event()");
      }
      let mut return_value: bool = false;
      if rand::thread_rng().gen::<f32>() <= config::RANDOM_EVENT_CHANCE {
        return_value = true;
      }
      if DEBUG {
        println!("return random_event() {}", return_value);
      }
      return return_value;
    }

    if parent_1.get_homosexual() || parent_2.get_homosexual() {
      if DEBUG {
        println!("parent_1: {}", parent_1);
        println!("parent_2: {}", parent_2);
        println!("parent.get_homosexual() true");
      }
      if config::RANDOM_EVENT_CHANCE > 0.0 && random_event() {
        *generation_offspring_random_event = *generation_offspring_random_event + 1;
        match parent_1.get_gender() {
          gender::Gender::M => {
            match parent_2.get_gender() {
              gender::Gender::M => {
                // Homosexual
                if DEBUG {
                  println!("mate() random[M:M] create_new_human_with_donor()");
                }
                return_value = create_new_human_with_donor(
                  &parent_1, 
                  &humans_vector, 
                  &generation, 
                  generation_offspring_counter, 
                  generation_offspring_homosexual
                );
              },
              gender::Gender::F => {
                // Bisexual
                if DEBUG {
                  println!("mate() random[M:F] create_new_human()");
                }
                return_value = create_new_human(
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
                // Homosexual
                if DEBUG {
                  println!("mate() random[F:F] create_new_human_with_donor()");
                }
                return_value = create_new_human_with_donor(
                  &parent_1, 
                  &humans_vector, 
                  &generation, 
                  generation_offspring_counter, 
                  generation_offspring_homosexual
                );
              },
              gender::Gender::M => {
                // Bisexual
                if DEBUG {
                  println!("mate() random[F:M] create_new_human()");
                }
                return_value = create_new_human(
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
    } else if parent_1.get_homosexual() != true && parent_2.get_homosexual() != true {
      if DEBUG {
        println!("mate() normal create_new_human()");
      }
      return_value = create_new_human(
        &parent_1, 
        &parent_2, 
        &generation, 
        generation_offspring_counter, 
        generation_offspring_homosexual
      );
    }
    if DEBUG {
      println!("return mate()");
    }
    return return_value;
  }

  fn check_parents_can_mate(parent_1: &person::Person, parent_2: &person::Person) -> bool {
    if DEBUG {
      println!("init check_parents_can_mate()");
    }
    let mut return_value: bool = false;
    if parent_1.get_can_breed() {
      if parent_2.get_can_breed() {
        return_value = true;
      }
    }
    if DEBUG {
      println!("return check_parents_can_mate() {}", return_value);
    }
    return return_value;
  }

  fn set_children_in_generation(alive_person_counter: &u64) -> u64 {
    if DEBUG {
      println!("init set_children_in_generation()");
    }
    let upper_percent = config::OFFSPRING_PERCENT[0];
    let upper_num = *alive_person_counter as f64 * upper_percent as f64;
    let lower_percent = config::OFFSPRING_PERCENT[1];
    let lower_num = *alive_person_counter as f64 * lower_percent as f64;
    let offspring_total: f64 = rand::thread_rng().gen_range(lower_num..upper_num);
    if DEBUG {
      println!("return set_children_in_generation()");
    }
    return offspring_total as u64 + 1;
  }

  fn init_population(humans_vector: &mut Vec<person::Person>) {
    while (humans_vector.len() as u16) < config::STARTING_POPULATION {
      let new_human = person::Person::random();
      humans_vector.push(new_human);
    }
  }

  fn init_text() {
    let mut ofile = OpenOptions::new().write(true).create(true).truncate(true).open("result.txt").unwrap();
    let output = "Generation,Generation Offspring,Homosexual Offspring,Random Event Offspring (Homosexual Parent) [IVF/Surrogacy/Bi-Sexual],Alive Homosexuals,Homosexuals %,Alive Population,Total Population (Alive + Dead)\n";
    ofile.write_all(output.as_bytes()).expect("unable to write");
  }

  fn write_generation(generation: &u64, generation_offspring_counter: &u64, generation_offspring_homosexual: &u64, generation_offspring_random_event: &u64, alive_homosexual_counter: &u64, alive_person_counter: &u64, total_person_counter: &u64) {
    let mut ofile = OpenOptions::new().write(true).append(true).create(false).truncate(false).open("result.txt").unwrap();
    let homosexual_percent: f64 = (*alive_homosexual_counter as f64 / *total_person_counter as f64) * 100.0;
    let output = generation.to_string() + "," + &generation_offspring_counter.to_string() + "," + &generation_offspring_homosexual.to_string() + "," + &generation_offspring_random_event.to_string() + "," + &alive_homosexual_counter.to_string() + "," + &homosexual_percent.to_string() + "," + &alive_person_counter.to_string() + "," + &total_person_counter.to_string() + "\n";
    ofile.write_all(output.as_bytes()).expect("unable to write");
  }

  fn check_homosexuals_extinct(humans_vector: &mut Vec<person::Person>, total_person_counter: &mut u64, alive_person_counter: &mut u64, alive_homosexual_counter: &mut u64, run_off_counter: &mut u16, are_homosexual_breeders_extinct: &mut bool, dead_person_counter: &mut u64) -> bool {
    if DEBUG {
      println!("init check_homosexuals_extinct()");
    }
    *total_person_counter = 0;
    *alive_person_counter = 0;
    *dead_person_counter = 0;
    *alive_homosexual_counter = 0;
    *are_homosexual_breeders_extinct = false;

    for person in humans_vector.iter_mut() {
      person.increment_age();
      *total_person_counter = * total_person_counter + 1;
      if person.get_dead() == false {
        *alive_person_counter = *alive_person_counter + 1;
        if person.get_homosexual() == true {
          *alive_homosexual_counter = *alive_homosexual_counter + 1;
          if person.get_in_breed_range() == true {
            *are_homosexual_breeders_extinct = false;
          }
        }
      } else {
        *dead_person_counter = *dead_person_counter + 1;
      }
    }
    if *are_homosexual_breeders_extinct == true {
      *run_off_counter = *run_off_counter + 1;
    }
    if DEBUG {
      println!("return check_homosexuals_extinct()");
    }
    return *are_homosexual_breeders_extinct;
  }

  println!("Process exiting");
}