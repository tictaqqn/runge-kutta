use gnuplot::{Caption, Color, Figure};
extern crate nalgebra as na;
use runge_kutta::auto_diff::Dual;
use runge_kutta::nabla::Func3D;

// fox and rabbit
// const A: f64 = 0.01;
// const B: f64 = 0.05;
// const C: f64 = 0.0001;
// const X0: f64 = 1000.0;
// const Y0: f64 = 100.0;
const DELTA_T: f64 = 0.001;
const MAX_ITER: usize = 500000;
// type NDim = na::U2;
// type VN = na::VectorN<f64, NDim>;

// fn time_evol(x: &VN) -> VN {
//     VN::new(A * x[0] - C * x[0] * x[1], -B * x[1] + C * x[0] * x[1])
// }

// birds swarms
const GAMMA: f64 = 0.01;
const VV0: f64 = 1.0;
const A1: f64 = 0.001;
const A2: f64 = 0.0008;
const AV: f64 = 0.001;
const RR1: f64 = 1.0;
const RR2: f64 = 2.0;
const RRV: f64 = 1.0;
type NDim = na::U40;
type NDimX = na::U20;
type NDimBird = na::U10;
const N_DIM: usize = 40;
const N_DIMX: usize = N_DIM / 2;
const N_BIRD: usize = N_DIMX / 2;
type VN = na::VectorN<f64, NDim>;
type V2 = na::Vector2<f64>;

fn dist_potential(x: &na::Vector2<Dual>) -> Dual {
    let z = x[0] * x[0] + x[1] * x[1];
    A1 * (z / -RR1).exp() - A2 * (z / -RR2).exp()
}
fn dist_reg(x: &V2) -> V2 {
    let f = Func3D::new(dist_potential);
    -f.nabla(x)
}
fn align_velocity(x: &V2, v: &V2) -> V2 {
    v * AV * (x.norm_squared() / RRV)
}
fn time_evol(xv: &VN) -> VN {
    let mut dx = VN::zeros();
    dx.fixed_rows_mut::<NDimX>(0)
        .copy_from(&xv.fixed_rows::<NDimX>(N_DIMX));
    let mut tmp_dist = na::MatrixMN::<na::Vector2<f64>, NDimBird, NDimBird>::zeros();
    let mut tmp_align = na::MatrixMN::<na::Vector2<f64>, NDimBird, NDimBird>::zeros();
    for i in 0..N_BIRD {
        let x = xv.fixed_rows::<na::U2>(2 * i);
        let v = xv.fixed_rows::<na::U2>(N_DIMX + 2 * i);
        let mut f = GAMMA * (VV0 - v.norm_squared()) * v;
        for j in 0..N_BIRD {
            if i >= j {
                f += tmp_dist[(j, i)] - tmp_align[(j, i)];
                continue;
            }
            let xx = xv.fixed_rows::<na::U2>(2 * j) - x;
            let vv = xv.fixed_rows::<na::U2>(N_DIMX + 2 * j) - v;
            tmp_dist[(i, j)] = dist_reg(&xx);
            tmp_align[(i, j)] = align_velocity(&xx, &vv);
            f += tmp_dist[(i, j)] + tmp_align[(i, j)];
        }
        dx.fixed_rows_mut::<na::U2>(N_DIMX).copy_from(&f);
    }
    dx
}

fn main() {
    let mut x = VN::new_random(); // new(X0, Y0);
    let mut xs = vec![];
    let mut ys = vec![];
    let mut ts = vec![];

    for i in 0..MAX_ITER {
        let t = (i as f64) * DELTA_T;
        let dx = runge_kutta(&x, time_evol);
        x += dx;
        if i % 10 == 0 {
            println!("{}", &x);
        }
        ts.push(t);
        xs.push(x[0]);
        ys.push(x[1]);
    }
    plot(&xs, &ys);
}

fn runge_kutta<F>(x: &VN, f: F) -> VN
where
    F: Fn(&VN) -> VN,
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
