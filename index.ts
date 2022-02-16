import { appendFileSync, writeFileSync } from 'fs';
import { Person } from './assets/Person';
import CONFIG from './assets/configuration';

// Non configurables
let generation: number = 0;
let humans: Person[] = [];

// Running variables
let offspring: number = 0;
let alive_person_counter: number = 0;
let homosexual_counter: number = 0;
let random_event_counter: number = 0;
let random_events_expected: number = 0;
let random_event_offspring_counter: number = 0;
let homosexual_offspring_counter: number = 0;
let homosexual_percent = CONFIG.starting_lgbt_percent;
let offspring_counter = 0;
let run_off_counter: number = 0;
let homosexual_breeders_extinct: boolean = true;

function getDonor(parent_1: Person): Person | boolean {
    for (let i = humans.length - 1; i >= 0; i--) {
        let donor: Person = humans[i];
        if (donor.gender != parent_1.gender) {
            if (donor.dead != true) {
                if (donor.age >= CONFIG.breed_range[0]) {
                    if (donor.age <= CONFIG.breed_range[1]) {
                        return donor;
                    }
                }
            }
        }
    }
    return false;
}

function randomEvent(): boolean {
    if (Math.random() <= CONFIG.random_event_chance) {
        return true;
    }
    return false;
}

function mate(parent_1?: Person, parent_2?: Person): boolean {
    // Homosexual couple
    if (parent_1 && parent_2) {
        // OR because of a bisexual relationship
        if (parent_1.homosexual == true || parent_2.homosexual == true) {
            random_event_counter += 1;
            if (randomEvent() == true) {
                random_event_offspring_counter += 1;
                // Require a donor for same sex
                if (parent_1.gender == parent_2.gender) {
                    let donor = getDonor(parent_1);
                    if (donor != false) {
                        newHuman(parent_1,parent_2);
                        return true;
                    }
                // Proceed for heterosex bisexual
                } else if (parent_1.gender != parent_2.gender) {
                    newHuman(parent_1,parent_2);
                    return true;
                }
            }
        } else if (parent_1.homosexual == false && parent_2.homosexual == false) {
            newHuman(parent_1, parent_2);
            return true;
        }
        function newHuman(parent_1: Person, parent_2: Person) {
            let new_human = new Person(parent_1, parent_2, undefined, undefined, undefined, generation);
            offspring_counter += 1;
            humans.push(new_human);
            if (new_human.homosexual == true) {
                homosexual_offspring_counter += 1;
            }
        }
    }
    return false;
}

function checkMate(parent_1?: Person, parent_2?: Person): boolean {
    if (!parent_1 || !parent_2) {
        return false;
    }
    if (parent_1 == parent_2) {
        return false;
    }
    // Minimum age
    if (parent_1.age >= CONFIG.breed_range[0]) {
        if (parent_2.age >= CONFIG.breed_range[0]) {
            // Maximum age
            if (parent_1.age <= CONFIG.breed_range[1]) {
                if (parent_2.age <= CONFIG.breed_range[1]) {
                    return true;
                }
            }
        }
    }
    return false;
}

function checkExtinct(age: boolean): boolean {
    homosexual_breeders_extinct = true;
    let extinct = true;
    homosexual_counter = 0;
    alive_person_counter = 0;
    for (let human of humans) {
        if (age == true) {
            human.age += 1;
            let random_age: number = Math.floor(Math.random() * (CONFIG.death_range[0] - CONFIG.death_range[1])) + CONFIG.death_range[1];
            if (human.age > random_age) {
                human.dead = true;
            }
        }
        if (human.dead == false) {
            alive_person_counter += 1;
            if (human.homosexual == true) {
                extinct = false;
                homosexual_counter += 1;
                if (human.age >= CONFIG.breed_range[0]) {
                    if (human.age <= CONFIG.breed_range[1]) {
                        homosexual_breeders_extinct = false;
                    }
                }
            }
        }
    }
    return extinct;
}

function setChildPerGeneration(): { offspring: number, percent: number } {
    let upper = CONFIG.offspring_percent[0];
    let max = Math.floor(alive_person_counter * upper);
    let lower = CONFIG.offspring_percent[1];
    let min = Math.floor(alive_person_counter * lower);

    offspring = Math.floor(Math.random() * (max - min)) + max;
    let percent = offspring / (alive_person_counter)
    return {
        offspring: offspring,
        percent: (isNaN(percent) ? 0 : percent)
    }
}

