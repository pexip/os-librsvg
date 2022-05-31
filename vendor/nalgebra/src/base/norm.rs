#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

use num::Zero;
use std::ops::Neg;

use crate::allocator::Allocator;
use crate::base::{DefaultAllocator, Dim, DimName, Matrix, MatrixMN, Normed, VectorN};
use crate::constraint::{SameNumberOfColumns, SameNumberOfRows, ShapeConstraint};
use crate::storage::{Storage, StorageMut};
use crate::{ComplexField, Scalar, SimdComplexField, Unit};
use simba::scalar::ClosedNeg;
use simba::simd::{SimdOption, SimdPartialOrd};

// FIXME: this should be be a trait on alga?
/// A trait for abstract matrix norms.
///
/// This may be moved to the alga crate in the future.
pub trait Norm<N: SimdComplexField> {
    /// Apply this norm to the given matrix.
    fn norm<R, C, S>(&self, m: &Matrix<N, R, C, S>) -> N::SimdRealField
    where
        R: Dim,
        C: Dim,
        S: Storage<N, R, C>;
    /// Use the metric induced by this norm to compute the metric distance between the two given matrices.
    fn metric_distance<R1, C1, S1, R2, C2, S2>(
        &self,
        m1: &Matrix<N, R1, C1, S1>,
        m2: &Matrix<N, R2, C2, S2>,
    ) -> N::SimdRealField
    where
        R1: Dim,
        C1: Dim,
        S1: Storage<N, R1, C1>,
        R2: Dim,
        C2: Dim,
        S2: Storage<N, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2>;
}

/// Euclidean norm.
pub struct EuclideanNorm;
/// Lp norm.
pub struct LpNorm(pub i32);
/// L-infinite norm aka. Chebytchev norm aka. uniform norm aka. suppremum norm.
pub struct UniformNorm;

impl<N: SimdComplexField> Norm<N> for EuclideanNorm {
    #[inline]
    fn norm<R, C, S>(&self, m: &Matrix<N, R, C, S>) -> N::SimdRealField
    where
        R: Dim,
        C: Dim,
        S: Storage<N, R, C>,
    {
        m.norm_squared().simd_sqrt()
    }

    #[inline]
    fn metric_distance<R1, C1, S1, R2, C2, S2>(
        &self,
        m1: &Matrix<N, R1, C1, S1>,
        m2: &Matrix<N, R2, C2, S2>,
    ) -> N::SimdRealField
    where
        R1: Dim,
        C1: Dim,
        S1: Storage<N, R1, C1>,
        R2: Dim,
        C2: Dim,
        S2: Storage<N, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2>,
    {
        m1.zip_fold(m2, N::SimdRealField::zero(), |acc, a, b| {
            let diff = a - b;
            acc + diff.simd_modulus_squared()
        })
        .simd_sqrt()
    }
}

impl<N: SimdComplexField> Norm<N> for LpNorm {
    #[inline]
    fn norm<R, C, S>(&self, m: &Matrix<N, R, C, S>) -> N::SimdRealField
    where
        R: Dim,
        C: Dim,
        S: Storage<N, R, C>,
    {
        m.fold(N::SimdRealField::zero(), |a, b| {
            a + b.simd_modulus().simd_powi(self.0)
        })
        .simd_powf(crate::convert(1.0 / (self.0 as f64)))
    }

    #[inline]
    fn metric_distance<R1, C1, S1, R2, C2, S2>(
        &self,
        m1: &Matrix<N, R1, C1, S1>,
        m2: &Matrix<N, R2, C2, S2>,
    ) -> N::SimdRealField
    where
        R1: Dim,
        C1: Dim,
        S1: Storage<N, R1, C1>,
        R2: Dim,
        C2: Dim,
        S2: Storage<N, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2>,
    {
        m1.zip_fold(m2, N::SimdRealField::zero(), |acc, a, b| {
            let diff = a - b;
            acc + diff.simd_modulus().simd_powi(self.0)
        })
        .simd_powf(crate::convert(1.0 / (self.0 as f64)))
    }
}

impl<N: SimdComplexField> Norm<N> for UniformNorm {
    #[inline]
    fn norm<R, C, S>(&self, m: &Matrix<N, R, C, S>) -> N::SimdRealField
    where
        R: Dim,
        C: Dim,
        S: Storage<N, R, C>,
    {
        // NOTE: we don't use `m.amax()` here because for the complex
        // numbers this will return the max norm1 instead of the modulus.
        m.fold(N::SimdRealField::zero(), |acc, a| {
            acc.simd_max(a.simd_modulus())
        })
    }

