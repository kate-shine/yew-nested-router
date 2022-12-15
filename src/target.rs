//! Routing target

use std::fmt::Debug;
use yew::Callback;

/// A target for used by a router.
pub trait Target: Clone + Debug + Eq + 'static {
    /// Render only our path segment.
    fn render_self(&self) -> Vec<String> {
        let mut path = vec![];
        self.render_self_into(&mut path);
        path
    }

    /// Render the full path, including our children.
    fn render_path(&self) -> Vec<String> {
        let mut path = vec![];
        self.render_path_into(&mut path);
        path
    }

    /// Render only our own path component.
    fn render_self_into(&self, path: &mut Vec<String>);

    /// Render the full path downwards.
    fn render_path_into(&self, path: &mut Vec<String>);

    /// Parse the target from the provided (segmented) path.
    ///
    /// The path will be the local path, with the prefix already removed.
    fn parse_path(path: &[&str]) -> Option<Self>;
}

#[derive(Debug, PartialEq)]
pub struct Mapper<P, C> {
    pub downwards: Callback<P, Option<C>>,
    pub upwards: Callback<C, P>,
}

impl<P, C> Clone for Mapper<P, C>
where
    P: Target,
    C: Target,
{
    fn clone(&self) -> Self {
        Self {
            downwards: self.downwards.clone(),
            upwards: self.upwards.clone(),
        }
    }
}

impl<P, C> Mapper<P, C>
where
    P: Target,
    C: Target,
{
    pub fn new<PF, CF>(downwards: PF, upwards: CF) -> Self
    where
        PF: Fn(P) -> Option<C> + 'static,
        CF: Fn(C) -> P + 'static,
    {
        Self {
            downwards: downwards.into(),
            upwards: upwards.into(),
        }
    }
}

impl<P, C> From<Mapper<P, C>> for Callback<(), Mapper<P, C>>
where
    P: Target,
    C: Target,
{
    fn from(mapper: Mapper<P, C>) -> Self {
        Callback::from(move |()| mapper.clone())
    }
}

impl<P, C, PF, CF> From<(PF, CF)> for Mapper<P, C>
where
    P: Target,
    C: Target,
    PF: Fn(P) -> Option<C> + 'static,
    CF: Fn(C) -> P + 'static,
{
    fn from((down, up): (PF, CF)) -> Self {
        Self::new(down, up)
    }
}
