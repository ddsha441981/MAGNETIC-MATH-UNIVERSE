#[derive(Debug, Clone, Copy, PartialEq)]
enum MagneticEnergy {
    Even(i64), // North Pole (Stable)
    Odd(i64),  // South Pole (Aggressive)
    Zero,      // The EMP / Void
}

impl MagneticEnergy {
    // Standard number ko Magnetic Dimension mein convert karna
    fn new(val: i64) -> Self {
        if val == 0 {
            MagneticEnergy::Zero
        } else if val % 2 == 0 {
            MagneticEnergy::Even(val)
        } else {
            MagneticEnergy::Odd(val)
        }
    }

    // 1. MAGNETIC ADDITION (The Merge)
    fn add(self, other: MagneticEnergy) -> MagneticEnergy {
        match (self, other) {
            // Rule 1.1: Attraction (Stable merge)
            (MagneticEnergy::Even(a), MagneticEnergy::Even(b)) => MagneticEnergy::new(a + b),

            // Rule 1.2: Repulsion (Aggressive blast to negative)
            (MagneticEnergy::Odd(a), MagneticEnergy::Odd(b)) => MagneticEnergy::new(-(a.abs() + b.abs())),

            // Rule 1.3: The Clash (Absolute difference)
            (MagneticEnergy::Even(a), MagneticEnergy::Odd(b)) |
            (MagneticEnergy::Odd(a), MagneticEnergy::Even(b)) => MagneticEnergy::new((a.abs() - b.abs()).abs()),

            // Rule 0.1: Safe Grounding
            (MagneticEnergy::Zero, MagneticEnergy::Even(_)) |
            (MagneticEnergy::Even(_), MagneticEnergy::Zero) => MagneticEnergy::Zero,

            // Rule 0.2: Emergency Reboot
            (MagneticEnergy::Zero, MagneticEnergy::Odd(_)) |
            (MagneticEnergy::Odd(_), MagneticEnergy::Zero) => MagneticEnergy::new(2),

            (MagneticEnergy::Zero, MagneticEnergy::Zero) => MagneticEnergy::Zero,
        }
    }

    // 2. MAGNETIC MULTIPLICATION (The Amplifier)
    fn mul(self, other: MagneticEnergy) -> MagneticEnergy {
        match (self, other) {
            // Rule 2.1: Stable Resonance
            (MagneticEnergy::Even(a), MagneticEnergy::Even(b)) => MagneticEnergy::new(a * b),

            // Rule 2.2: Overload
            (MagneticEnergy::Odd(a), MagneticEnergy::Odd(b)) => MagneticEnergy::new(-(a.abs() * b.abs())),

            // Rule 2.3: The EMP / Void (Clash in amplification causes crash)
            (MagneticEnergy::Even(_), MagneticEnergy::Odd(_)) |
            (MagneticEnergy::Odd(_), MagneticEnergy::Even(_)) => MagneticEnergy::Zero,

            // Rule 0.0: The Blackout
            (MagneticEnergy::Zero, _) | (_, MagneticEnergy::Zero) => MagneticEnergy::Zero,
        }
    }
}

pub(crate) fn main() {
    println!("--- MAGNETIC MATH UNIVERSE SIMULATION ---\n");

    // TEST 1: Addition Rules
    let even1 = MagneticEnergy::new(4);
    let even2 = MagneticEnergy::new(2);
    let odd1 = MagneticEnergy::new(3);
    let odd2 = MagneticEnergy::new(5);

    println!("Rule 1.1 (Attraction): 4 + 2 = {:?}", even1.add(even2)); // Expected: Even(6)
    println!("Rule 1.2 (Repulsion): 3 + 5 = {:?}", odd1.add(odd2));  // Expected: Even(-8)
    println!("Rule 1.3 (The Clash): 4 + 3 = {:?}", even1.add(odd1));  // Expected: Odd(1)

    println!("\n--- THE NP-HARD KILLER TEST ---");
    // TEST 2: Evaluating an invalid path: 4 * 3 * 8 * 5 * 2
    let path = vec![4, 3, 8, 5, 2];

    let mut current_state = MagneticEnergy::new(path[0]);
    println!("Starting Energy: {:?}", current_state);

    for i in 1..path.len() {
        let next_node = MagneticEnergy::new(path[i]);
        current_state = current_state.mul(next_node);

        println!("Step {}: Multiplied by {}, Current State -> {:?}", i, path[i], current_state);

        // Hardware Level Break: Stop calculating if EMP is triggered
        if current_state == MagneticEnergy::Zero {
            println!(">> THE EMP TRIGGERED! Path destroyed at step {}. Memory freed instantly.", i);
            break;
        }
    }

    println!("\nFINAL RESULT: {:?}", current_state);
}