use na::{DMatrix, Matrix3, Matrix4};

#[test]
#[rustfmt::skip]
fn schur_simpl_mat3() {
    let m = Matrix3::new(-2.0, -4.0, 2.0,
                                   -2.0,  1.0, 2.0,
                                   4.0,  2.0, 5.0);

    let schur = m.schur();
    let (vecs, vals) = schur.unpack();

    assert!(relative_eq!(vecs * vals * vecs.transpose(), m, epsilon = 1.0e-7));
}

#[cfg(feature = "proptest-support")]
mod proptest_tests {
    macro_rules! gen_tests(
        ($module: ident, $scalar: expr, $scalar_type: ty) => {
            mod $module {
                use na::DMatrix;
                #[allow(unused_imports)]
                use crate::core::helper::{RandScalar, RandComplex};
                use crate::proptest::*;
                use proptest::{prop_assert, proptest};

                proptest! {
                    #[test]
                    fn schur(n in PROPTEST_MATRIX_DIM) {
                        let m  = DMatrix::<$scalar_type>::new_random(n, n).map(|e| e.0);
                        let (vecs, vals) = m.clone().schur().unpack();
                        prop_assert!(relative_eq!(&vecs * vals * vecs.adjoint(), m, epsilon = 1.0e-7));
                    }

                    #[test]
                    fn schur_static_mat2(m in matrix2_($scalar)) {
                        let (vecs, vals) = m.clone().schur().unpack();
                        prop_assert!(relative_eq!(vecs * vals * vecs.adjoint(), m, epsilon = 1.0e-7));
                    }

                    #[test]
                    fn schur_static_mat3(m in matrix3_($scalar)) {
                        let (vecs, vals) = m.clone().schur().unpack();
                        prop_assert!(relative_eq!(vecs * vals * vecs.adjoint(), m, epsilon = 1.0e-7));
                    }

                    #[test]
                    fn schur_static_mat4(m in matrix4_($scalar)) {
                        let (vecs, vals) = m.clone().schur().unpack();
                        prop_assert!(relative_eq!(vecs * vals * vecs.adjoint(), m, epsilon = 1.0e-7));
                    }
                }
            }
        }
    );

    gen_tests!(complex, complex_f64(), RandComplex<f64>);
    gen_tests!(f64, PROPTEST_F64, RandScalar<f64>);
}

#[test]
#[rustfmt::skip]
fn schur_static_mat4_fail() {
    let m = Matrix4::new(
         33.32699857679677,  46.794945978960044, -20.792148817005838,   84.73945485997737,
        -53.04896234480401,  -4.031523330630989,  19.022858300892366,   -93.2258351951158,
        -94.61793793643038,  -18.64216213611094,   88.32376703241675,  -99.30169870309795,
         90.62661897246733,   96.74200696130146,    34.7421322611369,   84.86773307198098);

    let (vecs, vals) = m.clone().schur().unpack();
    assert!(relative_eq!(vecs * vals * vecs.transpose(), m, epsilon = 1.0e-7))
}

#[test]
#[rustfmt::skip]
fn schur_static_mat4_fail2() {
    let m = Matrix4::new(
        14.623586538485966, 7.646156622760756, -52.11923331576265, -97.50030223503413,
        53.829398131426785, -33.40560799661168, 70.31168286972388, -81.25248138434173,
        27.932377940728202, 82.94220150938, -35.5898884705951, 67.56447552434219,
        55.66754906908682, -42.14328890569226, -20.684709585152206, -87.9456949841046);

    let (vecs, vals) = m.clone().schur().unpack();
    assert!(relative_eq!(vecs * vals * vecs.transpose(), m, epsilon = 1.0e-7))
}

#[test]
#[rustfmt::skip]
fn schur_static_mat3_fail() {
    let m = Matrix3::new(
        -21.58457553143394,   -67.3881542667948, -14.619829849784338,
        -7.525423104386547, -17.827350599642287,  11.297377444555849,
        38.080736654870464,  -84.27428302131528,  -95.88198590331922);

    let (vecs, vals) = m.clone().schur().unpack();
    assert!(relative_eq!(vecs * vals * vecs.transpose(), m, epsilon = 1.0e-7))
}

// Test proposed on the issue #176 of rulinalg.
#[test]
#[rustfmt::skip]
fn schur_singular() {
    let m = DMatrix::from_row_slice(24, 24, &[
        1.0,  1.0,  1.0,  1.0,  1.0,  1.0,  0.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0,  0.0,  0.0,  0.0,  0.0,  1.0,  1.0,  1.0,  1.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0,  0.0,  1.0,  1.0,  1.0,
        0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0,
        0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0, -4.0,  0.0,  0.0,  0.0,  0.0,  4.0,  0.0,  0.0,  0.0,  0.0,  0.0]);

    let (vecs, vals) = m.clone().schur().unpack();
    assert!(relative_eq!(&vecs * vals * vecs.transpose(), m, epsilon = 1.0e-7))
}