    #[inline]
    fn metric_distance<R1, C1, S1, R2, C2, S2>(
        &self,
        m1: &Matrix<N, R1, C1, S1>,
        m2: &Matrix<N, R2, C2, S2>,
    ) -> N::SimdRealField
    where
        R1: Dim,
        C1: Dim,
        S1: Storage<N, R1, C1>,
        R2: Dim,
        C2: Dim,
        S2: Storage<N, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R1, R2> + SameNumberOfColumns<C1, C2>,
    {
        m1.zip_fold(m2, N::SimdRealField::zero(), |acc, a, b| {
            let val = (a - b).simd_modulus();
            acc.simd_max(val)
        })
    }
}

impl<N: SimdComplexField, R: Dim, C: Dim, S: Storage<N, R, C>> Matrix<N, R, C, S> {
    /// The squared L2 norm of this vector.
    #[inline]
    pub fn norm_squared(&self) -> N::SimdRealField {
        let mut res = N::SimdRealField::zero();

        for i in 0..self.ncols() {
            let col = self.column(i);
            res += col.dotc(&col).simd_real()
        }

        res
    }

    /// The L2 norm of this matrix.
    ///
    /// Use `.apply_norm` to apply a custom norm.
    #[inline]
    pub fn norm(&self) -> N::SimdRealField {
        self.norm_squared().simd_sqrt()
    }

    /// Compute the distance between `self` and `rhs` using the metric induced by the euclidean norm.
    ///
    /// Use `.apply_metric_distance` to apply a custom norm.
    #[inline]
    pub fn metric_distance<R2, C2, S2>(&self, rhs: &Matrix<N, R2, C2, S2>) -> N::SimdRealField
    where
        R2: Dim,
        C2: Dim,
        S2: Storage<N, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        self.apply_metric_distance(rhs, &EuclideanNorm)
    }

    /// Uses the given `norm` to compute the norm of `self`.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::{Vector3, UniformNorm, LpNorm, EuclideanNorm};
    ///
    /// let v = Vector3::new(1.0, 2.0, 3.0);
    /// assert_eq!(v.apply_norm(&UniformNorm), 3.0);
    /// assert_eq!(v.apply_norm(&LpNorm(1)), 6.0);
    /// assert_eq!(v.apply_norm(&EuclideanNorm), v.norm());
    /// ```
    #[inline]
    pub fn apply_norm(&self, norm: &impl Norm<N>) -> N::SimdRealField {
        norm.norm(self)
    }

    /// Uses the metric induced by the given `norm` to compute the metric distance between `self` and `rhs`.
    ///
    /// # Example
    ///
    /// ```
    /// # use nalgebra::{Vector3, UniformNorm, LpNorm, EuclideanNorm};
    ///
    /// let v1 = Vector3::new(1.0, 2.0, 3.0);
    /// let v2 = Vector3::new(10.0, 20.0, 30.0);
    ///
    /// assert_eq!(v1.apply_metric_distance(&v2, &UniformNorm), 27.0);
    /// assert_eq!(v1.apply_metric_distance(&v2, &LpNorm(1)), 27.0 + 18.0 + 9.0);
    /// assert_eq!(v1.apply_metric_distance(&v2, &EuclideanNorm), (v1 - v2).norm());
    /// ```
    #[inline]
    pub fn apply_metric_distance<R2, C2, S2>(
        &self,
        rhs: &Matrix<N, R2, C2, S2>,
        norm: &impl Norm<N>,
    ) -> N::SimdRealField
    where
        R2: Dim,
        C2: Dim,
        S2: Storage<N, R2, C2>,
        ShapeConstraint: SameNumberOfRows<R, R2> + SameNumberOfColumns<C, C2>,
    {
        norm.metric_distance(self, rhs)
    }

    /// A synonym for the norm of this matrix.
    ///
    /// Aka the length.
    ///
    /// This function is simply implemented as a call to `norm()`
    #[inline]
    pub fn magnitude(&self) -> N::SimdRealField {
        self.norm()
    }

    /// A synonym for the squared norm of this matrix.
    ///
    /// Aka the squared length.
    ///
    /// This function is simply implemented as a call to `norm_squared()`
    #[inline]
    pub fn magnitude_squared(&self) -> N::SimdRealField {
        self.norm_squared()
    }

    /// Sets the magnitude of this vector.
    #[inline]
    pub fn set_magnitude(&mut self, magnitude: N::SimdRealField)
    where
        S: StorageMut<N, R, C>,
    {
        let n = self.norm();
        self.scale_mut(magnitude / n)
    }

    /// Returns a normalized version of this matrix.
    #[inline]
    #[must_use = "Did you mean to use normalize_mut()?"]
    pub fn normalize(&self) -> MatrixMN<N, R, C>
    where
        DefaultAllocator: Allocator<N, R, C>,
    {
        self.unscale(self.norm())
    }

    /// The Lp norm of this matrix.
    #[inline]
    pub fn lp_norm(&self, p: i32) -> N::SimdRealField {
        self.apply_norm(&LpNorm(p))
    }

    /// Attempts to normalize `self`.
    ///
    /// The components of this matrix can be SIMD types.
    #[inline]
    #[must_use = "Did you mean to use simd_try_normalize_mut()?"]
    pub fn simd_try_normalize(&self, min_norm: N::SimdRealField) -> SimdOption<MatrixMN<N, R, C>>
    where
        N::Element: Scalar,
        DefaultAllocator: Allocator<N, R, C> + Allocator<N::Element, R, C>,
    {
        let n = self.norm();
        let le = n.simd_le(min_norm);
        let val = self.unscale(n);
        SimdOption::new(val, le)
    }
}

