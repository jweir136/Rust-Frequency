#![allow(non_snake_case)]

/// A data type used to represent a frquency.
/// You can specify the units of frequency used. The available types are Hz, kHz, MHz, GHz, THz.
/// 
/// Each item must specify a value, as a 64-bit float. Valid values should be positive, and ideally
/// less than 1000 (however not strictly required).
pub enum Frequency {
    Hz(f64),
    #[allow(non_camel_case_types)]
    kHz(f64),
    MHz(f64),
    GHz(f64),
    THz(f64)
}

/// Used to convert a Frequency item to its standardized value in hertz.
/// Takes a Frequency value and returns its values in Hertz as a 64-bit float.
///
/// This function is particularly useful for doing calculations where the frequency needs to be
/// in the standard hertz unit.
pub fn as_hz(freq : Frequency) -> f64 {
    match freq {
        Frequency::Hz(x) =>    {
                        x
                    },
        Frequency::kHz(x) =>   {
                        x * 1000f64
                    },
        Frequency::MHz(x) =>   {
                        x * 1000000f64
                    },
        Frequency::GHz(x) =>   {
                        x * 1e9
                    },
        Frequency::THz(x) =>   {
                        x * 1e12
                    }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Frequency, as_hz};

    #[test]
    fn as_hz_tests() {
        assert!(as_hz(Frequency::kHz(99.3)) == 99300f64);
        assert!(as_hz(Frequency::MHz(96.1)) == 96100000f64);
        assert!(as_hz(Frequency::GHz(100.1)) == 1.001e+11);
        assert!(as_hz(Frequency::THz(100.7)) == 1.007e+14);
    }
}
