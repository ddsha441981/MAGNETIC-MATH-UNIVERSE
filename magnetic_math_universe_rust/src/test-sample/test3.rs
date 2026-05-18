//MAGNETIC loop
fn magnetic_cycle(mut start_energy: i32) {
    println!("\n🌀 starting MAGNETIC loop with energy {}", start_energy);
    let mut cycle_count = 0;

    // Loop tab tak chalega jab tak energy 1 (Ground state) na ho jaye
    while start_energy > 1 {
        cycle_count += 1;
        print!("Cycle {}: Energy {} -> ", cycle_count, start_energy);

        if start_energy % 2 == 0 {
            // Rule 4.1: The Smooth Glide (Even)
            start_energy = start_energy / 2;
            println!("[Glide] Safely split to {}", start_energy);
        } else {
            // Rule 4.2: The Turbulence (Odd)
            start_energy = (start_energy * 3) + 1;
            println!("[Turbulence] Resisted and exploded to {}", start_energy);
        }
    }
    
    println!("Yanha loop terminate safely at energy 1 after {} cycles.", cycle_count);
}

pub fn main() {
    println!("**************************************MAGNETIC loop**************************************");

    // Experiment A: Stable Data (Even)
    magnetic_cycle(8);

    // Experiment B: Unstable Data (Odd)
    magnetic_cycle(7);
}
