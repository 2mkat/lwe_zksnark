use crate::common::FEp;

#[derive(Debug, PartialEq, Eq)]
pub enum CreationError {
    VectorsSizeMismatch,
    MatrixesSizeMismatch,
    /// Number of IOs should be less than witness size - 1
    InputOutputTooBig,
}

/**
 * A R1CS constraint is a formal expression of the form
 *
 *                < A , X > * < B , X > = < C , X > ,
 *
 * where X = (x_0,x_1,...,x_m) is a vector of formal variables and A,B,C each
 * consist of 1+m elements in <FieldT>.
 *
 * A R1CS constraint is used to construct a R1CS constraint system (see below).
 */

/// R1CS представление арифметической программы
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constraint {
    pub a: Vec<FEp>,
    pub b: Vec<FEp>,
    pub c: Vec<FEp>,
}

use std::fmt;
impl std::fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Constraint:\n  a: {:?}\n  b: {:?}\n  c: {:?}",
            self.a, self.b, self.c
        )
    }
}

/// R1CS представлен как веткор ограничений (векторов)
/// Каждое ограничение состоит из трех векторов
/// Все ограничения вместе формируют три матрицы A, B, C,
/// которые используются для описания R1CS
/// Первые вектора a из каждого ограничения соединяются в матрицу
/// Вторые вектора b тоже соединяются в матрицу
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct R1CS {
    pub constraints: Vec<Constraint>,
    pub number_of_inputs: usize,    // public input size
    pub number_of_outputs: usize,
}

impl R1CS {
    #[allow(dead_code)]
    pub fn new(
        constraints: Vec<Constraint>,
        number_of_inputs: usize,
        number_of_outputs: usize,
    ) -> Result<Self, CreationError> {
        // 
        let witness_size = constraints[0].a.len();
        // println!("Constraint [0] = {}", constraints[0]);
        // println!("Witness Size = {}", witness_size.clone());
        // Каждое ограничение уже имеет правильную размерность
        // то есть все вектора имеют один размер
        // проверяем, что все вектора имеют одинаковый размер
        let all_same_length = constraints
            .iter()
            .all(|v| v.a.len() == constraints[0].a.len());

        // Нужно ли добавить провекрки на другие вектора?

        if !all_same_length {
            Err(CreationError::VectorsSizeMismatch)
        } else if number_of_inputs + number_of_outputs > witness_size - 1 {
            Err(CreationError::InputOutputTooBig)
        } else {
            Ok(Self {
                constraints,
                number_of_inputs,
                number_of_outputs,
            })
        }
    }

    pub fn constraints_to_matrix(&self) -> (Vec<Vec<FEp>>, Vec<Vec<FEp>>, Vec<Vec<FEp>>) {
        let constr = &self.constraints;
        // num_constraints
        let m = constr.len();
        // public input + witness
        let n = (constr[0].a).len();


         // Create matrix A, B, C with size m x n
        let mut a_matrix = vec![vec![FEp::zero(); n]; m];
        let mut b_matrix = vec![vec![FEp::zero(); n]; m];
        let mut c_matrix = vec![vec![FEp::zero(); n]; m];

        // fulfill the matrix
        for (i, constraint) in constr.iter().enumerate() {
            // a_matrix[i].clone_from_slice(&constraint.a);
            // b_matrix[i].clone_from_slice(&constraint.b);
            // c_matrix[i].clone_from_slice(&constraint.c);

            a_matrix[i] = constraint.a.to_vec();
            b_matrix[i] = constraint.b.to_vec();
            c_matrix[i] = constraint.c.to_vec();
        }

        // println!("A Matrix:");
        // for i in 0..m {
        //     for j in 0..n {
        //         print!("{} ", a_matrix[i][j].representative().to_string());
        //     } 
        //     println!();
        // }
        
        // println!("B Matrix:");
        // for i in 0..m {
        //     for j in 0..n {
        //         print!("{} ", b_matrix[i][j].representative().to_string());
        //     } 
        //     println!();
        // }

        // println!("C Matrix:");
        // for i in 0..m {
        //     for j in 0..n {
        //         print!("{} ", c_matrix[i][j].representative().to_string());
        //     } 
        //     println!();
        // }

        (a_matrix, b_matrix, c_matrix)
    }

    pub fn new_with_matrixes(
        a: Vec<Vec<FEp>>,
        b: Vec<Vec<FEp>>,
        c: Vec<Vec<FEp>>,
        num_inputs: usize,
        num_outputs: usize,
    ) -> Result<Self, CreationError> {
        // Создаем пустой вектор 
        let mut constraints: Vec<Constraint> = Vec::with_capacity(a.len());
        // TO DO:
        // Проверить, что размеры совпадают, 
        // все три матрицы должны иметь одинаковое число проверяется в создании ограничения из матрицы
        // Удалить клоны
        for i in 0..a.len() {
            constraints.push(Constraint::new(a[i].clone(), b[i].clone(), c[i].clone()).unwrap())
        }
        R1CS::new(constraints, num_inputs, num_outputs)
    }

    #[allow(dead_code)]
    pub fn verify_solution(self, s: &[FEp]) -> bool {
        for constraint in self.constraints {
            if !constraint.verify_solution(s) {
                return false;
            }
        }
        true
    }

