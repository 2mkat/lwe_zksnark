use crate::{
    common::{FEp, FE},
    r1cs::{self, Constraint, R1CS},
};
use std::convert::From;
use lambdaworks_math::polynomial::Polynomial;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SquareArithmeticProgram {
    /// Number of public input (a.k.a. instance) variables in the underlying R1CS, including the leading `1`
    /// public input + intermidiate, include s_0 = 1
    pub num_instance_variables: usize,  
    /// Number of private (a.k.a. witness) variables in the underlying R1CS 
    pub num_r1cs_witness_variables: usize, 
    /// Number of constraints in the underlying R1CS.
    pub num_r1cs_constraints: usize,
    pub u_polynomials: Vec<Polynomial<FE>>,
    pub w_polynomials: Vec<Polynomial<FE>>,
    pub target: Polynomial<FE>,
    pub r1cs: R1CS,
}

#[derive(Debug)]
pub enum CreationError {
    PolynomialVectorsSizeMismatch,
}

impl SquareArithmeticProgram {
    /// Creates a new SAP
    pub fn new(
        num_instance_variables: usize,
        num_r1cs_witness_variables: usize,
        num_r1cs_constraints: usize,
        u_polynomials: Vec<Polynomial<FE>>,
        w_polynomials: Vec<Polynomial<FE>>,
        target: Polynomial<FE>,
        r1cs: R1CS,
    ) -> Result<Self, CreationError> {
        // if u_polynomials.len() != w_polynomials.len()
        //     || num_instance_variables + r1cs.number_of_outputs > u_polynomials.len()
        // {
        //     Err(CreationError::PolynomialVectorsSizeMismatch)
        // } else {
        //     Ok(Self {
        //         num_instance_variables,
        //         num_r1cs_witness_variables,
        //         num_r1cs_constraints,
        //         u_polynomials,
        //         w_polynomials,
        //         target,
        //         r1cs,
        //     })
        // }
        Ok(Self {
            num_instance_variables,
            num_r1cs_witness_variables,
            num_r1cs_constraints,
            u_polynomials,
            w_polynomials,
            target,
            r1cs,
        })
    }

    pub fn h_polinomial(&self, c: &[FE]) -> Polynomial<FE> {
        self.p_polinomial(c).div_with_ref(&self.target)
    }

    pub fn p_polinomial(&self, cs: &[FE]) -> Polynomial<FE> {
        let u_x: Polynomial<FE> = self.u_polynomials[0].clone()
            + self.u_polynomials[1..]
                .iter()
                .zip(cs.clone())
                .map(|(v, c)| v.mul_with_ref(&Polynomial::new_monomial(c.clone(), 0)))
                .reduce(|x, y| x + y)
                .unwrap();

        let w_x: Polynomial<FE> = self.w_polynomials[0].clone()
            + self.w_polynomials[1..]
                .iter()
                .zip(cs.clone())
                .map(|(v, c)| v.mul_with_ref(&Polynomial::new_monomial(c.clone(), 0)))
                .reduce(|x, y| x + y)
                .unwrap();

    // let p_polinomial = &double_u_x - &w_x2;

    // let h = p_polinomial.div_with_ref(&t);

    // // check condition
    // let right = w_x2 + h.mul_with_ref(&t);
    
    // println!("\nright\n");
    // for j in 0..right.coeff_len() {
    //     print!("{} ", right.coefficients[j]);
    // }

    // println!("\nleft\n");
    // for j in 0..double_u_x.coeff_len() {
    //     print!("{} ", double_u_x.coefficients[j]);
    // }

        u_x.clone() * u_x - w_x
    }

