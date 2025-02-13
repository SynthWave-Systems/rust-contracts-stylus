use num_traits::Zero;

mod affine;
pub use affine::*;

mod group;
pub use group::*;

use crate::{
    curve::{
        scalar_mul::{sw_double_and_add_affine, sw_double_and_add_projective},
        AffineRepr,
    },
    field::{group::AdditiveGroup, prime::PrimeField},
};

/// Constants and convenience functions that collectively define the [Short Weierstrass model](https://www.hyperelliptic.org/EFD/g1p/auto-shortw.html)
/// of the curve.
///
/// In this model, the curve equation is `y² = x³ + a * x + b`, for constants
/// `a` and `b`.
pub trait SWCurveConfig: super::CurveConfig {
    /// Coefficient `a` of the curve equation.
    const COEFF_A: Self::BaseField;
    /// Coefficient `b` of the curve equation.
    const COEFF_B: Self::BaseField;
    /// Generator of the prime-order subgroup.
    const GENERATOR: Affine<Self>;

    /// Helper method for computing `elem * Self::COEFF_A`.
    ///
    /// The default implementation should be overridden only if
    /// the product can be computed faster than standard field multiplication
    /// (eg: via doubling if `COEFF_A == 2`, or if `COEFF_A.is_zero()`).
    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        if Self::COEFF_A.is_zero() {
            Self::BaseField::ZERO
        } else {
            elem * Self::COEFF_A
        }
    }

    /// Helper method for computing `elem + Self::COEFF_B`.
    ///
    /// The default implementation should be overridden only if
    /// the sum can be computed faster than standard field addition (eg: via
    /// doubling).
    #[inline(always)]
    fn add_b(elem: Self::BaseField) -> Self::BaseField {
        if Self::COEFF_B.is_zero() {
            elem
        } else {
            elem + &Self::COEFF_B
        }
    }

    /// Check if the provided curve point is in the prime-order subgroup.
    ///
    /// The default implementation multiplies `item` by the order `r` of the
    /// prime-order subgroup, and checks if the result is zero. If the
    /// curve's cofactor is one, this check automatically returns true.
    /// Implementors can choose to override this default impl
    /// if the given curve has faster methods
    /// for performing this check (for example, via leveraging curve
    /// isomorphisms).
    fn is_in_correct_subgroup_assuming_on_curve(item: &Affine<Self>) -> bool {
        if Self::cofactor_is_one() {
            true
        } else {
            Self::mul_affine(item, Self::ScalarField::characteristic())
                .is_zero()
        }
    }

    /// Performs cofactor clearing.
    /// The default method is simply to multiply by the cofactor.
    /// Some curves can implement a more efficient algorithm.
    fn clear_cofactor(item: &Affine<Self>) -> Affine<Self> {
        item.mul_by_cofactor()
    }

    /// Default implementation of group multiplication for projective
    /// coordinates
    fn mul_projective(
        base: &Projective<Self>,
        scalar: &[u64],
    ) -> Projective<Self> {
        sw_double_and_add_projective(base, scalar)
    }

    /// Default implementation of group multiplication for affine
    /// coordinates.
    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        sw_double_and_add_affine(base, scalar)
    }

    /* TODO#q: implement msm for short weierstrass curves
    /// Default implementation for multi scalar multiplication
    fn msm(
        bases: &[Affine<Self>],
        scalars: &[Self::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        (bases.len() == scalars.len())
            .then(|| VariableBaseMSM::msm_unchecked(bases, scalars))
            .ok_or(bases.len().min(scalars.len()))
    }
    */
}
