import fs from 'fs';
import lodash from 'lodash';

class AugmentedMatrix {
    private matrix: number[][];

    constructor(matrix: number[][]) {
        if (matrix.length !== matrix[0].length - 1) {
            throw new Error("Invalid Matrix dimensions.");
        }

        this.matrix = matrix;
    }

    getSubMatrices(): Matrix[] {
        const baseMatrix = new Matrix(this.matrix.map(line => line.slice(0, -1)));
        let matrices: Matrix[] = new Array(this.matrix.length);

        for (let i = 0; i < matrices.length; i++) {
            matrices[i] = baseMatrix.replace(i, this.matrix.map(line => line.at(-1) || 0));
        }

        return [baseMatrix, ...matrices];
    }
}

class Matrix {
    private matrix: number[][];

    constructor(matrix: number[][]) {
        if (matrix.length !== matrix[0].length) {
            throw new Error("Invalid Matrix dimensions.");
        }

        this.matrix = matrix;
    }

    private calculateDeterminant(matrix: number[][]): number {
        if (matrix.length !== matrix[0].length) {
            throw new Error("Matrix is not square. Determinant is undefined.");
        } else {
            return matrix.length === 1
                ? matrix[0][0]
                : matrix[0].reduce((acc, value, index) => {
                    const subMatrix = matrix.slice(1).map(line => line.filter((_, i) => i !== index));
                    return acc + value * this.calculateDeterminant(subMatrix) * Math.pow(-1, index);
                }, 0);
        }
    }

    determinant() {
        return this.calculateDeterminant(this.matrix);
    }

    replace(index: number, vector: number[]): Matrix {
        if (vector.length !== this.matrix.length) {
            throw new Error("Invalid vector length.");
        }

        const matrix = lodash.cloneDeep(this.matrix);
        matrix.forEach((line, i) => line[index] = vector[i]);

        return new Matrix(matrix);
    }

    toArray() {
        return this.matrix;
    }
}

class Output {
    private data: { matrices: { matrix: number[][], determinant: number }[], solutions: number[] };

    constructor(matrices: Matrix[], determinants: number[], solutions: number[]) {
        this.data = {
            matrices: matrices.map((matrix, index) => ({
                matrix: matrix.toArray(),
                determinant: determinants[index],
            })),
            solutions
        };
    }

    toJSON() {
        return this.data;
    }
}

if (process.argv.length !== 3) {
    let filePath = process.argv[1].split('/');
    console.error(`Usage: node ${filePath[filePath.length - 1]} <input file>`);
    process.exit(1);
}
else {
    let augmentedMatrix = new AugmentedMatrix(
        process.argv[2].endsWith('.json')
        ? JSON.parse(fs.readFileSync(process.argv[2], 'utf8'))
        : fs.readFileSync(process.argv[2], 'utf8').split('\n').map(line => line.split(' ').map(number => parseFloat(number)))
    );

    const matrices = augmentedMatrix.getSubMatrices();
    const determinants = matrices.map(matrix => matrix.determinant());
    const solutions = determinants.slice(1).map(determinant => determinant / determinants[0]);

    console.log(solutions);

    const output = new Output(matrices, determinants, solutions);
    fs.writeFileSync('output.json', JSON.stringify(output, null, 4));
}