    pub fn r1cs_to_sap(r1cs: R1CS) -> Self {
        let num_r1cs_constraints = r1cs.num_of_constraints();
        let num_instance_variables = r1cs.num_instance_variables();
        let num_r1cs_aux_variables = r1cs.num_r1cs_aux_variables();

        let sap_num_var = num_r1cs_constraints + 2 * (num_instance_variables - 1) + num_r1cs_aux_variables;
        let extra_var_offset = num_instance_variables + num_r1cs_aux_variables;
        let extra_constr_offset = 2 * num_r1cs_constraints;
        let extra_var_offset2 = (num_instance_variables - 1) + num_r1cs_aux_variables + num_r1cs_constraints;

        let mut a = vec![vec![FE::zero(); sap_num_var + 1]; extra_constr_offset + 2 * num_instance_variables];
        let mut c = vec![vec![FE::zero(); sap_num_var + 1]; extra_constr_offset + 2 * num_instance_variables];

        let (a_matrix, b_matrix, c_matrix) = r1cs.constraints_to_matrix();

        //  (\sum a_i * s_i )* (\sum b_i * s_i ) = \sum c_i*s_i
        // into two constraints
        // (\sum (a_i + b_i) s_i)^2 = 4 \sum c_i*s_i + s'_i
        // (\sum (a_i - b_i) s_i)^2 = s'_i
        
        for i in 0..num_r1cs_constraints {
            for j in 0..r1cs.constraints[i].a.len() {
                a[j][2 * i] += a_matrix[i][j];
                a[j][2 * i + 1] += a_matrix[i][j];
            }

            for j in 0..r1cs.constraints[i].b.len() {
                a[j][2 * i] += b_matrix[i][j];
                a[j][2 * i + 1] = a[j][2 * i + 1] - b_matrix[i][j];
            }
    
            for j in 0..r1cs.constraints[i].c.len() {
                c[j][2 * i] += times_four(c_matrix[i][j]);
            }
    
            c[extra_var_offset + i][2 * i] += FE::from(1);
            c[extra_var_offset + i][2 * i + 1] += FE::from(1);
        }

        // (s_i + s_0)^2 = 4 s_i + s''_i
        // (s_i - s_0)^2 = s''_i
    
        a[0][extra_constr_offset] = FE::from(1);
        c[0][extra_constr_offset] = FE::from(1);

        for i in 1..num_instance_variables {
            a[i][extra_constr_offset + 2 * i - 1] += FE::from(1);
            a[0][extra_constr_offset + 2 * i - 1] += FE::from(1);
            c[i][extra_constr_offset + 2 * i - 1] += times_four(FE::from(1));
            c[extra_var_offset2 + i][extra_constr_offset + 2 * i - 1] += FE::from(1);

            a[i][extra_constr_offset + 2 * i] += FE::from(1);
            a[0][extra_constr_offset + 2 * i] = a[0][extra_constr_offset + 2 * i] - FE::from(1);
            c[extra_var_offset2 + i][2 * num_r1cs_constraints + 2 * i] += FE::from(1);
        }

        // println!("A");
        // for a_i in a {
        //     for j in a_i {
        //         print!("{}      ", j);
        //     }
        //     println!();
        // }

        // println!("C");
        // for c_i in c {
        //     for j in c_i {
        //         print!("{}      ", j);
        //     }
        //     println!();
        // }

        let rq_size = 2 * num_r1cs_constraints + 2 * (num_instance_variables - 1) + 1;

        let rs: Vec<FE> = (0..rq_size as u64)
            .map(|i| FE::new(i.into()))
            .collect();

        // println!("roots:");
        // for r in &rs {
        //     println!("{}", r);
        // }


        let mut us: Vec<Polynomial<FE>> = Vec::with_capacity(sap_num_var + 1);
        let mut ws: Vec<Polynomial<FE>> = Vec::with_capacity(sap_num_var + 1);
        let mut t: Polynomial<FE> = Polynomial::new_monomial(FE::from(1), 0);

        println!("t(x) = ");
        for r in &rs {
            t = t * Polynomial::new(&[-r, FE::from(1)]);
        }

        for i in 0..t.coeff_len() {
            println!("{}", t.coefficients[i]);
        }

        for i in 0..=sap_num_var {
            let u_ys: Vec<FE> = a[i].clone();
            let w_ys: Vec<FE> = c[i].clone();

            // println!("u_ys:");
            // for i in &u_ys {
            //     println!("{} ", i);
            // }

            // println!("y_ys:");
            // for i in &w_ys {
            //     println!("{} ", i);
            // }

            us.push(Polynomial::interpolate(&rs, &u_ys).expect("should interpolate"));
            ws.push(Polynomial::interpolate(&rs, &w_ys).expect("should interpolate"));
        }

        println!("A(x)");
        for u in &us {
            for j in 0..u.coeff_len() {
                print!("{} ", u.coefficients[j]);
            }
            println!("\n");
        }
        println!("C(x)");
        for w in &ws {
            for j in 0..w.coeff_len() {
                print!("{} ", w.coefficients[j]);
            }
            println!("\n");
        }
        
        Self {
            num_instance_variables,
            num_r1cs_witness_variables: num_r1cs_aux_variables,
            num_r1cs_constraints,
            u_polynomials: us,
            w_polynomials: ws,
            target: t,
            r1cs,
        }
    }

}

