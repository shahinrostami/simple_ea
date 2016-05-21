# simple_ea
Simple Multi-Objective Evolutionary Algorithm (MOEA) implemented in the Rust programming language.

Solves the ZDT1 multi-objective synthetic test function (30 problem variables, 2 problem objectives).

Fixed parent population size of 100, with 100 offspring solutions produced and evaluated at each generation. Offspring can only replace their parent, and only if they are dominant.

The only method to variation is gaussian mutation.
