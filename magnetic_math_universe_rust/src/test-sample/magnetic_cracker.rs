#[derive(Debug, Clone, Copy, PartialEq)]
enum MagneticEnergy {
    Even(i64),
    Odd(i64),
    Zero,
}

impl MagneticEnergy {
    fn new(val: i64) -> Self {
        if val == 0 { MagneticEnergy::Zero }
        else if val % 2 == 0 { MagneticEnergy::Even(val) }
        else { MagneticEnergy::Odd(val) }
    }

    // MULTIPLICATION: Magnetic Amplifier
    fn mul(self, other: MagneticEnergy) -> MagneticEnergy {
        match (self, other) {
            // Stable Resonance
            (MagneticEnergy::Even(a), MagneticEnergy::Even(b)) => MagneticEnergy::new(a * b),
            // Overload
            (MagneticEnergy::Odd(a), MagneticEnergy::Odd(b)) => MagneticEnergy::new(-(a.abs() * b.abs())),
            // The EMP / Void
            (MagneticEnergy::Even(_), MagneticEnergy::Odd(_)) |
            (MagneticEnergy::Odd(_), MagneticEnergy::Even(_)) => MagneticEnergy::Zero,
            // The Blackout
            (MagneticEnergy::Zero, _) | (_, MagneticEnergy::Zero) => MagneticEnergy::Zero,
        }
    }

    // EXPONENTS: Magnetic Resonance
    fn power(self, p: i64) -> MagneticEnergy {
        if p == 0 { return MagneticEnergy::Zero; } // The Power EMP
        let standard_result = match self {
            MagneticEnergy::Even(val) => val.pow(p as u32),
            MagneticEnergy::Odd(val) => val.pow(p as u32),
            MagneticEnergy::Zero => 0,
        };

        let pow_energy = MagneticEnergy::new(p);
        match (self, pow_energy) {
            (MagneticEnergy::Even(_), MagneticEnergy::Even(_)) => MagneticEnergy::new(standard_result),
            (MagneticEnergy::Odd(_), MagneticEnergy::Odd(_)) => MagneticEnergy::new(-standard_result.abs()),
            (MagneticEnergy::Even(_), MagneticEnergy::Odd(pow_val)) => MagneticEnergy::new(standard_result + pow_val), // Power Corrupts
            (MagneticEnergy::Odd(_), MagneticEnergy::Even(pow_val)) => MagneticEnergy::new(standard_result - pow_val), // Power Calms
            _ => MagneticEnergy::Zero,
        }
    }
}

pub fn main() {
    println!("--- QUANTUM CORE LOCK: MAGNETIC BRUTE-FORCE ---\n");

    let correct_lock = vec![4, 2, 6, 8, 2];

    // Test Case 1: The Hacker's Trap (Contains an Odd Power mutation)
    // Concept: 4 * (2^3) * 6 * 8 * 2
    let mut trap_energy = MagneticEnergy::new(4);
    let mutated_node = MagneticEnergy::new(2).power(3); // 2^3 -> 11 (Odd)
    println!("Evaluating Attack Path...");

    trap_energy = trap_energy.mul(mutated_node);
    if trap_energy == MagneticEnergy::Zero {
        println!("[!] EMP TRIGGERED AT STEP 2! Even(4) clashed with mutated Odd(11).");
        println!("[!] Blackout initiated. Remaining steps (6, 8, 2) aborted. Memory freed.\n");
    }

    // Test Case 2: The Golden Path (Correct Password)
    // Concept: 4 * 2 * 6 * 8 * 2
    let mut golden_energy = MagneticEnergy::new(1);
    for node in correct_lock.iter() {
        golden_energy = golden_energy.mul(MagneticEnergy::new(*node));
    }
    println!("[SUCCESS] Golden Path survived! Lock Energy: {:?}", golden_energy);
}