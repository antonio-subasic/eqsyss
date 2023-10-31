eqsyss is an [equation-system](https://en.wikipedia.org/wiki/System_of_equations)-solver written in TypeScript

# Requirements

- [Node.js](https://nodejs.org/en/)
- [TypeScript](https://www.typescriptlang.org/)

# Setup

1. Download
    - [Download the Source Code](https://github.com/antonio-subasic/eqsyss/archive/refs/heads/main.zip) or run `git clone https://github.com/antonio-subasic/eqsyss.git`
    - Run `npm install` in the *eqsyss* directory to install all dependencies

1. Compile the program with `tsc`

1. Run the program with `node ./dist/eqsyss.js <input file>`

# Usage

The program reads an equation-system in the form of an [augmented matrix](https://en.wikipedia.org/wiki/Augmented_matrix) from an input file and calculates the solutions

## Equation System

Given the following equation-system:
```
1:          0 a +         0 b +         0 c +         1 d   =    50
2:      8 000 a +       400 b +        20 c +         1 d   =   100
3:    125 000 a +     2 500 b +        50 c +         1 d   =   130
4:  1 000 000 a +    10 000 b +       100 c +         1 d   =   210
```

The augmented matrix would look like this:

<img src="augmented-matrix.svg" style="background-color: white;">

## Input File

The augmented matrix in a [normal input file](input.txt) would be represented like this:
```
0 0 0 1 50
8000 400 20 1 100
125000 2500 50 1 130
1000000 10000 100 1 210
```

The augmented matrix in a [json input file](input.json) would be represented like this:
```json
[
    [ 0, 0, 0, 1, 50 ],
    [ 8000, 400, 20, 1, 100 ],
    [ 125000, 2500, 50, 1, 130 ],
    [ 1000000, 10000, 100, 1, 210 ]
]
```

## Output File

The program calculates the solutions for the equation-system and writes them to an [output.json](output.json) file. The solutions are listed at the end and in order depending on the given equation system, along with all matrices from the system and their corresponding determinants.

*In our case* the **first solution** would be **a**, the **second solution** **b**, ...

```json
"solutions": [
    0.000375, // a
    -0.05625, // b
    3.475, // c
    50 // d
]
```
