mod computer;

use computer::Computer;

fn main() {
    //let computer = Computer::new();
    println!("Hello, world!");
    let a = 0b10010100 as u8;
    let b = 0b10010100 as u8;
    println!("{:b} {:b}", (a >> 1) as i8, (b as i8) >> 1);
}