impl<N: ComplexField, R: Dim, C: Dim, S: Storage<N, R, C>> Matrix<N, R, C, S> {
    /// Sets the magnitude of this vector unless it is smaller than `min_magnitude`.
    ///
    /// If `self.magnitude()` is smaller than `min_magnitude`, it will be left unchanged.
    /// Otherwise this is equivalent to: `*self = self.normalize() * magnitude.
    #[inline]
    pub fn try_set_magnitude(&mut self, magnitude: N::RealField, min_magnitude: N::RealField)
    where
        S: StorageMut<N, R, C>,
    {
        let n = self.norm();

        if n >= min_magnitude {
            self.scale_mut(magnitude / n)
        }
    }

    /// Returns a normalized version of this matrix unless its norm as smaller or equal to `eps`.
    ///
    /// The components of this matrix cannot be SIMD types (see `simd_try_normalize`) instead.
    #[inline]
    #[must_use = "Did you mean to use try_normalize_mut()?"]
    pub fn try_normalize(&self, min_norm: N::RealField) -> Option<MatrixMN<N, R, C>>
    where
        DefaultAllocator: Allocator<N, R, C>,
    {
        let n = self.norm();

        if n <= min_norm {
            None
        } else {
            Some(self.unscale(n))
        }
    }
}

impl<N: SimdComplexField, R: Dim, C: Dim, S: StorageMut<N, R, C>> Matrix<N, R, C, S> {
    /// Normalizes this matrix in-place and returns its norm.
    ///
    /// The components of the matrix cannot be SIMD types (see `simd_try_normalize_mut` instead).
    #[inline]
    pub fn normalize_mut(&mut self) -> N::SimdRealField {
        let n = self.norm();
        self.unscale_mut(n);

        n
    }

    /// Normalizes this matrix in-place and return its norm.
    ///
    /// The components of the matrix can be SIMD types.
    #[inline]
    #[must_use = "Did you mean to use simd_try_normalize_mut()?"]
    pub fn simd_try_normalize_mut(
        &mut self,
        min_norm: N::SimdRealField,
    ) -> SimdOption<N::SimdRealField>
    where
        N::Element: Scalar,
        DefaultAllocator: Allocator<N, R, C> + Allocator<N::Element, R, C>,
    {
        let n = self.norm();
        let le = n.simd_le(min_norm);
        self.apply(|e| e.simd_unscale(n).select(le, e));
        SimdOption::new(n, le)
    }
}

impl<N: ComplexField, R: Dim, C: Dim, S: StorageMut<N, R, C>> Matrix<N, R, C, S> {
    /// Normalizes this matrix in-place or does nothing if its norm is smaller or equal to `eps`.
    ///
    /// If the normalization succeeded, returns the old norm of this matrix.
    #[inline]
    pub fn try_normalize_mut(&mut self, min_norm: N::RealField) -> Option<N::RealField> {
        let n = self.norm();

        if n <= min_norm {
            None
        } else {
            self.unscale_mut(n);
            Some(n)
        }
    }
}

impl<N: SimdComplexField, R: Dim, C: Dim> Normed for MatrixMN<N, R, C>
where
    DefaultAllocator: Allocator<N, R, C>,
{
    type Norm = N::SimdRealField;

    #[inline]
    fn norm(&self) -> N::SimdRealField {
        self.norm()
    }

    #[inline]
    fn norm_squared(&self) -> N::SimdRealField {
        self.norm_squared()
    }

    #[inline]
    fn scale_mut(&mut self, n: Self::Norm) {
        self.scale_mut(n)
    }

    #[inline]
    fn unscale_mut(&mut self, n: Self::Norm) {
        self.unscale_mut(n)
    }
}

impl<N: Scalar + ClosedNeg, R: Dim, C: Dim> Neg for Unit<MatrixMN<N, R, C>>
where
    DefaultAllocator: Allocator<N, R, C>,
{
    type Output = Unit<MatrixMN<N, R, C>>;

    #[inline]
    fn neg(self) -> Self::Output {
        Unit::new_unchecked(-self.value)
    }
}

