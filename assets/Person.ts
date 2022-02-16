import CONFIG from './configuration';

type Gene = {
    'one': string,
    'two': string
}

export class Person {
    parent_1?: Person;
    parent_2?: Person
    homosexual: boolean;
    dead: boolean;
    birth_year: number;
    age: number;
    donor: boolean;
    gender: string;
    gwas: {
        // [ALL]
        'rs11114975-12q21.31': {
            'allele': Gene
        },
        'rs10261857-7q31.2': {
            'allele': Gene
        },
        // [MALE ONLY]
        'rs28371400-15q21.3': {
            'allele': Gene
        },
        'rs34730029-11q12.1': {
            'allele': Gene
        },
        // [FEMALE ONLY]
        'rs13135637-4p14': {
            'allele': Gene
        }
    };

    constructor(parent_1?: Person, parent_2?: Person, age: number = 0, homosexual: boolean = false, donor: boolean = false, year: number = 0) {
        this.parent_1 = parent_1;
        this.parent_2 = parent_2;
        this.homosexual = homosexual;
        this.dead = false;
        this.age = age;
        this.birth_year = year;

        if (donor == true)
            this.donor = donor;
        else
            this.donor = (Math.random() <= 0.5) ? true : false;

        this.gwas = {
            'rs11114975-12q21.31': {
                //'gender': 'ALL',
                'allele': {
                    'one': '',
                    'two': ''
                }
            },
            'rs10261857-7q31.2': {
                //'gender': 'ALL',
                'allele': {
                    'one': '',
                    'two': ''
                }
            },
            'rs28371400-15q21.3': {
                //'gender': 'M',
                'allele': {
                    'one': '',
                    'two': ''
                }
            },
            'rs34730029-11q12.1': {
                //'gender': 'M',
                'allele': {
                    'one': '',
                    'two': ''
                }
            },
            'rs13135637-4p14': {
                //'gender': 'F',
                'allele': {
                    'one': '',
                    'two': ''
                }
            }
        };

        if (Math.random() <= 0.5) {
            this.gender = 'M';
        } else {
            this.gender = 'F';
        }

        this.setAlleles(parent_1, parent_2);
        this.setHomosexual();
        if (CONFIG.sociological_toggle == true) {
            this.sociologicalFix();
        }
    }

    setAlleles(parent_1?: Person, parent_2?: Person): void {
        if (parent_1 && parent_2) {
            let parent_1_rs11114975_12q21_31 = parent_1.gwas['rs11114975-12q21.31']['allele'];
            let parent_2_rs11114975_12q21_31 = parent_2.gwas['rs11114975-12q21.31']['allele'];
            this.gwas['rs11114975-12q21.31']['allele'] = this.lawOfSeggregation(parent_1_rs11114975_12q21_31, parent_2_rs11114975_12q21_31);
            let parent_1_rs10261857_7q31_2 = parent_1.gwas['rs10261857-7q31.2']['allele'];
            let parent_2_rs10261857_7q31_2 = parent_2.gwas['rs10261857-7q31.2']['allele'];
            this.gwas['rs10261857-7q31.2']['allele'] = this.lawOfSeggregation(parent_1_rs10261857_7q31_2, parent_2_rs10261857_7q31_2);
            let parent_1_rs28371400_15q21_3 = parent_1.gwas['rs28371400-15q21.3']['allele'];
            let parent_2_rs28371400_15q21_3 = parent_2.gwas['rs28371400-15q21.3']['allele'];
            this.gwas['rs28371400-15q21.3']['allele'] = this.lawOfSeggregation(parent_1_rs28371400_15q21_3, parent_2_rs28371400_15q21_3);
            let parent_1_rs34730029_11q12_1 = parent_1.gwas['rs34730029-11q12.1']['allele'];
            let parent_2_rs34730029_11q12_1 = parent_2.gwas['rs34730029-11q12.1']['allele'];
            this.gwas['rs34730029-11q12.1']['allele'] = this.lawOfSeggregation(parent_1_rs34730029_11q12_1, parent_2_rs34730029_11q12_1);
            let parent_1_rs13135637_4p14 = parent_1.gwas['rs13135637-4p14']['allele'];
            let parent_2_rs13135637_4p14 = parent_2.gwas['rs13135637-4p14']['allele'];
            this.gwas['rs13135637-4p14']['allele'] = this.lawOfSeggregation(parent_1_rs13135637_4p14, parent_2_rs13135637_4p14);
        } else {
            if (this.homosexual == true) {
                if (this.gender == 'M') {
                    this.gwas['rs11114975-12q21.31']['allele'] = {
                        'one': 'r',
                        'two': 'r'
                    };
                    this.gwas['rs10261857-7q31.2']['allele'] = {
                        'one': 'r',
                        'two': 'r'
                    };
                    this.gwas['rs28371400-15q21.3']['allele'] = {
                        'one': 'r',
                        'two': 'r'
                    };
                    this.gwas['rs34730029-11q12.1']['allele'] = {
                        'one': 'r',
                        'two': 'r'
                    };
                    this.gwas['rs13135637-4p14']['allele'] = this.randomAllele();
                } else if (this.gender == 'F') {
                    this.gwas['rs11114975-12q21.31']['allele'] = {
                        'one': 'r',
                        'two': 'r'
                    };
                    this.gwas['rs10261857-7q31.2']['allele'] = {
                        'one': 'r',
                        'two': 'r'
                    };
                    this.gwas['rs28371400-15q21.3']['allele'] = this.randomAllele();
                    this.gwas['rs34730029-11q12.1']['allele'] = this.randomAllele();
                    this.gwas['rs13135637-4p14']['allele'] = {
                        'one': 'r',
                        'two': 'r'
                    };
                }
            } else {
                this.gwas['rs11114975-12q21.31']['allele'] = this.lawOfSeggregation(this.randomAlleleHeterosexual(), this.randomAlleleHeterosexual());
                this.gwas['rs10261857-7q31.2']['allele'] = this.lawOfSeggregation(this.randomAlleleHeterosexual(), this.randomAlleleHeterosexual());
                this.gwas['rs28371400-15q21.3']['allele'] = this.lawOfSeggregation(this.randomAlleleHeterosexual(), this.randomAlleleHeterosexual());
                this.gwas['rs34730029-11q12.1']['allele'] = this.lawOfSeggregation(this.randomAlleleHeterosexual(), this.randomAlleleHeterosexual());
                this.gwas['rs13135637-4p14']['allele'] = this.lawOfSeggregation(this.randomAlleleHeterosexual(), this.randomAlleleHeterosexual());
            }
        }
    }

