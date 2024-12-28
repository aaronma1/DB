use rand::Rng;

const HASH_MASK: usize = (1<<32)-1;

#[derive(Clone, Debug, Copy)]
pub struct HashFn {
    a: u32,
    hash_bits: u8 
}    

impl HashFn {
    pub fn hash(&self, key: usize) -> usize {
        let hv: usize = (self.a as usize)*key;
        (hv & HASH_MASK) >> (32 - self.hash_bits)
    }
}

pub fn sample(hash_bits: usize) -> HashFn {
    let mut rng = rand::thread_rng();
    let mut a: u32 = rng.gen();
    a = a - (a%2) + 1;  
    HashFn  {
        a: a as u32,
        hash_bits: hash_bits as u8,
    }
}
