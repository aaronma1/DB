mod rsqf;
mod hash;
mod memtable;
mod sstable;

use std::num::NonZeroU16;




pub const MEMTABLE_SIZE: usize = 1<<16;

pub type TreePtrNz = NonZeroU16;
pub type TreePtr = u16 ;

use crate::hash::sample;

use crate::memtable::Bst;


fn main() {
    
    memtable::test();
    sstable::test_bisect();
    sstable::test_read_write();
    


}
