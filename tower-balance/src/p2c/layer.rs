use super::BalanceMake;
use rand::{rngs::SmallRng, FromEntropy, Rng, SeedableRng};
use std::{fmt, marker::PhantomData};
use tower_layer::Layer;

/// Efficiently distributes requests across an arbitrary number of services
#[derive(Clone)]
pub struct BalanceLayer<D> {
    rng: SmallRng,
    _marker: PhantomData<fn(D)>,
}

impl<D> BalanceLayer<D> {
    /// Builds a balancer using the system entropy.
    pub fn new() -> Self {
        Self {
            rng: SmallRng::from_entropy(),
            _marker: PhantomData,
        }
    }

    /// Builds a balancer from the provided RNG.
    ///
    /// This may be preferrable when many balancers are initialized.
    pub fn from_rng<R: Rng>(rng: &mut R) -> Result<Self, rand::Error> {
        let rng = SmallRng::from_rng(rng)?;
        Ok(Self {
            rng,
            _marker: PhantomData,
        })
    }
}

impl<S> Layer<S> for BalanceLayer<S> {
    type Service = BalanceMake<S>;

    fn layer(&self, make_discover: S) -> Self::Service {
        BalanceMake::new(make_discover, self.rng.clone())
    }
}

impl<D> fmt::Debug for BalanceLayer<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BalanceLayer")
            .field("rng", &self.rng)
            .finish()
    }
}
