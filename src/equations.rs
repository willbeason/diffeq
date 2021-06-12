pub struct O1 {
    pub f: fn(p: (f64, f64)) -> f64
}

impl O1 {
    pub fn euler(self, p: (f64, f64), h: f64) -> f64 {
        let yp = (self.f)(p);
        p.1 + yp*h
    }

    pub fn rk4(self, p: (f64, f64), h: f64) -> f64 {
        let t1 = p.0 + h/2.0;

        let k1 = (self.f)(p);
        let y1 = p.1 + k1*(h/2.0);

        let k2 = (self.f)((t1, y1));
        let y2 = p.1 + k2*(h/2.0);

        let k3 = (self.f)((t1, y2));
        let y3 = p.1 + k3*h;

        let k4 = (self.f)((p.0+h, y3));

        p.0+(h/6.0)*(k1+2.0*k2+2.0*k3+k4)
    }
}

pub struct O3 {
    pub f: fn(p: (f64, f64, f64, f64)) -> f64
}

impl O3 {
    pub fn euler(self, p: (f64, f64, f64, f64), h: f64) -> (f64, f64, f64) {
        let yppp = (self.f)(p);

        let ypp = p.3 + yppp*h;

        let yp = p.2 + (p.3 + ypp)*(h/2.0);

        let y = p.1 + (p.2*5.0 + yp)*(h/6.0);
        (y, yp, ypp)
    }
}
