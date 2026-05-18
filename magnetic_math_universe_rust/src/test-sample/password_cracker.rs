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

    // Rule 2: Magnetic Multiplication (The Amplifier)
    fn mul(self, other: MagneticEnergy) -> MagneticEnergy {
        match (self, other) {
            // Rule 2.1: Stable Resonance
            (MagneticEnergy::Even(a), MagneticEnergy::Even(b)) => MagneticEnergy::new(a * b),
            // Rule 2.2: Overload (Negative blast)
            (MagneticEnergy::Odd(a), MagneticEnergy::Odd(b)) => MagneticEnergy::new(-(a.abs() * b.abs())),
            // Rule 2.3: The EMP / Void (Hardware crash!)
            (MagneticEnergy::Even(_), MagneticEnergy::Odd(_)) |
            (MagneticEnergy::Odd(_), MagneticEnergy::Even(_)) => MagneticEnergy::Zero,
            // Rule 0.1: The Blackout
            (MagneticEnergy::Zero, _) | (_, MagneticEnergy::Zero) => MagneticEnergy::Zero,
        }
    }
}

pub(crate) fn main() {
    println!("--- MAGNETIC P=NP PASSWORD CRACKER ---\n");

    // The Target Lock (Password): Built with Even (Stable) Energies
    let lock_nodes = vec![
        MagneticEnergy::new(4),
        MagneticEnergy::new(2),
        MagneticEnergy::new(6)
    ];

    // Hacker generating millions of guesses. Let's test 3 of them.
    let guesses = vec![
        vec![4, 3, 6], // Wrong (Has an Odd number)
        vec![5, 1, 3], // Wrong (All Odd)
        vec![4, 2, 6], // Correct Password!
    ];

    for (attempt_num, guess) in guesses.iter().enumerate() {
        println!("Testing Guess {}: {:?}", attempt_num + 1, guess);

        let mut path_energy = MagneticEnergy::new(1); // Starting flow
        let mut steps_taken = 0;

        // Feed the guess into the Magnetic Lock
        for i in 0..guess.len() {
            steps_taken += 1;
            let guess_node = MagneticEnergy::new(guess[i]);

            // Multiply Lock node with Guess node
            path_energy = path_energy.mul(guess_node).mul(lock_nodes[i]);

            // THE HARDWARE CIRCUIT BREAKER
            if path_energy == MagneticEnergy::Zero {
                println!("  [!] EMP TRIGGERED at digit {}. Wrong path burned instantly!", i + 1);
                break; // System stops calculating this path completely.
            }
        }

        if path_energy != MagneticEnergy::Zero {
            println!("  [SUCCESS] The Golden Path Found! Lock is OPEN. Energy: {:?}", path_energy);
            println!("  Correct Password is: {:?}", guess);
        }

        println!("  Time/Steps spent on this guess: {}\n", steps_taken);
    }
}