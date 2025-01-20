// Include the generated bindings
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
use bindings::{NoiseGate, UnreliableQubit};
use bindings::{NoiseGate_GATE_H, NoiseGate_GATE_X, NoiseGate_GATE_Z};

// Implement a safe Rust wrapper
pub const GATE_X: NoiseGate = NoiseGate_GATE_X;
pub const GATE_H: NoiseGate = NoiseGate_GATE_H;
pub const GATE_Z: NoiseGate = NoiseGate_GATE_Z;

pub struct RustyQubit {
    inner: UnreliableQubit,
}

impl Default for RustyQubit {
    fn default() -> Self {
        Self {
            inner: UnreliableQubit {
                noise_level: 0.0,
                times_measured: 0,
            },
        }
    }
}

impl RustyQubit {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_gate(&mut self, gate: NoiseGate) -> Result<i32, &'static str> {
        self.inner.noise_level += 1.0;

        match gate {
            GATE_X => Ok(1),
            GATE_H => Ok(1),
            GATE_Z => Ok(1),
            _ => Err("Unknown gate"),
        }
    }

    pub fn measure(&mut self) -> i32 {
        self.inner.times_measured += 1;
        let measurement = self.inner.noise_level as i32 % 2;
        self.inner.noise_level = measurement as f64;
        return measurement;
    }
}

// FFI interface
#[no_mangle]
pub extern "C" fn apply_gate(qubit: *mut UnreliableQubit, gate: NoiseGate) -> i32 {
    if qubit.is_null() {
        return 0;
    }

    let qubit = unsafe { &mut *qubit };
    let mut rusty_qubit = RustyQubit { inner: *qubit };

    let result = rusty_qubit.apply_gate(gate);
    *qubit = rusty_qubit.inner;

    match result {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn measure_qubit(qubit: *mut UnreliableQubit) -> i32 {
    if qubit.is_null() {
        return -1;
    }

    let qubit = unsafe { &mut *qubit };
    let mut rusty_qubit = RustyQubit { inner: *qubit };

    let measurement = rusty_qubit.measure();
    *qubit = rusty_qubit.inner;

    return measurement;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_qubit_creation() {
        let qubit = RustyQubit::new();
        assert_eq!(qubit.inner.noise_level, 0.0);
        assert_eq!(qubit.inner.times_measured, 0);
    }

    #[test]
    fn test_gates() {
        let mut qubit = RustyQubit::new();

        // Test X gate
        assert_eq!(qubit.apply_gate(NoiseGate_GATE_X).unwrap(), 1);

        // Test H gate
        assert_eq!(qubit.apply_gate(NoiseGate_GATE_H).unwrap(), 1);

        // Test Z gate
        assert_eq!(qubit.apply_gate(NoiseGate_GATE_Z).unwrap(), 1);

        // Test unknown gate
        assert!(qubit.apply_gate(999).is_err());
    }

    #[test]
    fn test_measurement() {
        let mut qubit = RustyQubit::new();
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            let noise_level = rng.gen_range(0.0..10.0);
            qubit.inner.noise_level = noise_level;

            let result = qubit.measure();
            assert_eq!(result, (noise_level as i32) % 2);
            assert_eq!(qubit.inner.noise_level, result as f64);
        }
    }
}
