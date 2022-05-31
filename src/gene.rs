use super::allele;
use rand::Rng;

#[derive(Copy, Clone)]
pub struct Gene {
  _allele_1: allele::Allele,
  _allele_2: allele::Allele
}

impl Gene {
  pub fn random() -> Self {
    return Self {
      _allele_1: allele::Allele::random(),
      _allele_2: allele::Allele::random(),
    }
  }

  pub fn new(gene_1: &Gene, gene_2: &Gene) -> Self {
    // [ ][R] [R]
    // [R][RR][RR]
    // [R][RR][RR]
    match gene_1.get_allele_1() {
      allele::Allele::Dominant => {
        match gene_1.get_allele_2() {
          allele::Allele::Dominant => {
            match gene_2.get_allele_1() {
              allele::Allele::Dominant => {
                match gene_2.get_allele_2() {
                  allele::Allele::Dominant => {
                    return Gene {
                      _allele_1: allele::Allele::Dominant,
                      _allele_2: allele::Allele::Dominant,
                    };
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][r] [r]
    // [r][rr][rr]
    // [r][rr][rr]
    match gene_1.get_allele_1() {
      allele::Allele::Recessive => {
        match gene_1.get_allele_2() {
          allele::Allele::Recessive => {
            match gene_2.get_allele_1() {
              allele::Allele::Recessive => {
                match gene_2.get_allele_2() {
                  allele::Allele::Recessive => {
                    return Gene {
                      _allele_1: allele::Allele::Recessive,
                      _allele_2: allele::Allele::Recessive,
                    };
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][R] [R]
    // [R][RR][RR]
    // [r][Rr][Rr]
    match gene_1.get_allele_1() {
      allele::Allele::Dominant => {
        match gene_1.get_allele_2() {
          allele::Allele::Dominant => {
            match gene_2.get_allele_1() {
              allele::Allele::Dominant => {
                match gene_2.get_allele_2() {
                  allele::Allele::Recessive => {
                    let rand: u8 = rand::thread_rng().gen_range(0..1);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Dominant,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][R] [R]
    // [r][Rr][Rr]
    // [R][RR][RR]
    match gene_1.get_allele_1() {
      allele::Allele::Dominant => {
        match gene_1.get_allele_2() {
          allele::Allele::Dominant => {
            match gene_2.get_allele_1() {
              allele::Allele::Recessive => {
                match gene_2.get_allele_2() {
                  allele::Allele::Dominant => {
                    let rand: u8 = rand::thread_rng().gen_range(0..1);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Dominant,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][R] [r]
    // [R][Rr][Rr]
    // [R][RR][RR]
    match gene_1.get_allele_1() {
      allele::Allele::Dominant => {
        match gene_1.get_allele_2() {
          allele::Allele::Recessive => {
            match gene_2.get_allele_1() {
              allele::Allele::Dominant => {
                match gene_2.get_allele_2() {
                  allele::Allele::Dominant => {
                    let rand: u8 = rand::thread_rng().gen_range(0..1);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Dominant,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][r] [R]
    // [R][Rr][RR]
    // [R][Rr][RR]
    match gene_1.get_allele_1() {
      allele::Allele::Recessive => {
        match gene_1.get_allele_2() {
          allele::Allele::Dominant => {
            match gene_2.get_allele_1() {
              allele::Allele::Dominant => {
                match gene_2.get_allele_2() {
                  allele::Allele::Dominant => {
                    let rand: u8 = rand::thread_rng().gen_range(0..1);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Dominant,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][R] [r]
    // [R][RR][Rr]
    // [r][Rr][rr]
    match gene_1.get_allele_1() {
      allele::Allele::Dominant => {
        match gene_1.get_allele_2() {
          allele::Allele::Recessive => {
            match gene_2.get_allele_1() {
              allele::Allele::Dominant => {
                match gene_2.get_allele_2() {
                  allele::Allele::Recessive => {
                    let rand: u8 = rand::thread_rng().gen_range(0..3);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Recessive,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Dominant,
                      }
                    }
                    if rand == 2 || rand == 3 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][r] [R]
    // [R][Rr][RR]
    // [r][Rr][rr]
    match gene_1.get_allele_1() {
      allele::Allele::Recessive => {
        match gene_1.get_allele_2() {
          allele::Allele::Dominant => {
            match gene_2.get_allele_1() {
              allele::Allele::Dominant => {
                match gene_2.get_allele_2() {
                  allele::Allele::Recessive => {
                    let rand: u8 = rand::thread_rng().gen_range(0..3);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Recessive,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Dominant,
                      }
                    }
                    if rand == 2 || rand == 3 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][R] [r]
    // [r][Rr][rr]
    // [R][RR][Rr]
    match gene_1.get_allele_1() {
      allele::Allele::Dominant => {
        match gene_1.get_allele_2() {
          allele::Allele::Recessive => {
            match gene_2.get_allele_1() {
              allele::Allele::Recessive => {
                match gene_2.get_allele_2() {
                  allele::Allele::Dominant => {
                    let rand: u8 = rand::thread_rng().gen_range(0..3);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Recessive,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Dominant,
                      }
                    }
                    if rand == 2 || rand == 3 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][r] [R]
    // [r][Rr][rr]
    // [R][Rr][RR]
    match gene_1.get_allele_1() {
      allele::Allele::Recessive => {
        match gene_1.get_allele_2() {
          allele::Allele::Dominant => {
            match gene_2.get_allele_1() {
              allele::Allele::Recessive => {
                match gene_2.get_allele_2() {
                  allele::Allele::Dominant => {
                    let rand: u8 = rand::thread_rng().gen_range(0..3);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Recessive,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Dominant,
                      }
                    }
                    if rand == 2 || rand == 3 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][R] [R]
    // [r][Rr][Rr]
    // [r][Rr][Rr]
    match gene_1.get_allele_1() {
      allele::Allele::Dominant => {
        match gene_1.get_allele_2() {
          allele::Allele::Dominant => {
            match gene_2.get_allele_1() {
              allele::Allele::Recessive => {
                match gene_2.get_allele_2() {
                  allele::Allele::Recessive => {
                    return Gene {
                      _allele_1: allele::Allele::Dominant,
                      _allele_2: allele::Allele::Recessive,
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][r] [r]
    // [R][Rr][Rr]
    // [R][Rr][Rr]
    match gene_1.get_allele_1() {
      allele::Allele::Recessive => {
        match gene_1.get_allele_2() {
          allele::Allele::Recessive => {
            match gene_2.get_allele_1() {
              allele::Allele::Dominant => {
                match gene_2.get_allele_2() {
                  allele::Allele::Dominant => {
                    return Gene {
                      _allele_1: allele::Allele::Dominant,
                      _allele_2: allele::Allele::Recessive,
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }
    
    // [ ][R] [r]
    // [r][Rr][rr]
    // [r][Rr][rr]
    match gene_1.get_allele_1() {
      allele::Allele::Dominant => {
        match gene_1.get_allele_2() {
          allele::Allele::Recessive => {
            match gene_2.get_allele_1() {
              allele::Allele::Recessive => {
                match gene_2.get_allele_2() {
                  allele::Allele::Recessive => {
                    let rand: u8 = rand::thread_rng().gen_range(0..1);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Recessive,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][r] [R]
    // [r][rr][Rr]
    // [r][rr][Rr]
    match gene_1.get_allele_1() {
      allele::Allele::Recessive => {
        match gene_1.get_allele_2() {
          allele::Allele::Dominant => {
            match gene_2.get_allele_1() {
              allele::Allele::Recessive => {
                match gene_2.get_allele_2() {
                  allele::Allele::Recessive => {
                    let rand: u8 = rand::thread_rng().gen_range(0..1);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Recessive,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][r] [r]
    // [R][Rr][Rr]
    // [r][rr][rr]
    match gene_1.get_allele_1() {
      allele::Allele::Recessive => {
        match gene_1.get_allele_2() {
          allele::Allele::Recessive => {
            match gene_2.get_allele_1() {
              allele::Allele::Dominant => {
                match gene_2.get_allele_2() {
                  allele::Allele::Recessive => {
                    let rand: u8 = rand::thread_rng().gen_range(0..1);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Recessive,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // [ ][r] [r]
    // [r][rr][rr]
    // [R][Rr][Rr]
    match gene_1.get_allele_1() {
      allele::Allele::Recessive => {
        match gene_1.get_allele_2() {
          allele::Allele::Recessive => {
            match gene_2.get_allele_1() {
              allele::Allele::Recessive => {
                match gene_2.get_allele_2() {
                  allele::Allele::Dominant => {
                    let rand: u8 = rand::thread_rng().gen_range(0..1);
                    if rand == 0 {
                      return Gene {
                        _allele_1: allele::Allele::Dominant,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                    if rand == 1 {
                      return Gene {
                        _allele_1: allele::Allele::Recessive,
                        _allele_2: allele::Allele::Recessive,
                      }
                    }
                  },
                  _ => ()
                }
              },
              _ => ()
            }
          },
          _ => ()
        }
      },
      _ => ()
    }

    // Fallback
    return Gene::random();
  }

  pub fn get_allele_1(&self) -> &allele::Allele {
    return &self._allele_1;
  }

  pub fn get_allele_2(&self) -> &allele::Allele {
    return &self._allele_2;
  }
}