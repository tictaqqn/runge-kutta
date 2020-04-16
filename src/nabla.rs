extern crate nalgebra as na;
use super::auto_diff::Dual;

pub struct Func3D<F>
where
    F: Fn(&na::Vector2<Dual>) -> Dual,
{
    func: F,
}

impl<F> Func3D<F>
where
    F: Fn(&na::Vector2<Dual>) -> Dual,
{
    pub fn new(func: F) -> Self {
        Func3D { func: func }
    }

    pub fn nabla(&self, r: &na::Vector2<f64>) -> na::Vector2<f64> {
        let dx = na::Vector2::new(Dual::new(r[0], 1.0), Dual::new(r[1], 0.0));
        let dy = na::Vector2::new(Dual::new(r[0], 0.0), Dual::new(r[1], 1.0));

        na::Vector2::new((self.func)(&dx).eps, (self.func)(&dy).eps)
    }
}