    setHomosexual(): void {
        // rs11114975-12q21.31 [ALL]
        if (this.gwas['rs11114975-12q21.31']['allele']['one'] == 'r') {
            if (this.gwas['rs11114975-12q21.31']['allele']['two'] == 'r') {
                // rs10261857-7q31.2 [ALL]
                if (this.gwas['rs10261857-7q31.2']['allele']['one'] == 'r') {
                    if (this.gwas['rs10261857-7q31.2']['allele']['two'] == 'r') {
                        if (this.gender == 'M') {
                            // rs28371400-15q21.3 [MALE ONLY]
                            if (this.gwas['rs28371400-15q21.3']['allele']['one'] == 'r') {
                                if (this.gwas['rs28371400-15q21.3']['allele']['two'] == 'r') {
                                    // rs34730029-11q12.1 [MALE ONLY]
                                    if (this.gwas['rs34730029-11q12.1']['allele']['one'] == 'r') {
                                        if (this.gwas['rs34730029-11q12.1']['allele']['two'] == 'r') {
                                            this.homosexual = true;
                                        }
                                    }
                                }
                            }
                        } else if (this.gender == 'F') {
                            // rs13135637-4p14 [FEMALE ONLY]
                            if (this.gwas['rs13135637-4p14']['allele']['one'] == 'r') {
                                if (this.gwas['rs13135637-4p14']['allele']['two'] == 'r') {
                                    this.homosexual = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        this.homosexual = false;
    }

    sociologicalFix(): void {
        // rs10261857-7q31.2 [ALL]
        if (this.gwas['rs10261857-7q31.2']['allele']['one'] == 'r') {
            if (this.gwas['rs10261857-7q31.2']['allele']['two'] == 'r') {
                if (Math.random() <= CONFIG.sociological_chance) {
                    this.homosexual = true;
                }
            }
        }
        // rs11114975-12q21.31 [ALL]
        if (this.gwas['rs11114975-12q21.31']['allele']['one'] == 'r') {
            if (this.gwas['rs11114975-12q21.31']['allele']['two'] == 'r') {
                if (Math.random() <= CONFIG.sociological_chance) {
                    this.homosexual = true;
                }
            }
        }
        if (this.gender == 'M') {
            // rs28371400-15q21.3 [MALE ONLY]
            if (this.gwas['rs28371400-15q21.3']['allele']['one'] == 'r') {
                if (this.gwas['rs28371400-15q21.3']['allele']['two'] == 'r') {
                    if (Math.random() <= CONFIG.sociological_chance) {
                        this.homosexual = true;
                    }
                }
            }
            // rs34730029-11q12.1 [MALE ONLY]
            if (this.gwas['rs34730029-11q12.1']['allele']['one'] == 'r') {
                if (this.gwas['rs34730029-11q12.1']['allele']['two'] == 'r') {
                    if (Math.random() <= CONFIG.sociological_chance) {
                        this.homosexual = true;
                    }
                }
            }
        }
        if (this.gender == 'F') {
            // rs13135637-4p14 [FEMALE ONLY]
            if (this.gwas['rs13135637-4p14']['allele']['one'] == 'r') {
                if (this.gwas['rs13135637-4p14']['allele']['two'] == 'r') {
                    if (Math.random() <= CONFIG.sociological_chance) {
                        this.homosexual = true;
                    }
                }
            }
        }
    }

    lawOfSeggregation(gene_1: Gene, gene_2: Gene): Gene {
        // [ ][R] [R]
        // [R][RR][RR]
        // [R][RR][RR]
        if (gene_1['one'] == 'R' && gene_1['two'] == 'R') {  //Row
            if (gene_2['one'] == 'R' && gene_2['two'] == 'R') { //Column
                return { 'one': 'R', 'two': 'R' }
            }
        }

        // [ ][r] [r]
        // [r][rr][rr]
        // [r][rr][rr]
        if (gene_1['one'] == 'r' && gene_1['two'] == 'r') {  //Row
            if (gene_2['one'] == 'r' && gene_2['two'] == 'r') { //Column
                return { 'one': 'r', 'two': 'r' }
            }
        }

        // [ ][R] [R]
        // [R][RR][RR]
        // [r][Rr][Rr]
        if (gene_1['one'] == 'R' && gene_1['two'] == 'R') {  //Row
            if (gene_2['one'] == 'R' && gene_2['two'] == 'r') { //Column
                const random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'R' }
                }
                if (random_chance == 1) {
                    return { 'one': 'R', 'two': 'r' }
                }
            }
        }

        // [ ][R] [R]
        // [r][Rr][Rr]
        // [R][RR][RR]
        if (gene_1['one'] == 'R' && gene_1['two'] == 'R') {  //Row
            if (gene_2['one'] == 'r' && gene_2['two'] == 'R') { //Column
                const random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'R' }
                }
                if (random_chance == 1) {
                    return { 'one': 'R', 'two': 'r' }
                }
            }
        }

        // [ ][R] [r]
        // [R][Rr][Rr]
        // [R][RR][RR]
        if (gene_1['one'] == 'R' && gene_1['two'] == 'r') {  //Row
            if (gene_2['one'] == 'R' && gene_2['two'] == 'R') {  //Column
                const random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'R' }
                }
                if (random_chance == 1) {
                    return { 'one': 'R', 'two': 'r' }
                }
            }
        }

        // [ ][r] [R]
        // [R][Rr][RR]
        // [R][Rr][RR]
        if (gene_1['one'] == 'r' && gene_1['two'] == 'R') { //Row
            if (gene_2['one'] == 'R' && gene_2['two'] == 'R') {  //Column
                const random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'R' }
                }
                if (random_chance == 1) {
                    return { 'one': 'R', 'two': 'r' }
                }
            }
        }

