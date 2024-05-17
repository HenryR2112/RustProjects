extern crate nalgebra as na;
use na::{DMatrix, DVector, Matrix, Dyn, U1, OMatrix, LU, Matrix3, Vector3};
use rand::{random};
fn main() {
    //generate static vector
    let vec = Vector3::new(1,2,3);
    //println!("Hello, world!");
    //println!("vector elements {vec}");

    //generate static matrix
    let matrix = Matrix3::new(1.0, 2.0, 3.0,
                              4.0, 5.0, 6.0,
                              7.0, 8.0, 10.0);
   // println!("matrix elements {matrix}");

    //generate random matrix of fixed size
    let matrix2 = Matrix3::from_fn(|_,_| random::<f64>());
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
    let lu2 = matrix3.lu();
    let l2 = lu2.l();
    let u2 = lu2.u();
    let p2 = lu2.p();
 //   println!(" L matrix elements {l2}");
 //   println!(" U matrix elements {u2}");
    let check = l2*u2;
    p2.permute_rows(&mut test);
    let difference = &test - &check;


    let mut b: DVector<f32> = DVector::new_random(10);
    println!("b vector {b}");

    let x: Option<OMatrix<f32, Dyn, U1>> = lu2.solve(&b);
    println!("x vector (Debug): {:?}", x);

    let tester = standard_solver::<f32>(test2, &b);
     println!("x vector (Debug): {:?}", tester);
}


///
///
/// # Arguments
///
/// * `A`: An input matrix of any size m x n dynamically with f32 values
/// * `b`: A b vector of length n which is the solution.
///
/// returns: Option<Matrix<f32, Dyn, Const<1>, VecStorage<f32, Dyn, Const<1>>>>
///          a vector x such that the system solution Ax = b using the LU decomposition method
/// # Examples
///
/// ```
///
/// ```
fn standard_solver<T>(A: OMatrix<f32, Dyn, Dyn>, b: &OMatrix<f32, Dyn, U1>) -> Option<DVector<f32>> {
    let lu = A.lu();
    let x = lu.solve(b);
    return x;
}


///
///
/// # Arguments
///
/// * `A`: An input matrix of any size m x n dynamically with f32 values
/// * `b`: A b vector of length n which is the solution.
///
/// returns: Option<Matrix<f32, Dyn, Const<1>, VecStorage<f32, Dyn, Const<1>>>>
///          returns a vector x which solves the system Ax=b for Strictly Diagonally
///          Dominant tri-diagonal matrices.
/// # Examples
///
/// ```
///
/// ```
fn tri_diag_solver<T>(A: OMatrix<f32, Dyn, Dyn>, b: &OMatrix<f32, Dyn, U1>) -> Option<DVector<f32>> {
    todo!()
}
