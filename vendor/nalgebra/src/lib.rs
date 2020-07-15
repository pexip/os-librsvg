/*!
# nalgebra

**nalgebra** is a linear algebra library written for Rust targeting:

* General-purpose linear algebra (still lacks a lot of features…)
* Real time computer graphics.
* Real time computer physics.

## Using **nalgebra**
You will need the last stable build of the [rust compiler](http://www.rust-lang.org)
and the official package manager: [cargo](https://github.com/rust-lang/cargo).

Simply add the following to your `Cargo.toml` file:

```.ignore
[dependencies]
nalgebra = "0.16"
```


Most useful functionalities of **nalgebra** are grouped in the root module `nalgebra::`.

However, the recommended way to use **nalgebra** is to import types and traits
explicitly, and call free-functions using the `na::` prefix:

```.rust
#[macro_use]
extern crate approx; // For the macro relative_eq!
extern crate nalgebra as na;
use na::{Vector3, Rotation3};

fn main() {
    let axis  = Vector3::x_axis();
    let angle = 1.57;
    let b     = Rotation3::from_axis_angle(&axis, angle);

    relative_eq!(b.axis().unwrap(), axis);
    relative_eq!(b.angle(), angle);
}
```


## Features
**nalgebra** is meant to be a general-purpose, low-dimensional, linear algebra library, with
an optimized set of tools for computer graphics and physics. Those features include:

* A single parametrizable type `Matrix` for vectors, (square or rectangular) matrices, and slices
  with dimensions known either at compile-time (using type-level integers) or at runtime.
* Matrices and vectors with compile-time sizes are statically allocated while dynamic ones are
  allocated on the heap.
* Convenient aliases for low-dimensional matrices and vectors: `Vector1` to `Vector6` and
  `Matrix1x1` to `Matrix6x6`, including rectangular matrices like `Matrix2x5`.
* Points sizes known at compile time, and convenience aliases: `Point1` to `Point6`.
* Translation (seen as a transformation that composes by multiplication): `Translation2`,
  `Translation3`.
* Rotation matrices: `Rotation2`, `Rotation3`.
* Quaternions: `Quaternion`, `UnitQuaternion` (for 3D rotation).
* Unit complex numbers can be used for 2D rotation: `UnitComplex`.
* Algebraic entities with a norm equal to one: `Unit<T>`, e.g., `Unit<Vector3<f32>>`.
* Isometries (translation ⨯ rotation): `Isometry2`, `Isometry3`
* Similarity transformations (translation ⨯ rotation ⨯ uniform scale): `Similarity2`, `Similarity3`.
* Affine transformations stored as an homogeneous matrix: `Affine2`, `Affine3`.
* Projective (i.e. invertible) transformations stored as an homogeneous matrix: `Projective2`,
  `Projective3`.
* General transformations that does not have to be invertible, stored as an homogeneous matrix:
  `Transform2`, `Transform3`.
* 3D projections for computer graphics: `Perspective3`, `Orthographic3`.
* Matrix factorizations: `Cholesky`, `QR`, `LU`, `FullPivLU`, `SVD`, `RealSchur`, `Hessenberg`, `SymmetricEigen`.
* Insertion and removal of rows of columns of a matrix.
* Implements traits from the [alga](https://crates.io/crates/alga) crate for
  generic programming.
*/

// #![feature(plugin)]
//
// #![plugin(clippy)]

#![deny(non_camel_case_types)]
#![deny(unused_parens)]
#![deny(non_upper_case_globals)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(missing_docs)]
#![warn(incoherent_fundamental_impls)]
#![doc(html_root_url = "http://nalgebra.org/rustdoc")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", not(feature = "std")), feature(alloc))]

#[cfg(feature = "arbitrary")]
extern crate quickcheck;

#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "serde")]
#[macro_use]
extern crate serde_derive;

#[cfg(feature = "abomonation-serialize")]
extern crate abomonation;

#[cfg(feature = "mint")]
extern crate mint;

#[macro_use]
extern crate approx;
extern crate generic_array;
#[cfg(feature = "std")]
extern crate matrixmultiply;
extern crate num_complex;
extern crate num_traits as num;
extern crate rand;
extern crate typenum;

extern crate alga;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(not(feature = "std"))]
extern crate core as std;

pub mod base;
#[cfg(feature = "debug")]
pub mod debug;
pub mod geometry;
pub mod linalg;

#[cfg(feature = "std")]
#[deprecated(
    note = "The 'core' module is being renamed 'based' to avoid conflicts with the 'core' crate."
)]
pub use base as core;
pub use base::*;
pub use geometry::*;
pub use linalg::*;

use std::cmp::{self, Ordering, PartialOrd};

use alga::general::{
    Additive, AdditiveGroup, Identity, Inverse, JoinSemilattice, Lattice, MeetSemilattice,
    Multiplicative, SupersetOf,
};
use alga::linear::SquareMatrix as AlgaSquareMatrix;
use alga::linear::{EuclideanSpace, FiniteDimVectorSpace, InnerSpace, NormedSpace};
use num::Signed;