        // [ ][R] [r]
        // [R][RR][Rr]
        // [r][Rr][rr]
        if (gene_1['one'] == 'R' && gene_1['two'] == 'r') {  //Row
            if (gene_2['one'] == 'R' && gene_2['two'] == 'r') {  //Column
                const random_chance = Math.floor(Math.random() * (4 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'R' }
                }
                if (random_chance == 1 || random_chance == 2) {
                    return { 'one': 'R', 'two': 'r' }
                }
                if (random_chance == 3) {
                    return { 'one': 'r', 'two': 'r' }
                }
            }
        }

        // [ ][r] [R]
        // [R][Rr][RR]
        // [r][Rr][rr]
        if (gene_1['one'] == 'r' && gene_1['two'] == 'R') {  //Row
            if (gene_2['one'] == 'R' && gene_2['two'] == 'r') {  //Column
                const random_chance = Math.floor(Math.random() * (4 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'R' }
                }
                if (random_chance == 1 || random_chance == 2) {
                    return { 'one': 'R', 'two': 'r' }
                }
                if (random_chance == 3) {
                    return { 'one': 'r', 'two': 'r' }
                }
            }
        }

        // [ ][R] [r]
        // [r][Rr][rr]
        // [R][Rr][Rr]
        if (gene_1['one'] == 'R' && gene_1['two'] == 'r') {  //Row
            if (gene_2['one'] == 'r' && gene_2['two'] == 'R') {  //Column
                const random_chance = Math.floor(Math.random() * (4 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'R' }
                }
                if (random_chance == 1 || random_chance == 2) {
                    return { 'one': 'R', 'two': 'r' }
                }
                if (random_chance == 3) {
                    return { 'one': 'r', 'two': 'r' }
                }
            }
        }

