# Simulating homosexuality as a Mendelian recessive gene
*Author:* [Jake S. Walklate](https://www.linkedin.com/in/svnty)

*Topics:* Bioinformatics, Same-sex attraction, Genetics

## Abstract
Homosexuals have long assumed their sexual proclivities are biologically grounded. A previous genome wide association study identified 5 genes involved in homosexuality, 2 are gender-neutral, 2 are male associated and 1 is female associated, for a male to be homosexual the highest probability will come when all 4 of the genes that are involved in the sexual preference are present, and for women all 3. A simulation was run which investigated the effect of breeding with these genes, all five genes were assumed to be recessive and to be a homosexual a male needed to be homozygous in all 4 genes, for example. The law of segregation was applied to all offspring with the calculated chance of their children inheriting genes from the parent's and homosexuals were given a chance to reproduce each generation of offspring. The data showed that homosexuals were extinct by generation 231. The simulation continued for 50 generations to check for a revival of the homosexual genes, no such case occurred.

## Introduction
A previous study by Ganna et al., identified that same-sex sexual behaviour is partly genetically influenced, this study identified that same-sex sexual behaviour is not influenced by one gene but several genes overlapping, one such gene is associated with personality trait openness to experience. In their analysis a large data set (n=477,522) of genome-wide association studies (GWAS) was used to identify genes that have a statistical significance involved in same-sex sexual behaviour, a total of 5 autosomal gene loci were identified, 4 were significant in males and 3 in females, indicating that between the sexes there are genetic differences in same-sex sexual attraction. The gender non-specific loci were rs11114975-12q21.31 and rs10261857-7q31.2, the male-specific loci were rs28371400-15q21.3 and rs34730029-11q12.1 and in women the significant loci was rs11114975. Each gene has a unique function and many roles involved in other functions of the body such as sensitivity to scents or male pattern balding, indicating that these genes are not isolated to sexual attraction. Also recorded in the research provided by Ganna et al., was the descriptive statistics of the prevalence of homosexuality as a sexual preference indicating that same-sex sexual behaviour is steadily increasing among populations, according to Darwinian natural selection genes are selected according to their fitness for survival and reproduction of a species, homosexuals not being able to pass on their genes reproductively would be negatively selected for survival. Proceeding with this information, a simulation was run to investigate the inheritance of homosexuality through a population, genetics would indicate to us that if a homosexual parent has a child there is a significant chance that the child would be a homosexual and continue homosexuality as a genotype, homosexual genes were assumed to be recessive due to the prevalence of people with same-sex sexual attraction at about 2 to 10% of the population, indicating that it is not a dominant mode of survival.

## Methods
A simulation was programmed using no pre-packaged software and ran using Node.js. The prototype of the simulator was written in python, but due to the asynchronous nature of python, the iterations of generations would be executed randomly causing problems with the linear time assumed in the simulation. Each iteration of the program was evaluated to be a new generation; in a generation, humans would either breed, die or idle and age 1 year. Initial variables of significance were identified such as breeding age and death age and these were modifiable in between executions of the software. The breeding range of humans was set to be between the ages of 20 and 50 and during each iteration of the simulation between 1.6 to 2% of the population would breed and create 1 offspring per couple to maintain population growth. Couples were selected randomly assuming three criteria of eligibility, they had to be alive and between the breeding age ranges and of the opposite sex, if a couple was selected for breeding and they were the same sex and either was homosexual they could enter a random event. Random events were created to correct for the possibility of homosexuals breeding heterosexually or bisexually, initially, the intentions were to have a high random event chance and as we approached homosexual legality the number of random events would decrease to assume for more homosexual marriages and this would occur by modifying the variable involved in the chance of random events occurring as progress towards marriage equality increased, this random event variable was set to a 50% chance of creating offspring and remained at 50% for the entire simulation, the random event also included the chances of IVF or surrogacy from the homosexual parent's gametes. During each iteration, a human between the ages of 70 and 90 had a 50% chance of death to remove them from the total number of alive homosexuals, this was important because as the number of homosexuals reached 0 the simulation would begin the countdown to stop. The countdown was also known as the run-off generations, as a generic number 50 run-off generations were chosen, however, sometimes it was set as high as 250 generations to check for a revival of homosexuality in the gene pool. During each generation when an offspring was conceived Mendel's law of segregation was applied and the genotype of the offspring was calculated using Punnett square analysis from the parent's genetic information.  A generation would end when the selected number of offspring had been created and a random event had at least occurred once. The simulation was run until homosexual extinction and the run-off period had elapsed.

## Results

The population increase was exponential and initially the random event offspring were born at the same rate as heterosexual offspring both in proportion to the increase in population but the paths beginning to separate at about generation 30 when the large initial homosexual population exits the breeding range, then the number of random event children fell below the number of heterosexual offspring. Random event children remained a slow but steady number of the births per generation until the homosexual population was diminished. In the example in figure 1 by generation 231 the homosexual population was entirely extinct and at generation 195 the last random event child was born, this last random event child was 36 years old at the time of homosexual extinction, indicating that it was a heterozygote and not a homosexual. Random even children were a low but steady number until the homosexual population diminished.
 
![Final results](https://raw.githubusercontent.com/svnty/Homosexuality-Simulator/main/results/forced_breeding_50percent.png)

**Figure 1:** The data shows us a clear exponential increase in the total population, as the total population increases the number of children also scales to remain at between 1.6 to 2% of the total population as expected. The number of homosexuals initially at 50% of the population decreases rapidly at year 50.

Variables were systematically randomized, sometimes to reflect real world data and sometimes to extend the lifetime of homosexuals for as long as possible to discover new data patterns, all results approximated towards the figure 1 data. The results of the randomized 
configurations can be seen in table 1.

![Table results](https://raw.githubusercontent.com/svnty/Homosexuality-Simulator/main/results/TABLE_DATA.png) 

**Table 1:** The table details the starting variable configuration of the software and the results from the run, in all configurations the homosexual genes were negatively selected for the survival.

A final simulation adjusted for sociological or psychological factors influencing same-sex sexual behaviour, in this simulation if any single of the four for male or three for female genes present was homozygous recessive there was an added 10% chance of the person being a homosexual. This was the longest of the simulations, surviving for the most generations in all variable settings. In this simulation males had the highest chance of homosexuality by having the larger number of genes, if three genes of four genes were present there was a 30% chance of same-sex preferences in the individual, however, if all four genes were present there was still a 100% chance of same-sex sexual behaviour, results of this simulation are available in figure 2. A random event chance of 50% was initiated and an approximate 20% starting homosexual population. 

![Sociological adjust](https://raw.githubusercontent.com/svnty/Homosexuality-Simulator/main/results/sociological.png)

**Figure 2:** A simulation adjusted for sociological, psychological and environmental factors increasing the chances of homosexual same-sex behaviour. This simulation required 11GB of memory and completed at generation 419 with a starting population of 10,000 and an ending total population of 17,689,762. 

## Discussion

*Implications of the intended research*

The simulation ended abruptly, the simulation was intended to be run for 100,000's of years to get an accurate representation of the entire evolution of the human species and then to adjust variables slowly to match the progressive nature of humans and the changing social environments, however, the extinction occurred so rapidly a generation past a few hundred years could never be reached. Also intended was to start the simulation at a population of 7 billion and a starting percent of homosexuals of between 5 to 10% of the population and stimulate the scenario of a modern world, however with limited memory available this was not possible, on a machine with more memory or a more efficient language such as C++ this may be achievable. 

*Significance of data*

These results indicate what would be expected from a non-breeding species, the number of the existing genotypes within that species decreases and the genotype with a higher chance of survival increases.

*Future research*

Future research should adjust for the proximity of a people and their in-group preference to associate with people of similar culture and assumed normal practices of themselves, such that a homosexual population would likely form a dense community and random events could occur more likely within that community. Current simulation results assume homosexuals and heterosexuals are evenly distributed throughout a population and people are selected to mate regardless of their geographical distance. A sociological and statistical analysis of the rate of breeding among homosexuals and bisexuals could provide stronger data for the variables in a simulation.  

## Conclusions
A rate of 50% breeding of homosexuals isn't a high enough number for them to continue their genes, even with a high initial homosexuality rate of 50% of a population, the number of homosexuals decreases so quickly they become extinct within a few generations. Future simulations could more accurately represent the breeding patterns by using variables that more closely represent the real world. Considerably the chances of homosexuality occurring and homosexuals breeding used in the variables of the simulation was likely larger than the real world.

## Assets 
[https://github.com/svnty/Homosexuality-Simulator](https://github.com/svnty/Homosexuality-Simulator)

## References
Ganna, A., Verweij, K. J. H., Nivard, M. G., Maier, R., Wedow, R., Busch, A. S., Abdellaoui, A., Guo, S., Sathirapongsasuti, J. F., Lichtenstein, P., Lundström, S., Långström, N., Auton, A., Harris, K. M., Beecham, G. W., Martin, E. R., Sanders, A. R., Perry, J. R. B., Neale, B. M., & Zietsch, B. P. (2019). Large-scale GWAS reveals insights into the genetic architecture of same-sex sexual behavior. Science, 365(6456), eaat7693. https://doi.org/10.1126/science.aat7693