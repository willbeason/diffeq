mod equations;

fn f(p: (f64, f64)) -> f64 {
    3.0 * p.1 * p.1 * (p.1 + p.0).sin()
}

fn main() {
    let eq = equations::O1 {
        f
    };


}