function write(): void {
    let text = generation + ',' + offspring + ',' + offspring_counter + ',' + random_events_expected + ',' + random_event_counter + ',' + random_event_offspring_counter + ',' + homosexual_counter + ',' + homosexual_percent +  ',' + alive_person_counter + ',' + humans.length + '\n';
    appendFileSync('results.txt', text);
}

function initText(): void {
    writeFileSync('results.txt', JSON.stringify(CONFIG) + '\n');
    appendFileSync('results.txt', 'Generation,Expected Offspring,Total Offspring,Random Events Expected,Random Events Triggered,Random Event Offspring (Homosexual Parent) [IVF/Surrogacy/Bi-Sexual],Alive Homosexuals,Homosexuals as percent of population,Alive Population,Total Population (Alive + Dead)\n');
    let text = 0 + ',' + 0 + ',' + 0 + ',' + 0 + ',' + 0 + ',' + 0 + ',' + (humans.length * CONFIG.starting_lgbt_percent) + ',' + homosexual_percent + ',' + humans.length + ',' + humans.length + '\n';
    appendFileSync('results.txt', text);
}

function initPopulation(): void {
    // Create homosexuals
    for (let i = 0; i < (CONFIG.starting_population * CONFIG.starting_lgbt_percent); i++) {
        let donor_random: boolean = (Math.random() <= 0.5);
        let age: number = Math.floor(Math.random() * (50 - 1) + 1);
        let new_human: Person = new Person(undefined, undefined, age, true, donor_random);
        humans.push(new_human);
    }

    // Create heterosexuals
    while (humans.length < CONFIG.starting_population) {
        let donor_random: boolean = (Math.random() <= 0.5);
        let age: number = Math.floor(Math.random() * (50 - 1) + 1);
        let new_human = new Person(undefined, undefined, age, false, donor_random);
        humans.push(new_human);
    }
}

function main(): void {
    initPopulation();
    initText();
    while (checkExtinct(false) == false || run_off_counter < CONFIG.run_off) {
        // Increment generation
        generation += 1;
        // Reset variables
        offspring_counter = 0;
        homosexual_offspring_counter = 0;
        random_event_counter = 0;
        random_event_offspring_counter = 0;
        random_events_expected = 0;

        // Check extinct
        const is_extinct = checkExtinct(true);
        if (is_extinct) {
            run_off_counter += 1;
        }

        // Calculate children
        let child_config = setChildPerGeneration();
        let reverse_walker = humans.length - 1;

        // Create children
        for (let i = 0; i < child_config.offspring; i++) {
            let parent_1!: Person;
            let parent_2!: Person;
            while (checkMate(parent_1, parent_2) == false) {
                parent_1 = humans[Math.floor(Math.random() * humans.length)];
                parent_2 = humans[reverse_walker];
                reverse_walker -= 1;
                // Reset walker due to random parent 1
                if (reverse_walker <= 0) {
                    reverse_walker = humans.length - 1;
                }
            }
            mate(parent_1, parent_2);
        }

        // Force random events to occur at the same rate of children expected   
        homosexual_percent = homosexual_counter / alive_person_counter;
        random_events_expected = Math.ceil(alive_person_counter * child_config.percent * homosexual_percent);

        // If there is a large gene pool and a small number of homosexuals this event
        // will trigger to adjust for in group preferential selection of communities and promixity
        while (random_event_counter < random_events_expected && homosexual_breeders_extinct == false) {
            let parent_1: Person | undefined;
            let parent_2: Person | undefined;

            while (checkMate(parent_1, parent_2) == false) {
                parent_1 = humans[Math.floor(Math.random() * humans.length)];
                parent_2 = humans[reverse_walker];
                reverse_walker = reverse_walker - 1;
                // Reset walker due to random parent 1
                if (reverse_walker <= 0) {
                    reverse_walker = humans.length - 1;
                }
            }
            if (parent_1 && parent_2) {
                if (parent_1.homosexual == true || parent_2.homosexual == true) {
                    mate(parent_1, parent_2);
                }
            }
        }
        write();
    }
    console.log('done');
    process.exit();
}

main();
