# Simulating homosexuality as a Mendelian recessive gene
*Author:* [Jake S. Walklate](https://www.linkedin.com/in/svnty)

*Topics:* Bioinformatics, Same-sex attraction, Genetics, Population simulation

## Abstract

Homosexuality was simulated as a genetic trait using custom software. Each year in the simulation a random percent of the population within a range would produce offspring, the laws of segregation were applied to the offspring using the parents' genome and homosexuals would not breed and pass on their genes. The simulator produced results indicating that if homosexuality was the condition of having recessive alleles for all genes involved in homosexuality the number of homosexuals averaged at around 0.08% ± 0.075% of the population. This result implies there is a large sociological or psychological aspect to same-sex attraction. 

## Introduction

Five autosomal loci have been discovered as significantly associated with same-sex attraction (Ganna et al, 2019). Meaningful predictions of someone's sexuality from genetic loci are not producible, indicating large individual variance and response to inherited traits, however same-sex sexual behavior can be observed to run in families and genetically identical twins. Yearly increasing homosexuality rates put into question the assertion that individuals are born exclusively attracted to the same sex and opens the discussion on the environmental influences on same-sex sexual behavior. The initial assumption was that a subset of the species that did not reproduce would be removed from the gene pool and eventually lead to extinction. Software was created to estimate the potential pool size for genetically and exclusively homosexual individuals assuming all autosomal loci were recessive and to approximate the year of extinction of homosexuality if they were excluded from the gene pool, discussed further in the methods section.

## Methods

The software was developed using rust as a programming language for its object orientation and low resource usage. A segment of the genome was isolated and it was considered to be a person's entire genome, excluding genes unrelated to same-sex attraction. Initial variables were set such as a breeding range, a death range, and an offspring number which was calculated as a percent of the total alive population. Each year a person would increment their age by one year and if their age exceeded the death range they would die and be excluded from the gene pool with a 50% chance of someone dying each year they live past 70. Two eligible parents were randomly selected who were within the age range of 20 to 50 from the alive population to create one child. This was repeated until the number of offspring equaled the number of calculated expected offspring, then the next year would commence. The number of births was selected randomly with an upper bound of 1.75% and a lower bound of 1.3% of the total alive population each year (Roser, 2013). Several scenarios were simulated with the parameters indicated in the following sections. Initially, each simulation started with a population of 10,000 people with random alleles assigned to each gene in their genome.

### Scenario One

To be a homosexual, an individual had to be recessive in all genes respective to their gender-related to same-sex attraction (4 for males, 3 for females). An individual who was recessive in all genes had a 100% chance of being a homosexual, an individual who was either heterozygous or homozygous dominant at any location in the genome was not a homosexual.

### Scenario Two

To be a homosexual an individual had to be recessive in at least one gene associated with same-sex sexual behavior. Each genetic correlation value was taken from the GWAS investigation conducted by A. Ganna et al, these values were used as a threshold for a random number generator, if a gene was associated with same-sex sexual behavior with a genetic correlation of 0.2, then a series calculation was peformed with the genetic correlation and a random number was generated and compared to the probability of failure, if the random number was larger than the probability of failure the individual was considered to be a homosexual. If multiple genes were present then the thresholds were calculated by multiplying the genetic correlation in series, then the threshold was compared to a number randomly generated between 0 and 1.

## Scenario Three

To be a homosexual, the same conditions as scenario two were established, however in this simulation every gene that was homozygous recessive had environmental pressures, an added 0.2 (20%) was added in series to every calculation and a random number was generated between 0 and 1 to compare with the series, if the number generated was greater than the chance of success the person was considered homosexual and would not reproduce.

#### Series calculation

*a = genetic correlation gene 1*
*b = genetic correlation gene 2*
*(1-a)(1-b) = x*
*random_number = y [where 0<y<1]*
*if y > x then homosexual = true*

## Results

### Scenario One

![1.3 Billion people, no chance of homosexual breeding](/results/scenario_3/1_3billion_percent.png?raw=true)

### Scenario Two

![500 Million people, no chance of homosexual breeding](/results/scenario_2/500m_percent.png?raw=true)

## Scenario Three

![500 Million people, no chance of homosexual breeding](/results/scenario_3/500m_percent.png?raw=true)

## Discussion

The simulation takes into account no specific sociological or psychological contribution to same-sex attraction, the data from the results act as a baseline for the potential number of genetically determined homosexuals within a population, assuming an individual who is heterozygous in all five active loci was a homosexual. Assumptions, however, are intrinsically inaccurate and provide us no further insight into the underlying biological activity. Significant limitations of this simulator were the lack of long-term relationships between individuals, a couple would breed once and then disassociate and put back into the list of possible mates for the next generation, not exclusionary or creating multiple offspring with similar genes. 

## Conclusion

Same-sex sexual attraction is a biologically complex manifestation co-factored by polygenetic influences and environmental factors. The levels of exclusively recessive genetic individuals remained steady at below 1% of the population and did not show signs of convergence towards zero but instead maintained an equilibrium at about 0.06%, comparison of this result to the percentage of individuals identifying as members of the LGBT community in the broader community shows a disparity. Accumulated research further into the biological activity and environmental influences in same-sex sexual behavior may provide stronger variables for use in future population simulations, however strictly genetic contributions prove to be an inaccurate method of predictability for same-sex sexual behavior.

## Funding

This project received no funding.

## Assets 
[https://github.com/svnty/Homosexuality-Simulator](https://github.com/svnty/Homosexuality-Simulator)

## References
Ganna, A., Verweij, K. J. H., Nivard, M. G., Maier, R., Wedow, R., Busch, A. S., Abdellaoui, A., Guo, S., Sathirapongsasuti, J. F., Lichtenstein, P., Lundström, S., Långström, N., Auton, A., Harris, K. M., Beecham, G. W., Martin, E. R., Sanders, A. R., Perry, J. R. B., Neale, B. M., & Zietsch, B. P. (2019). Large-scale GWAS reveals insights into the genetic architecture of same-sex sexual behavior. Science, 365(6456), eaat7693. https://doi.org/10.1126/science.aat7693

Roser, M. (2013). Future Population Growth. Our World in Data. https://ourworldindata.org/future-population-growth