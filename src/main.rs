fn indexes_for(np: usize) {
    println!(
        "/// Defines a set of indexes into a struct of type `PerPlayer<T, {}>`.",
        np
    );
    println!("pub mod for{} {{", np);
    println!("    use super::PlayerIdx;");
    for i in 0..np {
        println!(
            "    pub const P{}: PlayerIdx<{}> = PlayerIdx({});",
            i, np, i
        );
    }
    println!("}}\n");
}

pub fn main() {
    for np in 1..17 {
        indexes_for(np);
    }
    // indexes_for(16);
    // indexes_for(40);
    // indexes_for(50);
    // indexes_for(60);
    // indexes_for(64);
    // indexes_for(70);
    // indexes_for(80);
    // indexes_for(90);
    // indexes_for(100);
}
