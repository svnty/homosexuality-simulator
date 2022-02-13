const fs = require('fs');

let human_count = 0;
let generation = 0;
let homosexual_mates = 0;
let homosexual_forced_mate = false;
let humans = [];
let donors = [];

const starting_number_of_people = 1_000;
const starting_percent_as_LGBT = 0.5;
const children_percent_of_generation = [0.02, 0.016];
const random_event_range = [2, 1];
const death_range = [90, 70];
const run_off_generation = 50;
const breed_range = [20, 50];
const sociological = true;
const sociological_chance = 0.3;

class person {
    constructor(parent_1, parent_2, age = 0, homosexual = false, donor = false) {
        this.human_id = human_count;
        human_count += 1;
        this.parent_1 = parent_1;
        this.parent_2 = parent_2;
        this.birth_generation = generation;
        this.homosexual = homosexual;
        this.dead = false;
        this.age = age;
        this.donor = donor;

        if (Math.random() > 0.5) {
            this.gender = 'M';
        } else {
            this.gender = 'F';
        }

        this.gwas = {
            'rs11114975-12q21.31': {
                'gender': 'ALL',
                'allele': {
                    'one': '',
                    'two': ''
                }
            },
            'rs10261857-7q31.2': {
                'gender': 'ALL',
                'allele': {
                    'one': '',
                    'two': ''
                }
            },
            'rs28371400-15q21.3': {
                'gender': 'M',
                'allele': {
                    'one': '',
                    'two': ''
                }
            },
            'rs34730029-11q12.1': {
                'gender': 'M',
                'allele': {
                    'one': '',
                    'two': ''
                }
            },
            'rs13135637-4p14': {
                'gender': 'F',
                'allele': {
                    'one': '',
                    'two': ''
                }
            }
        };

        if (parent_1 && parent_2) {
            let parent_1_rs11114975_12q21_31 = parent_1.gwas['rs11114975-12q21.31']['allele'];
            let parent_2_rs11114975_12q21_31 = parent_2.gwas['rs11114975-12q21.31']['allele'];
            this.gwas['rs11114975-12q21.31']['allele'] = lawOfSeggregation(parent_1_rs11114975_12q21_31, parent_2_rs11114975_12q21_31);
            let parent_1_rs10261857_7q31_2 = parent_1.gwas['rs10261857-7q31.2']['allele'];
            let parent_2_rs10261857_7q31_2 = parent_2.gwas['rs10261857-7q31.2']['allele'];
            this.gwas['rs10261857-7q31.2']['allele'] = lawOfSeggregation(parent_1_rs10261857_7q31_2, parent_2_rs10261857_7q31_2);
            let parent_1_rs28371400_15q21_3 = parent_1.gwas['rs28371400-15q21.3']['allele'];
            let parent_2_rs28371400_15q21_3 = parent_2.gwas['rs28371400-15q21.3']['allele'];
            this.gwas['rs28371400-15q21.3']['allele'] = lawOfSeggregation(parent_1_rs28371400_15q21_3, parent_2_rs28371400_15q21_3);
            let parent_1_rs34730029_11q12_1 = parent_1.gwas['rs34730029-11q12.1']['allele'];
            let parent_2_rs34730029_11q12_1 = parent_2.gwas['rs34730029-11q12.1']['allele'];
            this.gwas['rs34730029-11q12.1']['allele'] = lawOfSeggregation(parent_1_rs34730029_11q12_1, parent_2_rs34730029_11q12_1);
            let parent_1_rs13135637_4p14 = parent_1.gwas['rs13135637-4p14']['allele'];
            let parent_2_rs13135637_4p14 = parent_2.gwas['rs13135637-4p14']['allele'];
            this.gwas['rs13135637-4p14']['allele'] = lawOfSeggregation(parent_1_rs13135637_4p14, parent_2_rs13135637_4p14);
        } else {
            this.gwas['rs11114975-12q21.31']['allele'] = lawOfSeggregation(randomAllele(), randomAllele());
            this.gwas['rs10261857-7q31.2']['allele'] = lawOfSeggregation(randomAllele(), randomAllele());
            this.gwas['rs28371400-15q21.3']['allele'] = lawOfSeggregation(randomAllele(), randomAllele());
            this.gwas['rs34730029-11q12.1']['allele'] = lawOfSeggregation(randomAllele(), randomAllele());
            this.gwas['rs13135637-4p14']['allele'] = lawOfSeggregation(randomAllele(), randomAllele());
        }

        // rs11114975-12q21.31 [ALL]
        if (this.gwas['rs11114975-12q21.31']['allele']['one'] == 'r') {
            if (this.gwas['rs11114975-12q21.31']['allele']['two'] == 'r') {
                if (Math.random() <= sociological_chance && sociological == true) {
                    this.homosexual = true;
                }
                // rs10261857-7q31.2 [ALL]
                if (this.gwas['rs10261857-7q31.2']['allele']['one'] == 'r') {
                    if (this.gwas['rs10261857-7q31.2']['allele']['two'] == 'r') {
                        if (Math.random() <= sociological_chance && sociological == true) {
                            this.homosexual = true;
                        }
                        if (this.gender == 'M') {
                            // rs28371400-15q21.3 [MALE ONLY]
                            if (this.gwas['rs28371400-15q21.3']['allele']['one'] == 'r') {
                                if (this.gwas['rs28371400-15q21.3']['allele']['two'] == 'r') {
                                    if (Math.random() <= sociological_chance && sociological == true) {
                                        this.homosexual = true;
                                    }
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
                                    if (Math.random() <= sociological_chance && sociological == true) {
                                        this.homosexual = true;
                                    }
                                    this.homosexual = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        if (Math.random() <= sociological_chance && sociological == true) {
            if (this.gwas['rs10261857-7q31.2']['allele']['one'] == 'r') {
                if (this.gwas['rs10261857-7q31.2']['allele']['two'] == 'r') {
                        this.homosexual = true;
                }
            }
        }


    }
}

function randomAllele() {
    alleles = [{
        'one': 'R',
        'two': 'R'
    }, {
        'one': 'R',
        'two': 'r'
    }, {
        'one': 'r',
        'two': 'r'
    }];
    let num = Math.floor(Math.random() * (3 - 0) + 0);
    return alleles[num];
}

function lawOfSeggregation(gene_1, gene_2) {
    // [ ][R] [R]
    // [R][RR][RR]
    // [R][RR][RR]
    if (gene_1['one'] == 'R' && gene_1['two'] == 'R') {  //Row
        if (gene_2['one'] == 'R' && gene_2['two'] == 'R')  //Column
            return { 'one': 'R', 'two': 'R' }
    }

    // [ ][r] [r]
    // [r][rr][rr]
    // [r][rr][rr]
    if (gene_1['one'] == 'r' && gene_1['two'] == 'r') {  //Row
        if (gene_2['one'] == 'r' && gene_2['two'] == 'r')  //Column
            return { 'one': 'r', 'two': 'r' }
    }

    // [ ][R] [R]
    // [R][RR][RR]
    // [r][Rr][Rr]
    if (gene_1['one'] == 'R' && gene_1['two'] == 'R') {  //Row
        if (gene_2['one'] == 'R' && gene_2['two'] == 'r') { //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'R' }
            if (random_chance == 1)
                return { 'one': 'R', 'two': 'r' }
        }
    }

    // [ ][R] [R]
    // [r][Rr][Rr]
    // [R][RR][RR]
    if (gene_1['one'] == 'R' && gene_1['two'] == 'R') {  //Row
        if (gene_2['one'] == 'r' && gene_2['two'] == 'R') { //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'R' }
            if (random_chance == 1)
                return { 'one': 'R', 'two': 'r' }
        }
    }

    // [ ][R] [r]
    // [R][Rr][Rr]
    // [R][RR][RR]
    if (gene_1['one'] == 'R' && gene_1['two'] == 'r') {  //Row
        if (gene_2['one'] == 'R' && gene_2['two'] == 'R') {  //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'R' }
            if (random_chance == 1)
                return { 'one': 'R', 'two': 'r' }
        }
    }

    // [ ][r] [R]
    // [R][Rr][RR]
    // [R][Rr][RR]
    if (gene_1['one'] == 'r' && gene_1['two'] == 'R') { //Row
        if (gene_2['one'] == 'R' && gene_2['two'] == 'R') {  //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'R' }
            if (random_chance == 1)
                return { 'one': 'R', 'two': 'r' }
        }
    }

    // [ ][R] [r]
    // [R][RR][Rr]
    // [r][Rr][rr]
    if (gene_1['one'] == 'R' && gene_1['two'] == 'r') {  //Row
        if (gene_2['one'] == 'R' && gene_2['two'] == 'r') {  //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'R' }
            if (random_chance == 1 || random_chance == 2)
                return { 'one': 'R', 'two': 'r' }
            if (random_chance == 3)
                return { 'one': 'r', 'two': 'r' }
        }
    }

    // [ ][r] [R]
    // [R][Rr][RR]
    // [r][Rr][rr]
    if (gene_1['one'] == 'r' && gene_1['two'] == 'R') {  //Row
        if (gene_2['one'] == 'R' && gene_2['two'] == 'r') {  //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'R' }
            if (random_chance == 1 || random_chance == 2)
                return { 'one': 'R', 'two': 'r' }
            if (random_chance == 3)
                return { 'one': 'r', 'two': 'r' }
        }
    }

    // [ ][R] [r]
    // [r][Rr][rr]
    // [R][Rr][Rr]
    if (gene_1['one'] == 'R' && gene_1['two'] == 'r') {  //Row
        if (gene_2['one'] == 'r' && gene_2['two'] == 'R') {  //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'R' }
            if (random_chance == 1 || random_chance == 2)
                return { 'one': 'R', 'two': 'r' }
            if (random_chance == 3)
                return { 'one': 'r', 'two': 'r' }
        }
    }

    // [ ][r] [R]
    // [r][Rr][rr]
    // [R][Rr][Rr]
    if (gene_1['one'] == 'r' && gene_1['two'] == 'R') {  //Row
        if (gene_2['one'] == 'r' && gene_2['two'] == 'R') {  //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'R' }
            if (random_chance == 1 || random_chance == 2)
                return { 'one': 'R', 'two': 'r' }
            if (random_chance == 3)
                return { 'one': 'r', 'two': 'r' }
        }
    }

    // [ ][R] [R]
    // [r][Rr][Rr]
    // [r][Rr][Rr]
    if (gene_1['one'] == 'R' && gene_1['two'] == 'R') {  //Row
        if (gene_2['one'] == 'r' && gene_2['two'] == 'r')  //Column
            return { 'one': 'R', 'two': 'r' }
    }

    // [ ][r] [r]
    // [R][Rr][Rr]
    // [R][Rr][Rr]
    if (gene_1['one'] == 'r' && gene_1['two'] == 'r') {  //Row
        if (gene_2['one'] == 'R' && gene_2['two'] == 'R') //Column
            return { 'one': 'R', 'two': 'r' }
    }

    // [ ][R] [r]
    // [r][Rr][Rr]
    // [r][Rr][Rr]
    if (gene_1['one'] == 'R' && gene_1['two'] == 'r') {  //Row
        if (gene_2['one'] == 'r' && gene_2['two'] == 'r') {  //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'r' }
            if (random_chance == 1)
                return { 'one': 'r', 'two': 'r' }
        }
    }

    // [ ][r] [R]
    // [r][rr][Rr]
    // [r][rr][Rr]
    if (gene_1['one'] == 'r' && gene_1['two'] == 'R') { //  //Row
        if (gene_2['one'] == 'r' && gene_2['two'] == 'r') { //Column
            let random_chance = random.randint(0, 1)
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'r' }
            if (random_chance == 1)
                return { 'one': 'r', 'two': 'r' }
        }
    }

    // [ ][r] [r]
    // [R][Rr][Rr]
    // [r][rr][rr]
    if (gene_1['one'] == 'r' && gene_1['two'] == 'r') {  //Row
        if (gene_2['one'] == 'R' && gene_2['two'] == 'r') {  //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'r' }
            if (random_chance == 1)
                return { 'one': 'r', 'two': 'r' }
        }
    }

    // [ ][r] [r]
    // [r][rr][rr]
    // [R][Rr][Rr]
    if (gene_1['one'] == 'r' && gene_1['two'] == 'r') {  //Row
        if (gene_2['one'] == 'r' && gene_2['two'] == 'R') {  //Column
            let random_chance = Math.floor(Math.random() * (2 - 0)) + 0;
            if (random_chance == 0)
                return { 'one': 'R', 'two': 'r' }
            if (random_chance == 1)
                return { 'one': 'r', 'two': 'r' }
        }
    }
}

function randomEvent() {
    // chance of surrgoacy, ivf or reproducing heterosexually
    let chance = Math.floor(Math.random() * (random_event_range[0] - random_event_range[1])) + random_event_range[1];
    if (chance == 1) {
    // let chance = Math.random();
    // if (chance <= 0.75) {
        return true;
    }
    return false;
}

function getDonor(parent) {
    try {
        let donor_recent = donors.length - 1;
        let donor = donors[donor_recent];
        while (donor.gender == parent.gender && donor.dead == true) {
            donor_recent -= 1;
            donor = donors[donor_recent];
        }
        return donor;
    } catch (err) {
        console.log(err);
        return false;
    }
}

function mate(parent_1, parent_2) {
    // Homosexual couple
    // Using OR instead of AND to consider bisexuals
    if (parent_1.homosexual == true || parent_2.homosexual == true) {
        homosexual_forced_mate = true;
        if (randomEvent() == true) {
            let donor = getDonor(parent_1);
            if (donor != false) {
                homosexual_mates += 1;
                let new_human = new person(parent_1, parent_2);
                humans.push(new_human);
            }
        }
    }
    // Heterosexual couple
    if (parent_1.homosexual != true && parent_2.homosexual != true) {
        let new_human = new person(parent_1, parent_2);
        humans.push(new_human);
    }
}

function checkMate(parent_1, parent_2) {
    if (!parent_1 || !parent_2) {
        return false;
    }
    if (parent_1 != parent_2) {
        if (parent_1.age >= breed_range[0] && parent_2.age >= breed_range[0]) {
            if (parent_1.age <= breed_range[1] && parent_2.age <= breed_range[1]) {
                return true;
            }
        }
    }
    return false;
}

function checkExtinct(save) {
    let extinct = true;
    let homosexual_counter = 0;
    let person_counter = 0;
    for (let human of humans) {
        if (human.dead == false) {
            person_counter += 1;
            if (human.homosexual == true) {
                extinct = false;
                homosexual_counter += 1;
            }
        }
    }
    if (save) {
        write(homosexual_counter);
        write(person_counter);
        write(humans.length);
    }
    return extinct;
}

function childPerGeneration() {
    let max = Math.ceil(human_count * children_percent_of_generation[0]);
    let min = Math.ceil(human_count * children_percent_of_generation[1]);
    return [max, min];
}

function main() {
    while (humans.length < starting_number_of_people || checkExtinct()) {
        let homosexuality_random = Math.random() > starting_percent_as_LGBT ? 0 : 1;
        let donor_random = Math.floor(Math.random() * (2 - 0)) + 0;
        let new_human = new person(null, null, 21, homosexuality_random, donor_random);
        humans.push(new_human);
        if (new_human.donor) {
            donors.push(new_human)
        }
    }

    let run_off_count_down = 0;
    while (checkExtinct(true) == false || run_off_count_down < run_off_generation) {
        is_extinct = checkExtinct(false);
        if (is_extinct == true) {
            run_off_count_down += 1;
        }
        write('__');
        generation += 1;
        write(generation);
        for (let human of humans) {
            let random_age = Math.floor(Math.random() * (death_range[0] - death_range[1])) + death_range[1];
            if (human.age > random_age) {
                human.dead = true;
            } else {
                human.age += 1;
            }
        }

        let children = childPerGeneration();
        let max = children[0];
        let min = children[1];
        let child_counter = Math.floor(Math.random() * (max - min)) + min;
        let total_child_count = child_counter;
        homosexual_mates = 0;
        write(child_counter);
        homosexual_forced_mate = false;

        for (let i = 0; i < child_counter; i++) {
            let parent_1;
            let parent_2;
            while (checkMate(parent_1, parent_2) != true) {
                parent_1 = humans[Math.floor(Math.random() * humans.length)];
                parent_2 = humans[Math.floor(Math.random() * humans.length)];
            }
            mate(parent_1, parent_2);
        }

        let reverse_walker = humans.length - 1;
        // Forcing at least the chance of 1 homosexual mating per generation
        for (; ;) {
            if (homosexual_forced_mate == true) {
                break;
            }
            if (is_extinct == true) {
                break;
            }
            let breeding_age_gay = false;
            for (const gay_check of humans) {
                if (gay_check.homosexual) {
                    if (gay_check.age >= 20) {
                        if (!gay_check.dead) {
                            breeding_age = true;
                        }
                    }
                }
            }
            if (breeding_age_gay == false) {
                break;
            }
            let parent_1;
            let parent_2;
            while (checkMate(parent_1, parent_2) != true) {
                parent_1 = humans[Math.floor(Math.random() * humans.length)];
                parent_2 = humans[reverse_walker];
                console.log(reverse_walker);
                reverse_walker = reverse_walker - 1;
            }
            if (parent_1.homosexual == true || parent_2.homosexual == true) {
                total_child_count = child_counter + 1;
                mate(parent_1, parent_2);
            }
        }

        write(total_child_count);
        write(homosexual_mates);
    }
}

function write(line) {
    let text;
    if (line != '__') {
        text = line + ',';
    } else {
        text = '\n';
    }
    fs.appendFileSync('results.txt', text);
}

function initText() {
    fs.appendFileSync('results.txt', 'generation,calculated_children,children_created,random_event_child,homosexuals,alive_population,total_population\n');
    fs.appendFileSync('results.txt', '0,0,0,');
}

initText();
main();
console.log('Done');
process.exit();
