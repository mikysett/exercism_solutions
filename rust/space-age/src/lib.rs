// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
#[derive(Debug)]
pub struct Duration(f64);

macro_rules! new_planet {
    ($planet:ident, $orbital_period:literal) => {
        pub struct $planet;

        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                **d / 31557600. / $orbital_period
            }
        }
    };
    ($planet:ident) => {
        pub struct $planet;
        impl Planet for $planet {}
    };
}

impl std::ops::Deref for Duration {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        **d / 31557600.
    }
}

new_planet!(Mercury, 0.2408467);
new_planet!(Venus, 0.61519726);
new_planet!(Earth);
new_planet!(Mars, 1.8808158);
new_planet!(Jupiter, 11.862615);
new_planet!(Saturn, 29.447498);
new_planet!(Uranus, 84.016846);
new_planet!(Neptune, 164.79132);
