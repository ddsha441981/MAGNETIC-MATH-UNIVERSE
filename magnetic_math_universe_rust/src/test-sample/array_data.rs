struct MagneticRam {
    memory_blocks: Vec<i32>,
}

impl MagneticRam {
    fn new() -> Self {
        MagneticRam { memory_blocks: Vec::new() }
    }

    // wild push fuc
    fn magnetic_push(&mut self, new_data: i32) {
        println!("\n📥 Inserting: {}", new_data);
        
        // Agar memory khali hai, toh chup chap daal do
        if self.memory_blocks.is_empty() {
            self.memory_blocks.push(new_data);
            println!("   -> Memory empty. Placed {} at the start.", new_data);
            return;
        }

        // Array ka aakhri number nikalo
        let last_idx = self.memory_blocks.len() - 1;
        let tail_data = self.memory_blocks[last_idx];

        let is_new_even = new_data % 2 == 0;
        let is_tail_even = tail_data % 2 == 0;

        // MAGNETIC rules apply hota hai
        if is_new_even && is_tail_even {
            println!("Attraction North ({}) + North ({}). Sticking together!", tail_data, new_data);
            self.memory_blocks.push(new_data);
        } 
        else if !is_new_even && !is_tail_even {
            println!("Repulsion South ({}) + South ({}). Repelling to the front!", tail_data, new_data);
            self.memory_blocks.insert(0, new_data); // forcefully jhatka maar ke index 0 pe fek diya!
        } 
        else {
            println!("Clash: ({}) and ({}) are fighting for space!", tail_data, new_data);
            let survivor = (tail_data - new_data).abs();
            self.memory_blocks.pop(); // Puraane tail ko burn diya
            self.memory_blocks.push(survivor); // Naye survivor ko rakh diya
            println!("Survior{} takes the spot.", survivor);
        }
        
        println!("Current Memeory state {:?}", self.memory_blocks);
    }
}

pub(crate) fn main() {
    println!("--------------------MAGNETIC RAM BOOT SEQUENCE-----------------------------------------");
    let mut ram = MagneticRam::new();

    // Data insert start
    ram.magnetic_push(4); // Even
    ram.magnetic_push(8); // Even (Attraction hogi)
    ram.magnetic_push(5); // Odd  (Clash hoga 8 ke sath)
    ram.magnetic_push(3); // Odd  (Repulsion hogi 5 (ab 3 ban chuka survivor) ke sath)
}
