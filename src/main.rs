use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::Deserialize;
use std::fs;

struct AugmentedMatrix {
    matrix: Vec<Vec<f64>>,
}

impl AugmentedMatrix {
    fn new(matrix: Vec<Vec<f64>>) -> Self {
        if matrix.len() != matrix[0].len() - 1 {
            panic!("Invalid matrix dimensions");
        } else {
            Self { matrix }
        }
    }

    fn get_sub_matrices(&self) -> Vec<Matrix> {
        let base_matrix = Matrix::new(
            self.matrix
                .iter()
                .map(|line| line[..line.len() - 1].to_vec())
                .collect(),
        );

        let mut matrices = vec![base_matrix.clone()];

        for i in 0..self.matrix.len() {
            matrices.push(
                base_matrix.replace(
                    i,
                    self.matrix
                        .iter()
                        .map(|line| line.last().cloned().unwrap_or(0_f64))
                        .collect(),
                ),
            );
        }

        matrices
    }

    fn get_solution(&self) -> Solution {
        let sub_matrices = self.get_sub_matrices();
        let determinants = sub_matrices
            .iter()
            .map(|matrix| Matrix::calculate_determinant(&matrix.matrix))
            .collect::<Vec<f64>>();
        let solutions = determinants[1..]
            .iter()
            .map(|determinant| determinant / determinants[0])
            .collect::<Vec<f64>>();

        Solution {
            matrices: sub_matrices,
            solutions,
        }
    }
}

#[derive(Clone, Deserialize)]
struct Matrix {
    matrix: Vec<Vec<f64>>,
}

impl Matrix {
    fn new(matrix: Vec<Vec<f64>>) -> Self {
        if matrix.len() != matrix[0].len() {
            panic!("Invalid matrix dimensions");
        } else {
            Self { matrix }
        }
    }

    fn calculate_determinant(matrix: &Vec<Vec<f64>>) -> f64 {
        if matrix.len() == 1 {
            matrix[0][0]
        } else {
            let mut determinant = 0_f64;

            for i in 0..matrix.len() {
                let mut sub_matrix = matrix.clone();
                sub_matrix.remove(0);
                sub_matrix = sub_matrix
                    .iter()
                    .map(|line| {
                        let mut line = line.clone();
                        line.remove(i);
                        line
                    })
                    .collect();

                determinant += matrix[0][i]
                    * Matrix::calculate_determinant(&sub_matrix)
                    * (-1_f64).powi(i as i32);
            }

            determinant
        }
    }

    fn replace(&self, index: usize, vector: Vec<f64>) -> Self {
        let mut matrix = self.matrix.clone();

        for (i, line) in matrix.iter_mut().enumerate() {
            line[index] = vector[i];
        }

        Self { matrix }
    }
}

impl Serialize for Matrix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Matrix", 2)?;
        state.serialize_field("matrix", &self.matrix)?;
        state.serialize_field("determinant", &Matrix::calculate_determinant(&self.matrix))?;
        state.end()
    }
}

#[derive(Deserialize)]
struct Solution {
    matrices: Vec<Matrix>,
    solutions: Vec<f64>,
}

impl Serialize for Solution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Solution", 2)?;
        state.serialize_field("matrices", &self.matrices)?;
        state.serialize_field("solutions", &self.solutions)?;
        state.end()
    }
}

fn main() {
    if !std::env::args().count() == 2 {
        panic!("Invalid number of arguments");
    } else {
        let vector_matrix: Vec<Vec<f64>> = serde_json::from_str(
            &fs::read_to_string(std::env::args().nth(1).unwrap())
                .expect("couldn't read input file"),
        )
        .unwrap();

        let matrix = AugmentedMatrix::new(vector_matrix);
        let solution = matrix.get_solution();

        println!("{:?}", solution.solutions);

        let json = serde_json::to_string_pretty(&solution).unwrap();
        fs::write("output.json", json).expect("couldn't write output file");
    }
}
