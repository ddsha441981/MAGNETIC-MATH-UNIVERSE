mod mod_file {
    include!("mod.rs");
}

fn main() {
    println!("=== RUNNING EXPERIMENTS ===");


    // mod_file::array_data::main();

    // mod_file::ram_virus_remove::main();
    // mod_file::test3::main();
    //mod_file::maze_solver::main();
    //mod_file::magnetic_cracker::main();
    //mod_file::password_cracker::main();
    mod_file::np_killer::main();
}