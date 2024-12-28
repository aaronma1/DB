mod rsqf;
mod hash;
mod memtable;
use crate::hash::sample;


fn main() {
    println!("Hello, world!");


    let h = sample(6);
    println!("{:?}", h);

    let mut table: [usize; 64] = [0;64];
    for i in 1..10000 {
        println!("{}", h.hash(i));

        table[(h.hash(i)) as usize] += 1;
    } 
    for i in 0..64 {
        println!("{}", table[i]);

    }



}
