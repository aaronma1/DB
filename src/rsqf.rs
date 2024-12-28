
use crate::hash::{HashFn, sample};


const KEY_BITS = 32; 

const CANNONICAL_BITS:usize = 14;
const FINGERPRINT_BITS: usize = 8;

#[derive(Copy, Debug, Clone)]
pub struct RsqfBlock {
    offset: u8,
    occupied: u64, 
    runend: u64,
    slots: [u8; 64],
}

impl RsqfBlock {
    fn new() -> RsqfBlock{
        RsqfBlock {
            offset: 0, 
            occupied: 0, 
            runend: 0, 
            slots: [0; 64],
        }
    }

    const fn occupied(&self, i: usize) -> bool {
        self.occupied & (1 << i) != 0
    }
    const fn runend(&self, i: usize) -> bool {
        self.runend & (1 << i) != 0
    }

    const fn set_occupied(&mut self, i: usize) {
        self.occupied = self.occupied | (1 << i);
    }
    const fn set_runend(&mut self, i: usize) {
        self.runend = self.runend | (1 << i);
    }

}


const NUM_BLOCKS: usize = 1<<(CANNONICAL_BITS-6);

pub struct Rsqf {
    hash_fn: HashFn,
    blocks: [RsqfBlock; NUM_BLOCKS],  
}


impl Rsqf {
    fn new() -> Rsqf {
        Rsqf {
            hash_fn: sample(CANNONICAL_BITS + FINGERPRINT_BITS), 
            blocks: [RsqfBlock::new(); NUM_BLOCKS], 
        }
    }

    fn locate_run(&self, c_idx: usize) -> (usize, usize){
        let block_idx  = c_idx & ((1<<6) -1);
        let idx = c_idx >> 6; 



    }

    fn insert(&mut self, key: usize) {
        let hash = self.hash_fn.hash(key);
        let c_idx = hash & ((1 << CANNONICAL_BITS )- 1);
        let fingerprint = hash >> CANNONICAL_BITS;

        let (n_blk, blk_idx) = self.locate_run(c_idx);

    

    }


}

