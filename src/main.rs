use gnuplot::{Caption, Color, Figure};
extern crate nalgebra as na;

const A: f64 = 0.01;
const B: f64 = 0.05;
const C: f64 = 0.0001;
const X0: f64 = 1000.0;
const Y0: f64 = 100.0;
const DELTA_T: f64 = 0.001;
type NDim = na::U2;

fn main() {
    let mut x = na::RowVectorN::<_, NDim>::new(X0, Y0);
    let mut xs = vec![];
    let mut ys = vec![];
    let mut ts = vec![];

    for i in 1..500000 {
        let t = (i as f64) * DELTA_T;
        let dx = runge_kutta(&x, |x| {
            na::RowVectorN::<_, NDim>::new(
                A * x[0] - C * x[0] * x[1], 
                -B * x[1] + C * x[0] * x[1])
        });
        x += dx;
        ts.push(t);
        xs.push(x[0]);
        ys.push(x[1]);
    }
    plot(&xs, &ys);
}

fn runge_kutta<F>(x: &na::RowVectorN<f64, NDim>, f: F) -> na::RowVectorN<f64, NDim>
where
    F: Fn(&na::RowVectorN<f64, NDim>) -> na::RowVectorN<f64, NDim>,
{
    let k1 = f(x) * DELTA_T;
    let df = |x| f(&x) * DELTA_T;
    let k2 = df(x + &k1 / 2.0);
    let k3 = df(x + &k2 / 2.0);
    let k4 = df(x + &k3);
    (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
}

fn plot(x: &[f64], y: &[f64]) {
    let mut fg = Figure::new();
    fg.set_terminal("pngcairo", "output.png");
    fg.axes2d()
        .lines(x, y, &[Caption("A line"), Color("black")]);
    let _ = fg.show();
}