fn times_four(x : FE) -> FE {
    let times_two = x + x;
    times_two + times_two
}

fn eval(terms: &[FE], assignment: &[FE]) -> FE {
    let mut acc = FE::from(0);
    for i in 0..terms.len() {
        let value = assignment[i];
        acc += value * terms[i];
    }

    acc
}


#[cfg(test)]
pub mod tests {
    use crate::test_ex::{new_test_first_constraint, new_test_r1cs, new_test_second_constraint};

    use super::*;
   
    #[test]
    fn matrix_constraints() {
        let constraints = vec![new_test_first_constraint(), new_test_second_constraint()];
        let r1cs = R1CS::new(constraints, 4, 1).unwrap();

        r1cs.constraints_to_matrix();
    }

    #[test]
    fn r1cs_to_sap_test_with_evaluation() {
        let constraints = vec![new_test_first_constraint(), new_test_second_constraint()];
        let r1cs = new_test_r1cs();

        let sap = SquareArithmeticProgram::r1cs_to_sap(r1cs.clone());

        // instance_plus_withess
        let mut full_input: Vec<FE> = vec![FE::from(1), FE::from(3), FE::from(5), FE::from(4),
                                           FE::from(2), FE::from(8), FE::from(64)];

        let (a, b, _c) = &r1cs.constraints_to_matrix();

        // extra_var = (a_i - b_i)^2
        let temp = a.iter()
                                                    .zip(b.iter())
                                                    .map(|(a_i, b_i)| {
                                                        let mut extra_var = eval(a_i, &full_input);
                                                        extra_var = extra_var - eval(b_i, &full_input);
                                                        extra_var * extra_var
                                                    })
                                                    .collect::<Vec<_>>();
    
        full_input.extend(temp); 

        // extra_var = (x_i - 1)^2
        let one = FE::from(1);
        for i in 1..sap.num_instance_variables {
            let mut extra_var = full_input[i];
            extra_var = extra_var - &one;
            extra_var *= extra_var;
            full_input.push(extra_var);
        }


        println!("Full instance"); 
        for i in &full_input {
            println!("{}", i);
        }  
        
        // assert_eq!(double_u_x, right);

        // let zero = FE::from(0);
        // let rq_size = 2 * sap.num_r1cs_constraints + 2 * (sap.num_instance_variables - 1) + 1;

        // let mut a = vec![zero; rq_size];
        // for i in 0..sap.num_r1cs_constraints {
        //     a[2*i] += eval(&sap.r1cs.constraints[i].a, &full_input);
        //     a[2*i] += eval(&sap.r1cs.constraints[i].b, &full_input);

        //     a[2*i + 1] += eval(&sap.r1cs.constraints[i].a, &full_input);
        //     a[2*i + 1] = a[2*i + 1] -  eval(&sap.r1cs.constraints[i].b, &full_input);
        // }

        // println!("\na[]");
        // for i in a {
        //     println!("{}", i);
        // }
    }

    fn test_solution() -> Vec<FE> {
        vec![
            FE::from(0),
            FE::from(1),
            FE::from(2),
            FE::from(3),
            FE::from(4),
            FE::from(12),
            FE::from(36),
        ]
    }
}