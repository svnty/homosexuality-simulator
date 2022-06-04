# Simulating homosexuality as a mendelian recessive gene
*Author:* [Jake S. Walklate](https://www.linkedin.com/in/svnty)

*Topics:* Bioinformatics, Same-sex attraction, Genetics, Population simulation

## Abstract

Homosexuality was simulated as a genetic trait using custom software. Each year in the simulation a random percent of the population within a range would produce offspring, homosexuals would not breed and pass on their genes, the laws of seggregation was applied to the offspring using the parents genome. The simulator produced results indicating that if homosexuality was the condition of having recessive alleles for all genes involved in homosexuality the number of homosexuals averaged at around 0.08% of the population. This result implys there is a large sociological or psychological aspect to same-sex attraction. 

## Introduction



## Methods

Software was developed using rust as a programming language for it's object orientation and low resource usage. A segment of the genome was isolated and it was considered to be a persons entire genome, excluding genes unrelated to same-sex attraction. Initial variables were set such as a breeding range, a death range and an offspring number which was calculated a as a percent of the total alive population. Each year a person would incriment their age by one year and if their age exceeded the death range they would die and be excluded from the gene pool with a 50% chance of someone dying each year they live past 70. Two eligible parents were randomly selected who were within the age range of 20 to 50 from the alive population to create one child. This was repeated until the number of offspring equaled the number of calculated expected offspring, then the next year would commence. A number of scenarios were simulated indicated in the following sections. Initally each simulation started with a population of 10,000 people with random alleles assigned to each gene in their genome. 

### Scenario One

To be a homosexual, an individual had to be recessive in all genes respective for their gender related to same-sex attraction (4 for males, 3 for females). An individual who was recessive in all genes had a 100% chance of being a homosexual, an indivudal who did not have all recessive alleles for the entire genome was not a homosexual.

### Scenario Two

To be a homosexual an individual had to be recessive in at least one gene associated with same-sex sexual behavior. Each R squared value was taken from the GWAS investigation conducted by A. Ganna et al, these R squared values were used as a threshold for a random number generator, if a gene was associated with same-sex sexual behavior with an R squared of 0.2, then any number randomly generated higher than 0.2 and the individual was considered to be homoesexual.

## Results

![1.3 Billion people, no chance of homosexual breeding](/img/1_3billion_percent.png?raw=true)

## Assets 
[https://github.com/svnty/Homosexuality-Simulator](https://github.com/svnty/Homosexuality-Simulator)

## References
Ganna, A., Verweij, K. J. H., Nivard, M. G., Maier, R., Wedow, R., Busch, A. S., Abdellaoui, A., Guo, S., Sathirapongsasuti, J. F., Lichtenstein, P., Lundström, S., Långström, N., Auton, A., Harris, K. M., Beecham, G. W., Martin, E. R., Sanders, A. R., Perry, J. R. B., Neale, B. M., & Zietsch, B. P. (2019). Large-scale GWAS reveals insights into the genetic architecture of same-sex sexual behavior. Science, 365(6456), eaat7693. https://doi.org/10.1126/science.aat7693