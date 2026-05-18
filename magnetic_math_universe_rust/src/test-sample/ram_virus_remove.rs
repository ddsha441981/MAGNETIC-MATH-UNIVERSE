fn magnetic_add(a: i32, b: i32) -> i32 {
    // ZERO EMP RULE (Experiment 3)
    // Agar dono mein se koi bhi 0 hai, toh EMP trigger hoga
    if a == 0 {
        if b % 2 == 0 { return 0; } // 0 Safe Grounding (Even wipes out safely)
        else { return 2; }          // Emergency Reboot (Odd virus resets to stable 2)
    }
    if b == 0 {
        if a % 2 == 0 { return 0; } 
        else { return 2; }
    }

    // The main CORE RULES
    let is_a_even = a % 2 == 0;
    let is_b_even = b % 2 == 0;

    if is_a_even && is_b_even {
        // Rule 1: Attraction (North + North)
        a + b
    } else if !is_a_even && !is_b_even {
        // Rule 2: Repulsion (South + South) -> Polarity flips to negative
        -(a + b)
    } else {
        // Rule 3: The Clash (North + South) -> Badi energy chhoti ko nigal jayegi (Experiment 1)
        (a - b).abs() 
    }
}

//new Data Structure Magnetic Array (Auto-Healing Memory)
struct MagneticRam {
    memory_blocks: Vec<i32>,
}

impl MagneticRam {
    fn new(data: Vec<i32>) -> Self {
        MagneticRam { memory_blocks: data }
    }

    // Antivirus flush method (Experiment 3)
    fn emp_flush_virus(&mut self) {
        println!("ALERT: Sending EMP (0) to all memory blocks...");
        for i in 0..self.memory_blocks.len() {
            // Add 0 (EMP) to every block
            self.memory_blocks[i] = magnetic_add(0, self.memory_blocks[i]);
        }
    }
}

pub(crate) fn main() {
    println!("--- MAGNETIC MATH UNIVERSE INITIALIZED ---\n");

    //********************************************************************
    // Experiment 1: Network Collision (Auto Compression)
    // ********************************************************************
    let packet_a = 7; // Odd (Aggressive)
    let packet_b = 4; // Even (Stable)
    
    println!("📡 EXPERIMENT 1: Network Collision");
    println!("Merging Packet A ({}MB) and Packet B ({}MB)...", packet_a, packet_b);
    let compressed_load = magnetic_add(packet_a, packet_b);
    println!("💥 CLASH RESULT: Server load is automatically compressed to {}MB!\n", compressed_load);
    // Output will be 3MB instead of 11MB!

    // ********************************************************************
    // Experiment 3: The Corrupted RAM (Self-Healing)
    // ********************************************************************
    // Ram has some stable files (8, 4) and a Virus (9)
    let mut system_ram = MagneticRam::new(vec![8, 9, 4]); 
    
    println!("💻 Experiment 3: Self-Healing RAM");
    println!("Original Memory State: {:?}", system_ram.memory_blocks);
    // yanha EMP Trigger hoga
    system_ram.emp_flush_virus();
    println!("HEALED Memory State: {:?}", system_ram.memory_blocks);
    //yanha 8 aur 4 0 ho jayenge(safely flush hoga ) 9 (virus) 2 ho jaega 
}
