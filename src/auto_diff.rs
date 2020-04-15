use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dual {
    var: f64,
    pub eps: f64,
}

impl Add for Dual {
    type Output = Dual;
    fn add(self, r: Dual) -> Dual {
        Dual {
            var: self.var + r.var,
            eps: self.eps + r.eps,
        }
    }
}

impl Sub for Dual {
    type Output = Dual;
    fn sub(self, r: Dual) -> Dual {
        //   (self_var + self_dual*ε) - (r_var + r_dual*ε)
        // = (self_var - r_var)      + (self_dual - r_dual)*ε
        Dual {
            var: self.var - r.var,
            eps: self.eps - r.eps,
        }
    }
}

impl Mul for Dual {
    type Output = Dual;
    fn mul(self, r: Dual) -> Dual {
        Dual {
            var: self.var * r.var,
            eps: self.eps * r.var + self.var * r.eps,
        }
    }
}

impl Mul<Dual> for f64 {
    type Output = Dual;
    fn mul(self, r: Dual) -> Dual {
        Dual {
            var: self * r.var,
            eps: self * r.eps,
        }
    }
}

impl Div for Dual {
    type Output = Dual;
    fn div(self, r: Dual) -> Dual {
        //   (self_var + self_dual*ε) / (r_var + r_dual*ε)
        // = (self_var / r_var) + (r_dual*self_var/r_var^2)ε
        Dual {
            var: self.var / r.var,
            eps: self.eps / r.var - r.eps * self.var / r.var / r.var,
        }
    }
}

impl Div<f64> for Dual {
    type Output = Dual;
    fn div(self, r: f64) -> Dual {
        Dual {
            var: self.var / r,
            eps: self.eps / r,
        }
    }
}

impl Dual {
    pub fn new(var: f64, eps: f64) -> Self {
        Self { var: var, eps: eps}
    }
    pub fn sin(&self) -> Dual {
        Dual {
            var: self.var.sin(),
            eps: self.eps * self.var.cos(),
        }
    }

    pub fn cos(&self) -> Dual {
        Dual {
            var: self.var.cos(),
            eps: -self.eps * self.var.sin(),
        }
    }

    pub fn exp(&self) -> Dual {
        Dual {
            var: self.var.exp(),
            eps: self.eps * self.var.exp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_x2_add_x() {
        fn x2_add_x(x: Dual) -> Dual {
            x * x + x
        }
        let x = Dual { var: 2.0, eps: 1.0 };
        assert_eq!(x2_add_x(x), Dual { var: 6.0, eps: 5.0 });
    }
    #[test]
    fn test_x3_add_ax() {
        let x = Dual { var: 2.0, eps: 1.0 };
        let a = Dual { var: 2.0, eps: 0.0 };
        let y = x * x * x + a * x;
        assert_eq!(
            y,
            Dual {
                var: 12.0,
                eps: 14.0
            }
        );
    }
    #[test]
    fn test_sin() {
        let x = Dual { var: 0.0, eps: 1.0 };
        assert_eq!(x.sin(), Dual { var: 0.0, eps: 1.0 });
    }
    #[test]
    fn test_sin_exp() {
        fn sin_x_add_xe_x(x: Dual) -> Dual {
            x.sin() + x * x.exp()
        }
        let x = Dual { var: 0.0, eps: 1.0 };
        assert_eq!(sin_x_add_xe_x(x), Dual { var: 0.0, eps: 2.0 });
    }
    #[test]
    fn test_multivar() {
        fn sin_y_add_xy_add_y_e_x(x: Dual, y: Dual) -> Dual {
            y.sin() + x * y + y * x.exp()
        }
        let x = Dual { var: 0.0, eps: 1.0 };
        let y = Dual { var: 2.0, eps: 0.0 };
        let z = sin_y_add_xy_add_y_e_x(x, y);
        assert_approx_eq!(z.var, 2.9092975, 1e-7);
        assert_eq!(z.eps, 4.0);
    }
}
