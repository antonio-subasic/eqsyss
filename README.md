eqsyss is an [equation-system](https://en.wikipedia.org/wiki/System_of_equations)-solver written in Rust

# Download

1. [Download the Source Code](https://github.com/antonio-subasic/eqsyss/archive/refs/heads/main.zip)
1. Unzip the folder
1. Run `cargo build --release`
1. Copy the executable from `target/release/` to `/usr/local/bin/`: `sudo cp target/release/eqsyss /usr/local/bin/eqsyss`

# Usage

The program reads an equation-system in the form of an [augmented matrix](https://en.wikipedia.org/wiki/Augmented_matrix) from an `input.json` file and calculates the solutions

## Equation System

Given the following equation-system:
```
1:          0 a +         0 b +         0 c +         1 d   =    50
2:      8 000 a +       400 b +        20 c +         1 d   =   100
3:    125 000 a +     2 500 b +        50 c +         1 d   =   130
4:  1 000 000 a +    10 000 b +       100 c +         1 d   =   210
```

The augmented matrix would look like this:

![](augmented-matrix.svg)

## Input

The augmented matrix in the [json input file](input.json) would be represented like this:
```json
[
    [ 0, 0, 0, 1, 50 ],
    [ 8000, 400, 20, 1, 100 ],
    [ 125000, 2500, 50, 1, 130 ],
    [ 1000000, 10000, 100, 1, 210 ]
]
```

## Output

The program splits the large matrix into sub-matrices and calculates each sub-matrix' determinant (similar approach to [Cramer's rule](https://en.wikipedia.org/wiki/Cramer%27s_rule)). The sub-matrices and their determinants along with the solutions for the entire matrix are written into a `output.json` file. The solutions to the entire matrix are also printed to the console.

*In our case* the 
1. solution would be **a**
1. solution would be **b**
1. solution would be **c**
1. solution would be **d**

```bash
[ 0.000375, -0.05625, 3.475, 50.0 ]
```
