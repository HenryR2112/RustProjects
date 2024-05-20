extern crate nalgebra as na;

use std::time::Instant;
use na::{DMatrix, DVector, Matrix, Dyn, U1, OMatrix, LU, Matrix3, Vector3};
use rand::{random};
use plotly::{Plot, Scatter};
use plotly::common::Mode;
use rayon::prelude::*;
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};

fn main() {
    algorithmic_comparison()
}

fn regression_modeling(){
    let sizes: Vec<usize> = (10..=20000).step_by(1000).collect();
    let tri_diag_times: Vec<f64> = sizes.par_iter().map(|&size| {
        let (A, b) = tri_diag_generator(size);

        // Measure time for tri-diagonal solver
        let start = Instant::now();
        let _x_tri_diag = tri_diag_solver(A.clone(), b.clone());
        let duration = start.elapsed();
        duration.as_secs_f64()
    }).collect();

    let sizes_f64: Vec<f64> = sizes.iter().map(|&s| s as f64).collect();
    let data = vec![
        ("sizes".to_string(), sizes_f64.clone()),
        ("times".to_string(), tri_diag_times.clone())
    ];

    let regression_data = RegressionDataBuilder::new()
        .build_from(data)
        .expect("Failed to create regression data");

    let formula = "times ~ sizes";
    let model = FormulaRegressionBuilder::new()
        .data(&regression_data)
        .formula(formula)
        .fit()
        .expect("Failed to fit linear regression model");

    let params = model.parameters();

    let intercept = params[0];
    let slope = params[1];

    println!("Linear Regression Result:");
    println!("Intercept: {}", intercept);
    println!("Slope: {}", slope);

    let regression_line: Vec<f64> = sizes_f64.iter().map(|&x| intercept + slope * x).collect();

    // Plotting
    let mut plot = Plot::new();
    let tri_diag_trace = Scatter::new(sizes.clone(), tri_diag_times).mode(Mode::Lines).name("Tri-diag Solver");
    let regression_trace = Scatter::new(sizes.clone(), regression_line).mode(Mode::Lines).name("Regression Line");

    plot.add_trace(tri_diag_trace);
    plot.add_trace(regression_trace);

    plot.write_html("out2.html");
}
fn algorithmic_comparison(){
    let sizes: Vec<usize> = (10..=10000).step_by(1000).collect();
    let tri_diag_times: Vec<f64> = sizes.par_iter().map(|&size| {
        let (A, b) = tri_diag_generator(size);

        // Measure time for tri-diagonal solver
        let start = Instant::now();
        let _x_tri_diag = tri_diag_solver(A.clone(), b.clone());
        let duration = start.elapsed();
        duration.as_secs_f64()
    }).collect();

    let standard_times: Vec<f64> = sizes.par_iter().map(|&size| {
        let (A, b) = tri_diag_generator(size);

        // Measure time for standard solver
        let start = Instant::now();
        let _x_standard = standard_solver(A.clone(), b.clone());
        let duration = start.elapsed();
        duration.as_secs_f64()
    }).collect();

    // Plotting
    let mut plot = Plot::new();
    let tri_diag_trace = Scatter::new(sizes.clone(), tri_diag_times).mode(Mode::Lines).name("Tri-diag Solver");
    let standard_trace = Scatter::new(sizes.clone(), standard_times).mode(Mode::Lines).name("Standard Solver");

    plot.add_trace(tri_diag_trace);
    plot.add_trace(standard_trace);

    plot.write_html("out.html");
}
fn debug() {
    //generate static vector
    let vec = Vector3::new(1, 2, 3);
    //println!("Hello, world!");
    //println!("vector elements {vec}");

    //generate static matrix
    let matrix = Matrix3::new(1.0, 2.0, 3.0,
                              4.0, 5.0, 6.0,
                              7.0, 8.0, 10.0);
    // println!("matrix elements {matrix}");

    //generate random matrix of fixed size
    let matrix2 = Matrix3::from_fn(|_, _| random::<f64>());
    // println!("matrix elements {matrix2}");

    //LU decomposition of matrix
    let lu = matrix.lu();
    let l = lu.l();
    let u = lu.u();
    //let p = lu.p();
    //  println!("matrix elements {matrix}");
    //  println!(" L matrix elements {l}");
    //  println!(" U matrix elements {u}");


    //random matrix of size nxm if choosing with variable typing with print followed by code which
    // then takes it LU factorization and then checks by reversing the ensure that PA=LU based off
    //initial checks
    let matrix3: OMatrix<f32, Dyn, Dyn> = DMatrix::from_fn(10, 10, |_, _| random::<f32>());
    // println!("matrix elements {matrix3}");
    let mut test = matrix3.clone();
    let mut test2 = matrix3.clone();
    let mut test3 = matrix3.clone();
    let lu2 = matrix3.lu();
    let l2 = lu2.l();
    let u2 = lu2.u();
    let p2 = lu2.p();
    //   println!(" L matrix elements {l2}");
    //   println!(" U matrix elements {u2}");
    let check = l2 * u2;
    p2.permute_rows(&mut test);
    let difference = &test - &check;


    let mut b: DVector<f32> = DVector::new_random(10);
    // println!("b vector {b}");

    let x: Option<OMatrix<f32, Dyn, U1>> = lu2.solve(&b);
    println!("x vector (Debug): {:?}", x);

    let tester = standard_solver(test2, b);
    println!("x vector (Debug): {:?}", tester);

    let matrix_size = test3.shape();
    let (row_len, column_len) = matrix_size;
    println!("{row_len} and {column_len}");
    let diag = test3.diagonal();
    println!("{diag}");


    let(A, b) = tri_diag_generator(10);

    // Solve using the tri-diagonal solver
    let x_tri_diag = tri_diag_solver(A.clone(), b.clone());
    println!("Solution using tri_diag_solver: {:?}", x_tri_diag);

    // Solve using the standard solver
    let x_standard = standard_solver(A.clone(), b.clone());
    match x_standard {
        Some(x) => println!("Solution using standard_solver: {:?}", x),
        None => println!("standard_solver failed to find a solution."),
    }

}
fn tri_diag_generator(n: usize) -> (DMatrix<f32>, DVector<f32>){
    // Generate a tri-diagonal matrix
    let mut A = DMatrix::<f32>::zeros(n, n);
    for i in 0..n {
        A[(i, i)] = random::<f32>() * 10.0 + 1.0; // Main diagonal
        if i < n - 1 {
            A[(i, i + 1)] = random::<f32>() * 10.0; // Super-diagonal
            A[(i + 1, i)] = random::<f32>() * 10.0; // Sub-diagonal
        }
    }
    //println!("Tri-diagonal matrix A:\n{}", A);

    let b: DVector<f32> = DVector::new_random(n);
    //println!("Vector b:\n{}", b);
    return (A, b);
}
fn standard_solver(A: OMatrix<f32, Dyn, Dyn>, b: DVector<f32>) -> Option<DVector<f32>> {
    let lu = A.lu();
    //if !lu.is_invertible() {
    //   println!("Matrix is singular or nearly singular.");
    //    return None;
    //}
    lu.solve(&b)
}
fn tri_diag_solver(A: DMatrix<f32>, b: DVector<f32>) -> DVector<f32> {
    let (row_len, column_len) = A.shape();
    let (diag_opt, super_diag_opt, sub_diag_opt) = diag_grabber::<(Option<DVector<f32>>, Option<DVector<f32>>, Option<DVector<f32>>)>(A);

    let diag = diag_opt.unwrap();
    let super_diag = super_diag_opt.unwrap();
    let sub_diag = sub_diag_opt.unwrap();

    let mut forward_vector = DVector::<f32>::zeros(row_len);
    let mut modified_b = b.clone();
    let mut c_prime = DVector::<f32>::zeros(row_len-1);

    // Forward elimination
    forward_vector[0] = diag[0];
    for i in 1..row_len {
        let m = sub_diag[i - 1] / forward_vector[i - 1];
        forward_vector[i] = diag[i] - m * super_diag[i - 1];
        modified_b[i] = modified_b[i] - m * modified_b[i - 1];
    }

    // Back substitution
    let mut x = DVector::from_element(row_len, 0.0);
    x[row_len - 1] = modified_b[row_len - 1] / forward_vector[row_len - 1];
    for i in (0..row_len - 1).rev() {
        x[i] = (modified_b[i] - super_diag[i] * x[i + 1]) / forward_vector[i];
    }

    x
}
fn diag_grabber<T>(A: OMatrix<f32, Dyn, Dyn>) -> (Option<DVector<f32>>, Option<DVector<f32>>, Option<DVector<f32>>){
    let (row_len, column_len) = A.shape();
    let min_dim = row_len.min(column_len);

    let main_diag = A.diagonal();

    let super_diag = if min_dim > 1 {
        Some(DVector::from_iterator(min_dim - 1, (0..min_dim -1).map(|i| A[(i, i+1)])))
    } else {
        None
    };

    let sub_diag: Option<DVector<f32>> = if min_dim > 1 {
        Some(DVector::from_iterator(min_dim - 1, (0..min_dim - 1).map(|i| A[(i + 1, i)])))
    } else {
        None
    };

    let diag_opt = if main_diag.len() > 0 {Some(main_diag)} else {None};

    (diag_opt, super_diag, sub_diag)
}


