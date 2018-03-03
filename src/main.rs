#[cfg(test)]
#[macro_use]
extern crate hamcrest;
extern crate game_of_live;

use game_of_live::Status;

fn main() {
    println!("Game of Live");
    println!("============");
    println!();
    println!("{}", Status::new(42, 23, 5, 3));
}


#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

}
