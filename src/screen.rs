/// Prints the game header.
pub fn print_header() {
    println!("Game of Life");
    println!("============");
    println!();
}

/// Clears the screen.
pub fn clear() {
    print!("\x1b[2J\x1b[1;1H");
}