pub use alga::general::{Id, Real};

/*
 *
 * Multiplicative identity.
 *
 */
/// Gets the ubiquitous multiplicative identity element.
///
/// Same as `Id::new()`.
#[inline]
pub fn id() -> Id {
    Id::new()
}

/// Gets the multiplicative identity element.
#[inline]
pub fn one<T: Identity<Multiplicative>>() -> T {
    T::identity()
}

/// Gets the additive identity element.
#[inline]
pub fn zero<T: Identity<Additive>>() -> T {
    T::identity()
}

/// Gets the origin of the given point.
#[inline]
pub fn origin<P: EuclideanSpace>() -> P {
    P::origin()
}

/*
 *
 * Dimension
 *
 */
/// The dimension of the given algebraic entity seen as a vector space.
#[inline]
pub fn dimension<V: FiniteDimVectorSpace>() -> usize {
    V::dimension()
}

/*
 *
 * Ordering
 *
 */
// XXX: this is very naive and could probably be optimized for specific types.
// XXX: also, we might just want to use divisions, but assuming `val` is usually not far from `min`
// or `max`, would it still be more efficient?
/// Wraps `val` into the range `[min, max]` using modular arithmetics.
///
/// The range must not be empty.
#[inline]
pub fn wrap<T>(mut val: T, min: T, max: T) -> T
where
    T: Copy + PartialOrd + AdditiveGroup,
{
    assert!(min < max, "Invalid wrapping bounds.");
    let width = max - min;

    if val < min {
        val += width;

        while val < min {
            val += width
        }

        val
    } else if val > max {
        val -= width;

        while val > max {
            val -= width
        }

        val
    } else {
        val
    }
}

/// Returns a reference to the input value clamped to the interval `[min, max]`.
///
/// In particular:
///     * If `min < val < max`, this returns `val`.
///     * If `val <= min`, this returns `min`.
///     * If `val >= max`, this returns `max`.
#[inline]
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val > min {
        if val < max {
            val
        } else {
            max
        }
    } else {
        min
    }
}

/// Same as `cmp::max`.
#[inline]
pub fn max<T: Ord>(a: T, b: T) -> T {
    cmp::max(a, b)
}

/// Same as `cmp::min`.
#[inline]
pub fn min<T: Ord>(a: T, b: T) -> T {
    cmp::min(a, b)
}

/// The absolute value of `a`.
#[inline]
pub fn abs<T: Signed>(a: &T) -> T {
    a.abs()
}

/// Returns the infimum of `a` and `b`.
#[inline]
pub fn inf<T: MeetSemilattice>(a: &T, b: &T) -> T {
    a.meet(b)
}

/// Returns the supremum of `a` and `b`.
#[inline]
pub fn sup<T: JoinSemilattice>(a: &T, b: &T) -> T {
    a.join(b)
}

/// Returns simultaneously the infimum and supremum of `a` and `b`.
#[inline]
pub fn inf_sup<T: Lattice>(a: &T, b: &T) -> (T, T) {
    a.meet_join(b)
}

/// Compare `a` and `b` using a partial ordering relation.
#[inline]
pub fn partial_cmp<T: PartialOrd>(a: &T, b: &T) -> Option<Ordering> {
    a.partial_cmp(b)
}

/// Returns `true` iff `a` and `b` are comparable and `a < b`.
#[inline]
pub fn partial_lt<T: PartialOrd>(a: &T, b: &T) -> bool {
    a.lt(b)
}

/// Returns `true` iff `a` and `b` are comparable and `a <= b`.
#[inline]
pub fn partial_le<T: PartialOrd>(a: &T, b: &T) -> bool {
    a.le(b)
}

/// Returns `true` iff `a` and `b` are comparable and `a > b`.
#[inline]
pub fn partial_gt<T: PartialOrd>(a: &T, b: &T) -> bool {
    a.gt(b)
}

/// Returns `true` iff `a` and `b` are comparable and `a >= b`.
#[inline]
pub fn partial_ge<T: PartialOrd>(a: &T, b: &T) -> bool {
    a.ge(b)
}

/// Return the minimum of `a` and `b` if they are comparable.
#[inline]
pub fn partial_min<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> Option<&'a T> {
    if let Some(ord) = a.partial_cmp(b) {
        match ord {
            Ordering::Greater => Some(b),
            _ => Some(a),
        }
    } else {
        None
    }
}

/// Return the maximum of `a` and `b` if they are comparable.
#[inline]
pub fn partial_max<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> Option<&'a T> {
    if let Some(ord) = a.partial_cmp(b) {
        match ord {
            Ordering::Less => Some(b),
            _ => Some(a),
        }
    } else {
        None
    }
}

