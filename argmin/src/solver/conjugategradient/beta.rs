// Copyright 2018-2022 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Beta update methods for [`NonlinearConjugateGradient`](`crate::solver::conjugategradient::NonlinearConjugateGradient`)
//!
//! TODO: Proper documentation.
//!
//! # Reference
//!
//! \[0\] Jorge Nocedal and Stephen J. Wright (2006). Numerical Optimization.
//! Springer. ISBN 0-387-30303-0.

use crate::core::{ArgminFloat, SerializeAlias};
use argmin_math::{ArgminDot, ArgminNorm, ArgminSub};
#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

/// Common interface for beta update methods (Nonlinear-CG)
pub trait NLCGBetaUpdate<G, P, F>: SerializeAlias {
    /// Update beta
    /// Parameter 1: \nabla f_k
    /// Parameter 2: \nabla f_{k+1}
    /// Parameter 3: p_k
    fn update(&self, nabla_f_k: &G, nabla_f_k_p_1: &G, p_k: &P) -> F;
}

/// Fletcher and Reeves (FR) method
///
/// TODO: Reference
#[derive(Default, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct FletcherReeves {}

impl FletcherReeves {
    /// Constructor
    pub fn new() -> Self {
        FletcherReeves {}
    }
}

impl<G, P, F> NLCGBetaUpdate<G, P, F> for FletcherReeves
where
    G: ArgminDot<G, F>,
    F: ArgminFloat,
{
    fn update(&self, dfk: &G, dfk1: &G, _pk: &P) -> F {
        dfk1.dot(dfk1) / dfk.dot(dfk)
    }
}

/// Polak and Ribiere (PR) method
///
/// TODO: Reference
#[derive(Default, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct PolakRibiere {}

impl PolakRibiere {
    /// Constructor
    pub fn new() -> Self {
        PolakRibiere {}
    }
}

impl<G, P, F> NLCGBetaUpdate<G, P, F> for PolakRibiere
where
    G: ArgminDot<G, F> + ArgminSub<G, G> + ArgminNorm<F>,
    F: ArgminFloat,
{
    fn update(&self, dfk: &G, dfk1: &G, _pk: &P) -> F {
        let dfk_norm_sq = dfk.norm().powi(2);
        dfk1.dot(&dfk1.sub(dfk)) / dfk_norm_sq
    }
}

/// Polak and Ribiere Plus (PR+) method
///
/// TODO: Reference
#[derive(Default, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct PolakRibierePlus {}

impl PolakRibierePlus {
    /// Constructor
    pub fn new() -> Self {
        PolakRibierePlus {}
    }
}

impl<G, P, F> NLCGBetaUpdate<G, P, F> for PolakRibierePlus
where
    G: ArgminDot<G, F> + ArgminSub<G, G> + ArgminNorm<F>,
    F: ArgminFloat,
{
    fn update(&self, dfk: &G, dfk1: &G, _pk: &P) -> F {
        let dfk_norm_sq = dfk.norm().powi(2);
        let beta = dfk1.dot(&dfk1.sub(dfk)) / dfk_norm_sq;
        F::from_f64(0.0).unwrap().max(beta)
    }
}

/// Hestenes and Stiefel (HS) method
///
/// TODO: Reference
#[derive(Default, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct HestenesStiefel {}

impl HestenesStiefel {
    /// Constructor
    pub fn new() -> Self {
        HestenesStiefel {}
    }
}

impl<G, P, F> NLCGBetaUpdate<G, P, F> for HestenesStiefel
where
    G: ArgminDot<G, F> + ArgminDot<P, F> + ArgminSub<G, G>,
    F: ArgminFloat,
{
    fn update(&self, dfk: &G, dfk1: &G, pk: &P) -> F {
        let d = dfk1.sub(dfk);
        dfk1.dot(&d) / d.dot(pk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_trait_impl;

    test_trait_impl!(fletcher_reeves, FletcherReeves);
    test_trait_impl!(polak_ribiere, PolakRibiere);
    test_trait_impl!(polak_ribiere_plus, PolakRibierePlus);
    test_trait_impl!(hestenes_stiefel, HestenesStiefel);
}
