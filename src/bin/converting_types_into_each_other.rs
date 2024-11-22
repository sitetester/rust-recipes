#[derive(Debug)]
struct Meter (f32);

#[derive(Debug)]
struct KiloMeter (f32);

trait Convertible<T> {
    fn convert(&self) -> T;
}

impl Convertible<KiloMeter> for Meter {
    fn convert(&self) -> KiloMeter {
        KiloMeter(self.0 / 1000 as f32)
    }
}

impl Convertible<Meter> for KiloMeter {
    fn convert(&self) -> Meter {
        Meter(self.0 * 1000 as f32)
    }
}

fn main() {
    let m = Meter(1500.0);
    let km = m.convert();
    assert!(km.0 == 1.5);

    let m = km.convert();
    assert!(m.0 == 1.5);

}