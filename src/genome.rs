use super::person;
use super::gender;
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
pub enum Allele {
  Recessive,
  Dominant,
}

#[derive(Copy, Clone)]
pub struct Gene {
  pub allele_1: Allele,
  pub allele_2: Allele,
}

impl Gene {
  pub fn generate_random_gene() -> Gene {
    let alleles: [Gene; 4] = [
      Gene {
        allele_1: Allele::Recessive,
        allele_2: Allele::Recessive,
      },
      Gene {
        allele_1: Allele::Dominant,
        allele_2: Allele::Dominant,
      },
      Gene {
        allele_1: Allele::Dominant,
        allele_2: Allele::Recessive,
      },
      Gene {
        allele_1: Allele::Recessive,
        allele_2: Allele::Dominant,
      },
    ];
    return alleles[rand::thread_rng().gen_range(0..3)];
  }
}

#[derive(Copy, Clone)]
pub struct Genome {
  // GENDER [ALL]
  pub rs11114975_12q21_31: Gene,
  pub rs10261857_7q31_2: Gene,
  // GENDER [MALE]
  pub rs28371400_15q21_3: Gene,
  pub rs34730029_11q12_1: Gene,
  // [FEMALE ONLY]
  pub rs13135637_4p14: Gene,
}

impl Genome {
  pub fn generate_random_genome() -> Genome {
    return Genome {
      // ALL
      rs11114975_12q21_31: Gene::generate_random_gene(),
      rs10261857_7q31_2: Gene::generate_random_gene(),
      // MALE
      rs28371400_15q21_3: Gene::generate_random_gene(),
      rs34730029_11q12_1: Gene::generate_random_gene(),
      // FEMALE
      rs13135637_4p14: Gene::generate_random_gene(),
    };
  }

  pub fn law_of_seggregation(gene_1: &Gene, gene_2: &Gene) -> Gene {
    // [ ][R] [R]
    // [R][RR][RR]
    // [R][RR][RR]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Dominant && gene_1.allele_2 == Allele::Dominant) {
      if (gene_2.allele_1 == Allele::Dominant && gene_2.allele_2 == Allele::Dominant) {
        return Gene {
          allele_1: Allele::Dominant,
          allele_2: Allele::Dominant,
        };
      }
    }

