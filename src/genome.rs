use super::gene;
use super::person;

pub struct Genome {
  // GENDER [ALL]
  _rs11114975_12q21_31: gene::Gene,
  _rs10261857_7q31_2: gene::Gene,
  // GENDER [MALE]
  _rs28371400_15q21_3: gene::Gene,
  _rs34730029_11q12_1: gene::Gene,
  // [FEMALE ONLY]
  _rs13135637_4p14: gene::Gene,
}

impl Genome {
  pub fn random() -> Self {
    return Self {
      // GENDER [ALL]
      _rs11114975_12q21_31: gene::Gene::random(),
      _rs10261857_7q31_2: gene::Gene::random(),
      // GENDER [MALE]
      _rs28371400_15q21_3: gene::Gene::random(),
      _rs34730029_11q12_1: gene::Gene::random(),
      // [FEMALE ONLY]
      _rs13135637_4p14: gene::Gene::random(),
    };
  }

  pub fn new(parent_1: &person::Person, parent_2: &person::Person) -> Self {
    let genome_1 = parent_1.get_genome();
    let genome_2 = parent_2.get_genome();
    return Self {
      _rs11114975_12q21_31: gene::Gene::new(genome_1.get_rs11114975_12q21_31(), genome_2.get_rs11114975_12q21_31()),
      _rs10261857_7q31_2: gene::Gene::new(genome_1.get_rs10261857_7q31_2(), genome_2.get_rs10261857_7q31_2()),
      _rs28371400_15q21_3: gene::Gene::new(genome_1.get_rs28371400_15q21_3(), genome_2.get_rs28371400_15q21_3()),
      _rs34730029_11q12_1: gene::Gene::new(genome_1.get_rs34730029_11q12_1(), genome_2.get_rs34730029_11q12_1()),
      _rs13135637_4p14:  gene::Gene::new(genome_1.get_rs13135637_4p14(), genome_2.get_rs13135637_4p14())
    }
  }

  pub fn get_rs11114975_12q21_31(&self) -> &gene::Gene {
    return &self._rs11114975_12q21_31;
  }

  pub fn get_rs10261857_7q31_2(&self) -> &gene::Gene {
    return &self._rs10261857_7q31_2;
  }

  pub fn get_rs28371400_15q21_3(&self) -> &gene::Gene {
    return &self._rs28371400_15q21_3;
  }

  pub fn get_rs34730029_11q12_1(&self) -> &gene::Gene {
    return &self._rs34730029_11q12_1;
  }

  pub fn get_rs13135637_4p14(&self) -> &gene::Gene {
    return &self._rs13135637_4p14;
  }
} 