/// Clamp `value` between `min` and `max`. Returns `None` if `value` is not comparable to
/// `min` or `max`.
#[inline]
pub fn partial_clamp<'a, T: PartialOrd>(value: &'a T, min: &'a T, max: &'a T) -> Option<&'a T> {
    if let (Some(cmp_min), Some(cmp_max)) = (value.partial_cmp(min), value.partial_cmp(max)) {
        if cmp_min == Ordering::Less {
            Some(min)
        } else if cmp_max == Ordering::Greater {
            Some(max)
        } else {
            Some(value)
        }
    } else {
        None
    }
}

/// Sorts two values in increasing order using a partial ordering.
#[inline]
pub fn partial_sort2<'a, T: PartialOrd>(a: &'a T, b: &'a T) -> Option<(&'a T, &'a T)> {
    if let Some(ord) = a.partial_cmp(b) {
        match ord {
            Ordering::Less => Some((a, b)),
            _ => Some((b, a)),
        }
    } else {
        None
    }
}

/*
 * Inverse
 */

/// Tries to gets an inverted copy of a square matrix.
#[inline]
pub fn try_inverse<M: AlgaSquareMatrix>(m: &M) -> Option<M> {
    m.try_inverse()
}

/// Computes the multiplicative inverse of an (always invertible) algebraic entity.
#[inline]
pub fn inverse<M: Inverse<Multiplicative>>(m: &M) -> M {
    m.inverse()
}

/*
 * Inner vector space
 */

/// Computes the dot product of two vectors.
#[inline]
pub fn dot<V: FiniteDimVectorSpace>(a: &V, b: &V) -> V::Field {
    a.dot(b)
}

/// Computes the smallest angle between two vectors.
#[inline]
pub fn angle<V: InnerSpace>(a: &V, b: &V) -> V::Real {
    a.angle(b)
}

/*
 * Normed space
 */

/// Computes the L2 (euclidean) norm of a vector.
#[inline]
pub fn norm<V: NormedSpace>(v: &V) -> V::Field {
    v.norm()
}

/// Computes the squared L2 (euclidean) norm of the vector `v`.
#[inline]
pub fn norm_squared<V: NormedSpace>(v: &V) -> V::Field {
    v.norm_squared()
}

/// A synonym function for `norm()` aka length.
#[inline]
pub fn magnitude<V: NormedSpace>(v: &V) -> V::Field {
    v.norm()
}

/// A synonym function for `norm_squared()` aka length squared.
#[inline]
pub fn magnitude_squared<V: NormedSpace>(v: &V) -> V::Field {
    v.norm_squared()
}

/// Computes the normalized version of the vector `v`.
#[inline]
pub fn normalize<V: NormedSpace>(v: &V) -> V {
    v.normalize()
}

/// Computes the normalized version of the vector `v` or returns `None` if its norm is smaller than `min_norm`.
#[inline]
pub fn try_normalize<V: NormedSpace>(v: &V, min_norm: V::Field) -> Option<V> {
    v.try_normalize(min_norm)
}

/*
 *
 * Point operations.
 *
 */
/// The center of two points.
#[inline]
pub fn center<P: EuclideanSpace>(p1: &P, p2: &P) -> P {
    P::from_coordinates((p1.coordinates() + p2.coordinates()) * convert(0.5))
}

/// The distance between two points.
#[inline]
pub fn distance<P: EuclideanSpace>(p1: &P, p2: &P) -> P::Real {
    (p2.coordinates() - p1.coordinates()).norm()
}

/// The squared distance between two points.
#[inline]
pub fn distance_squared<P: EuclideanSpace>(p1: &P, p2: &P) -> P::Real {
    (p2.coordinates() - p1.coordinates()).norm_squared()
}

/*
 * Cast
 */
/// Converts an object from one type to an equivalent or more general one.
///
/// See also `::try_convert` for conversion to more specific types.
#[inline]
pub fn convert<From, To: SupersetOf<From>>(t: From) -> To {
    To::from_subset(&t)
}

/// Attempts to convert an object to a more specific one.
///
/// See also `::convert` for conversion to more general types.
#[inline]
pub fn try_convert<From: SupersetOf<To>, To>(t: From) -> Option<To> {
    t.to_subset()
}

/// Indicates if `::try_convert` will succeed without actually performing the conversion.
#[inline]
pub fn is_convertible<From: SupersetOf<To>, To>(t: &From) -> bool {
    t.is_in_subset()
}

/// Use with care! Same as `try_convert` but without any property checks.
#[inline]
pub unsafe fn convert_unchecked<From: SupersetOf<To>, To>(t: From) -> To {
    t.to_subset_unchecked()
}

/// Converts an object from one type to an equivalent or more general one.
#[inline]
pub fn convert_ref<From, To: SupersetOf<From>>(t: &From) -> To {
    To::from_subset(t)
}

/// Attempts to convert an object to a more specific one.
#[inline]
pub fn try_convert_ref<From: SupersetOf<To>, To>(t: &From) -> Option<To> {
    t.to_subset()
}

/// Use with care! Same as `try_convert` but without any property checks.
#[inline]
pub unsafe fn convert_ref_unchecked<From: SupersetOf<To>, To>(t: &From) -> To {
    t.to_subset_unchecked()
}