    // [ ][r] [r]
    // [r][rr][rr]
    // [r][rr][rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Recessive && gene_1.allele_2 == Allele::Recessive) {
      if (gene_2.allele_1 == Allele::Recessive && gene_2.allele_2 == Allele::Recessive) {
        return Gene {
          allele_1: Allele::Recessive,
          allele_2: Allele::Recessive,
        };
      }
    }

    // [ ][R] [R]
    // [R][RR][RR]
    // [r][Rr][Rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Dominant && gene_1.allele_2 == Allele::Dominant) {
      if (gene_2.allele_1 == Allele::Dominant && gene_2.allele_2 == Allele::Recessive) {
        if (rand::thread_rng().gen_range(0..1) == 1) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        } else {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        }
      }
    }

    // [ ][R] [R]
    // [r][Rr][Rr]
    // [R][RR][RR]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Dominant && gene_1.allele_2 == Allele::Dominant) {
      if (gene_2.allele_1 == Allele::Recessive && gene_2.allele_2 == Allele::Dominant) {
        if (rand::thread_rng().gen_range(0..1) == 1) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        } else {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        }
      }
    }

    // [ ][R] [r]
    // [R][Rr][Rr]
    // [R][RR][RR]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Dominant && gene_1.allele_2 == Allele::Recessive) {
      if (gene_2.allele_1 == Allele::Dominant && gene_2.allele_2 == Allele::Dominant) {
        if (rand::thread_rng().gen_range(0..1) == 1) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        } else {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        }
      }
    }

    // [ ][r] [R]
    // [R][Rr][RR]
    // [R][Rr][RR]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Recessive && gene_1.allele_2 == Allele::Dominant) {
      if (gene_2.allele_1 == Allele::Dominant && gene_2.allele_2 == Allele::Dominant) {
        if (rand::thread_rng().gen_range(0..1) == 1) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        } else {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        }
      }
    }

    // [ ][R] [r]
    // [R][RR][Rr]
    // [r][Rr][rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Dominant && gene_1.allele_2 == Allele::Recessive) {
      if (gene_2.allele_1 == Allele::Dominant && gene_2.allele_2 == Allele::Recessive) {
        let num: u8 = rand::thread_rng().gen_range(0..3);
        if (num == 0) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        } else if (num == 1 || num == 2) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        } else if (num == 3) {
          return Gene {
            allele_1: Allele::Recessive,
            allele_2: Allele::Recessive,
          };
        }
      }
    }

    // [ ][r] [R]
    // [R][Rr][RR]
    // [r][Rr][rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Recessive && gene_1.allele_2 == Allele::Dominant) {
      if (gene_2.allele_1 == Allele::Dominant && gene_2.allele_2 == Allele::Recessive) {
        let num: u8 = rand::thread_rng().gen_range(0..3);
        if (num == 0) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        } else if (num == 1 || num == 2) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        } else if (num == 3) {
          return Gene {
            allele_1: Allele::Recessive,
            allele_2: Allele::Recessive,
          };
        }
      }
    }

    // [ ][R] [r]
    // [r][Rr][rr]
    // [R][RR][Rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Dominant && gene_1.allele_2 == Allele::Recessive) {
      if (gene_2.allele_1 == Allele::Recessive && gene_2.allele_2 == Allele::Dominant) {
        let num: u8 = rand::thread_rng().gen_range(0..3);
        if (num == 0) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        } else if (num == 1 || num == 2) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        } else if (num == 3) {
          return Gene {
            allele_1: Allele::Recessive,
            allele_2: Allele::Recessive,
          };
        }
      }
    }

    // [ ][r] [R]
    // [r][Rr][rr]
    // [R][Rr][RR]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Recessive && gene_1.allele_2 == Allele::Dominant) {
      if (gene_2.allele_1 == Allele::Recessive && gene_2.allele_2 == Allele::Dominant) {
        let num: u8 = rand::thread_rng().gen_range(0..3);
        if (num == 0) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        } else if (num == 1 || num == 2) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        } else if (num == 3) {
          return Gene {
            allele_1: Allele::Recessive,
            allele_2: Allele::Recessive,
          };
        }
      }
    }

    // [ ][R] [R]
    // [r][Rr][Rr]
    // [r][Rr][Rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Dominant && gene_1.allele_2 == Allele::Dominant) {
      if (gene_2.allele_1 == Allele::Recessive && gene_2.allele_2 == Allele::Recessive) {
        return Gene {
          allele_1: Allele::Dominant,
          allele_2: Allele::Recessive,
        };
      }
    }

    // [ ][r] [r]
    // [R][Rr][Rr]
    // [R][Rr][Rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Recessive && gene_1.allele_2 == Allele::Recessive) {
      if (gene_2.allele_1 == Allele::Dominant && gene_2.allele_2 == Allele::Dominant) {
        return Gene {
          allele_1: Allele::Dominant,
          allele_2: Allele::Recessive,
        };
      }
    }

    // [ ][R] [r]
    // [r][Rr][rr]
    // [r][Rr][rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Dominant && gene_1.allele_2 == Allele::Recessive) {
      if (gene_2.allele_1 == Allele::Recessive && gene_2.allele_2 == Allele::Recessive) {
        if (rand::thread_rng().gen_range(0..1) == 1) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        } else {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        }
      }
    }

    // [ ][r] [R]
    // [r][rr][Rr]
    // [r][rr][Rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Recessive && gene_1.allele_2 == Allele::Dominant) {
      if (gene_2.allele_1 == Allele::Recessive && gene_2.allele_2 == Allele::Dominant) {
        if (rand::thread_rng().gen_range(0..1) == 1) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        } else {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        }
      }
    }

    // [ ][r] [r]
    // [R][Rr][Rr]
    // [r][rr][rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Recessive && gene_1.allele_2 == Allele::Recessive) {
      if (gene_2.allele_1 == Allele::Dominant && gene_2.allele_2 == Allele::Recessive) {
        if (rand::thread_rng().gen_range(0..1) == 1) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        } else {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        }
      }
    }

    // [ ][r] [r]
    // [r][rr][rr]
    // [R][Rr][Rr]
    #[allow(unused_parens)]
    if (gene_1.allele_1 == Allele::Recessive && gene_1.allele_2 == Allele::Recessive) {
      if (gene_2.allele_1 == Allele::Recessive && gene_2.allele_2 == Allele::Dominant) {
        if (rand::thread_rng().gen_range(0..1) == 1) {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Recessive,
          };
        } else {
          return Gene {
            allele_1: Allele::Dominant,
            allele_2: Allele::Dominant,
          };
        }
      }
    } 

    // Fallback
    return Gene::generate_random_gene();
  }

  pub fn get_genome(parent_1: &person::Person, parent_2: &person::Person) -> Genome {
    let parent_1_genome = parent_1.get_genome();
    let parent_2_genome = parent_2.get_genome();
    return Genome {
      // ALL
      rs11114975_12q21_31: Genome::law_of_seggregation(&parent_1_genome.rs11114975_12q21_31, &parent_2_genome.rs11114975_12q21_31),
      rs10261857_7q31_2: Genome::law_of_seggregation(&parent_1_genome.rs10261857_7q31_2, &parent_2_genome.rs10261857_7q31_2),
      // MALE
      rs28371400_15q21_3: Genome::law_of_seggregation(&parent_1_genome.rs28371400_15q21_3, &parent_2_genome.rs28371400_15q21_3),
      rs34730029_11q12_1: Genome::law_of_seggregation(&parent_1_genome.rs34730029_11q12_1, &parent_2_genome.rs34730029_11q12_1),
      // FEMALE
      rs13135637_4p14: Genome::law_of_seggregation(&parent_1_genome.rs13135637_4p14, &parent_2_genome.rs13135637_4p14)
    };
  }

  pub fn get_homosexual(gender: &gender::Gender, genome: &Genome) -> bool {
    #[allow(unused_parens)]
    if (genome.rs11114975_12q21_31.allele_1 == Allele::Dominant && genome.rs11114975_12q21_31.allele_2 == Allele::Dominant) {
      if (genome.rs10261857_7q31_2.allele_1 == Allele::Dominant && genome.rs10261857_7q31_2.allele_2 == Allele::Dominant) {
        if (*gender == gender::Gender::M) {
          if (genome.rs28371400_15q21_3.allele_1 == Allele::Dominant && genome.rs28371400_15q21_3.allele_2 == Allele::Dominant) {
            if (genome.rs34730029_11q12_1.allele_1 == Allele::Dominant && genome.rs34730029_11q12_1.allele_2 == Allele::Dominant) {
              return true;
            }
          }
        } else if (*gender == gender::Gender::F) {
          if (genome.rs13135637_4p14.allele_1 == Allele::Dominant && genome.rs13135637_4p14.allele_2 == Allele::Dominant) {
            return true;
          }
        }
      }
    }
    return false;
  }
}