    pub fn num_of_constraints(&self) -> usize {
        self.constraints.len()
    }

    // include leading "1"
    pub fn num_instance_variables(&self) -> usize {
        self.number_of_inputs + 1
    }

    pub fn num_r1cs_aux_variables(&self) -> usize {
        self.witness_size() - 1 - self.number_of_inputs
    }

    /// Возвращает полный размер witness
    /// This is the constant part, plus the of inputs + intermediate values +
    /// outputs
    pub fn witness_size(&self) -> usize {
        // все ограничения имет одинаковый размер
        // это предусмотрено перд созданием ограничения
        self.constraints[0].a.len() // возвращмеи количество столбцов
    }
}


impl Constraint {
    /// Создаем новое ограничение для a,b,c векторов
    /// размеры всех векторов должны совпадать
    #[allow(dead_code)]
    pub fn new(a: Vec<FEp>, b: Vec<FEp>, c: Vec<FEp>) -> Result<Self, CreationError> {
        if a.len() != b.len() || a.len() != c.len() || b.len() != c.len() {
            Err(CreationError::VectorsSizeMismatch)
        } else {
            Ok(Self { a, b, c })
        }
    }

    #[allow(dead_code)]
    pub fn verify_solution(self, s: &[FEp]) -> bool {
        inner_product(&self.a, s) * inner_product(&self.b, s) == inner_product(&self.c, s)
    }
}

// вычисляем скалярное произведение двух векторов
pub fn inner_product(v1: &[FEp], v2: &[FEp]) -> FEp {
    v1.iter()
        .zip(v2)
        .map(|(x, y)| x * y)
        .fold(FEp::from(0), |x, y| x + y)
}

#[cfg(test)]
pub mod tests {
    use crate::test_ex::{new_test_first_constraint, new_test_r1cs, new_test_second_constraint};

    use super::*;

    #[test]
    fn mul_vectors_2_2_3_3_equals_12() {
        let v1 = &[FEp::from(2), FEp::from(2)];
        let v2 = &[FEp::from(3), FEp::from(3)];

        assert_eq!(inner_product(v1, v2), FEp::from(12));
    }

    #[test]
    fn mul_vectors_3_5_equals_15() {
        let v1 = &[FEp::from(3)];
        let v2 = &[FEp::from(5)];

        assert_eq!(inner_product(v1, v2), FEp::from(15));
    }

    #[test]
    fn verify_solution_with_test_circuit_c5_constraints() {
        assert!(new_test_second_constraint().verify_solution(&test_solution()));
    }

    #[test]
    fn verify_solution_with_test_circuit_c6_constraints() {
        assert!(new_test_second_constraint().verify_solution(&test_solution()));
    }

    #[test]
    fn verify_bad_solution_with_test_circuit_c5_constraints() {
        let solution = vec![
            FEp::from(0),
            FEp::from(0),
            FEp::from(0),
            FEp::from(4),
            FEp::from(1),
            FEp::from(0),
            FEp::from(0),
        ];
        assert!(!new_test_first_constraint().verify_solution(&solution));
    }

    #[test]
    fn verify_bad_solution_with_test_circuit_c6_constraints() {
        let solution = vec![
            FEp::from(0),
            FEp::from(2),
            FEp::from(1),
            FEp::from(4),
            FEp::from(5),
            FEp::from(2),
            FEp::from(2),
        ];
        assert!(!new_test_second_constraint().verify_solution(&solution));
    }

    #[test]
    fn verify_solution_with_new_test_r1cs() {
        assert!(new_test_r1cs().verify_solution(&test_solution()))
    }

    #[test]
    fn verify_bad_solution_with_new_test_r1cs() {
        let solution = vec![
            FEp::from(0),
            FEp::from(2),
            FEp::from(1),
            FEp::from(4),
            FEp::from(5),
            FEp::from(2),
            FEp::from(2),
        ];

        assert!(!new_test_r1cs().verify_solution(&solution))
    }

    #[test]
    fn verify_bad_solution_because_of_second_constraint_with_new_test_r1cs() {
        let solution = vec![
            FEp::from(0),  // c0
            FEp::from(2),  // c1
            FEp::from(1),  // c2
            FEp::from(5),  // c3
            FEp::from(10), // c4
            FEp::from(50), // c5 = c4 * c3
            FEp::from(2),  // c6 != c5 * (c1+c2)
        ];
        assert!(!new_test_r1cs().verify_solution(&solution))
    }

    #[test]
    fn verify_bad_solution_because_of_first_constraint_with_new_test_r1cs() {
        let solution = vec![
            FEp::from(0),  // c0
            FEp::from(1),  // c1
            FEp::from(1),  // c2
            FEp::from(5),  // c3
            FEp::from(10), // c4
            FEp::from(10), // c5 != c4 * c3
            FEp::from(19), // c6 = c5 * (c1+c2), so this should fail
        ];
        assert!(!new_test_r1cs().verify_solution(&solution))
    }

    fn test_solution() -> Vec<FEp> {
        vec![
            FEp::from(0),
            FEp::from(1),
            FEp::from(2),
            FEp::from(3),
            FEp::from(4),
            FEp::from(12),
            FEp::from(36),
        ]
    }
}