        // [ ][r] [R]
        // [r][Rr][rr]
        // [R][Rr][Rr]
        if (gene_1['one'] == 'r' && gene_1['two'] == 'R') {  //Row
            if (gene_2['one'] == 'r' && gene_2['two'] == 'R') {  //Column
                const random_chance = Math.floor(Math.random() * (4 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'R' }
                }
                if (random_chance == 1 || random_chance == 2) {
                    return { 'one': 'R', 'two': 'r' }
                }
                if (random_chance == 3) {
                    return { 'one': 'r', 'two': 'r' }
                }
            }
        }

        // [ ][R] [R]
        // [r][Rr][Rr]
        // [r][Rr][Rr]
        if (gene_1['one'] == 'R' && gene_1['two'] == 'R') {  //Row
            if (gene_2['one'] == 'r' && gene_2['two'] == 'r') { //Column
                return { 'one': 'R', 'two': 'r' }
            }
        }

        // [ ][r] [r]
        // [R][Rr][Rr]
        // [R][Rr][Rr]
        if (gene_1['one'] == 'r' && gene_1['two'] == 'r') {  //Row
            if (gene_2['one'] == 'R' && gene_2['two'] == 'R') { //Column
                return { 'one': 'R', 'two': 'r' }
            }
        }

        // [ ][R] [r]
        // [r][Rr][Rr]
        // [r][Rr][Rr]
        if (gene_1['one'] == 'R' && gene_1['two'] == 'r') {  //Row
            if (gene_2['one'] == 'r' && gene_2['two'] == 'r') {  //Column
                const random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'r' }
                }
                if (random_chance == 1) {
                    return { 'one': 'r', 'two': 'r' }
                }
            }
        }

        // [ ][r] [R]
        // [r][rr][Rr]
        // [r][rr][Rr]
        if (gene_1['one'] == 'r' && gene_1['two'] == 'R') { //  //Row
            if (gene_2['one'] == 'r' && gene_2['two'] == 'r') { //Column
                let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'r' }
                }
                if (random_chance == 1) {
                    return { 'one': 'r', 'two': 'r' }
                }
            }
        }

        // [ ][r] [r]
        // [R][Rr][Rr]
        // [r][rr][rr]
        if (gene_1['one'] == 'r' && gene_1['two'] == 'r') {  //Row
            if (gene_2['one'] == 'R' && gene_2['two'] == 'r') {  //Column
                let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'r' }
                }
                if (random_chance == 1) {
                    return { 'one': 'r', 'two': 'r' }
                }
            }
        }

        // [ ][r] [r]
        // [r][rr][rr]
        // [R][Rr][Rr]
        if (gene_1['one'] == 'r' && gene_1['two'] == 'r') {  //Row
            if (gene_2['one'] == 'r' && gene_2['two'] == 'R') {  //Column
                let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
                if (random_chance == 0) {
                    return { 'one': 'R', 'two': 'r' }
                }
                if (random_chance == 1) {
                    return { 'one': 'r', 'two': 'r' }
                }
            }
        }

        return this.randomAllele();
    }

    randomAlleleHeterosexual(): Gene {
        const alleles: Gene[] = [{
            'one': 'R',
            'two': 'R'
        }, {
            'one': 'R',
            'two': 'r'
        }];
        const num = Math.floor(Math.random() * (2 - 0) + 0);
        return alleles[num];
    }

    randomAllele(): Gene {
        const alleles: Gene[] = [{
            'one': 'R',
            'two': 'R'
        }, {
            'one': 'R',
            'two': 'r'
        }, {
            'one': 'r',
            'two': 'r'
        }];
        const num = Math.floor(Math.random() * (3 - 0) + 0);
        return alleles[num];
    }
}