// FIXME: specialization will greatly simplify this implementation in the future.
// In particular:
//   − use `x()` instead of `::canonical_basis_element`
//   − use `::new(x, y, z)` instead of `::from_slice`
impl<N: ComplexField, D: DimName> VectorN<N, D>
where
    DefaultAllocator: Allocator<N, D>,
{
    /// The i-the canonical basis element.
    #[inline]
    fn canonical_basis_element(i: usize) -> Self {
        assert!(i < D::dim(), "Index out of bound.");

        let mut res = Self::zero();
        unsafe {
            *res.data.get_unchecked_linear_mut(i) = N::one();
        }

        res
    }

    /// Orthonormalizes the given family of vectors. The largest free family of vectors is moved at
    /// the beginning of the array and its size is returned. Vectors at an indices larger or equal to
    /// this length can be modified to an arbitrary value.
    #[inline]
    pub fn orthonormalize(vs: &mut [Self]) -> usize {
        let mut nbasis_elements = 0;

        for i in 0..vs.len() {
            {
                let (elt, basis) = vs[..i + 1].split_last_mut().unwrap();

                for basis_element in &basis[..nbasis_elements] {
                    *elt -= &*basis_element * elt.dot(basis_element)
                }
            }

            if vs[i].try_normalize_mut(N::RealField::zero()).is_some() {
                // FIXME: this will be efficient on dynamically-allocated vectors but for
                // statically-allocated ones, `.clone_from` would be better.
                vs.swap(nbasis_elements, i);
                nbasis_elements += 1;

                // All the other vectors will be dependent.
                if nbasis_elements == D::dim() {
                    break;
                }
            }
        }

        nbasis_elements
    }

    /// Applies the given closure to each element of the orthonormal basis of the subspace
    /// orthogonal to free family of vectors `vs`. If `vs` is not a free family, the result is
    /// unspecified.
    // FIXME: return an iterator instead when `-> impl Iterator` will be supported by Rust.
    #[inline]
    pub fn orthonormal_subspace_basis<F>(vs: &[Self], mut f: F)
    where
        F: FnMut(&Self) -> bool,
    {
        // FIXME: is this necessary?
        assert!(
            vs.len() <= D::dim(),
            "The given set of vectors has no chance of being a free family."
        );

        match D::dim() {
            1 => {
                if vs.len() == 0 {
                    let _ = f(&Self::canonical_basis_element(0));
                }
            }
            2 => {
                if vs.len() == 0 {
                    let _ = f(&Self::canonical_basis_element(0))
                        && f(&Self::canonical_basis_element(1));
                } else if vs.len() == 1 {
                    let v = &vs[0];
                    let res = Self::from_column_slice(&[-v[1], v[0]]);

                    let _ = f(&res.normalize());
                }

                // Otherwise, nothing.
            }
            3 => {
                if vs.len() == 0 {
                    let _ = f(&Self::canonical_basis_element(0))
                        && f(&Self::canonical_basis_element(1))
                        && f(&Self::canonical_basis_element(2));
                } else if vs.len() == 1 {
                    let v = &vs[0];
                    let mut a;

                    if v[0].norm1() > v[1].norm1() {
                        a = Self::from_column_slice(&[v[2], N::zero(), -v[0]]);
                    } else {
                        a = Self::from_column_slice(&[N::zero(), -v[2], v[1]]);
                    };

                    let _ = a.normalize_mut();

                    if f(&a.cross(v)) {
                        let _ = f(&a);
                    }
                } else if vs.len() == 2 {
                    let _ = f(&vs[0].cross(&vs[1]).normalize());
                }
            }
            _ => {
                #[cfg(any(feature = "std", feature = "alloc"))]
                {
                    // XXX: use a GenericArray instead.
                    let mut known_basis = Vec::new();

                    for v in vs.iter() {
                        known_basis.push(v.normalize())
                    }

                    for i in 0..D::dim() - vs.len() {
                        let mut elt = Self::canonical_basis_element(i);

                        for v in &known_basis {
                            elt -= v * elt.dot(v)
                        }

                        if let Some(subsp_elt) = elt.try_normalize(N::RealField::zero()) {
                            if !f(&subsp_elt) {
                                return;
                            };

                            known_basis.push(subsp_elt);
                        }
                    }
                }
                #[cfg(all(not(feature = "std"), not(feature = "alloc")))]
                {
                    panic!("Cannot compute the orthogonal subspace basis of a vector with a dimension greater than 3 \
                            if #![no_std] is enabled and the 'alloc' feature is not enabled.")
                }
            }
        }
    }
}