use super::MEMTABLE_SIZE;
use crate::memtable::Bst;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::vec::Vec;
use std::io::{Seek, Write};
use std::io;


const DB_PATH: &str = "/home/aaron/Sync/Database/db";
const FS_BLOCK_SIZE: usize = 512;
const BLOCK_ENTRIES: usize = 512/std::mem::size_of::<(usize, usize)>();


// return the last index i sutch that lst[i] <= key
fn bisect(lst: &[(usize, usize)], key: usize) -> usize {
    let mut hi = lst.len() - 1;
    let mut lo = 0;

    if lst[lo].0 > key {
        0
    } else if lst[hi].0 < key {
        hi
    } else {
        loop {
            let m = (lo + hi) >> 1;

            if lst[m].1 <= key {
                lo = m;
            } else if lst[m].1 > key {
                hi = m;
            }
            if hi - lo < 2 {
                return lo;
            }
        }
    }
}

pub fn test_bisect() {
    let lst = [
        (1, 1),
        (1, 1),
        (1, 1),
        (3, 3),
        (3, 3),
        (3, 3),
        (3, 3),
        (5, 5),
        (10, 10),
    ];
    println!("{} {}", bisect(&lst, 3), bisect(&lst, 3));
    println!("{} {}", bisect(&lst, 1), bisect(&lst, 1));
}

struct MemSST {
    rank: usize,
    kv: Vec<(usize, usize)>,
}
impl MemSST {
    fn write(&self) -> Option<DiskSST> {
        let rank: usize = self.rank;
        let size: usize = self.kv.len();
        let fname = format!("sst_{}_{}", self.rank, self.kv.len());
        let path: PathBuf = Path::new(&DB_PATH).join(&fname);

        if let Ok(mut f) = File::create(&path) {
            f.write_all(
                unsafe {std::mem::transmute(self.kv.as_slice())}
            );
            drop(f);
            

            Some( DiskSST {
                rank,
                size,
                fp: File::open(&path).unwrap(),
            })

        } else {
            None
        }
    }

    fn scan(&self, lo: usize, hi: usize) -> Vec<(usize, usize)> {
        let lb = bisect(&self.kv, lo);
        let ub = bisect(&self.kv, hi).saturating_add(1);

        let mut rtn: Vec<(usize, usize)> = Vec::with_capacity(hi.saturating_sub(lo));

        if self.kv[lo].0 == lo {
            rtn.push(self.kv[lo]);
        }
        for i in (lb + 1)..ub {
            rtn.push(self.kv[i])
        }
        rtn
    }
}

impl From<Bst> for MemSST {
    fn from(other: Bst) -> Self {
        MemSST {
            rank: 1,
            kv: other.flush(),
        }
    }
}

struct DiskSST {
    rank: usize,
    size: usize,
    fp: File,
}

impl DiskSST {
    // pub fn open(name: &tr) -> std::io::Result<DiskSST> {
    //     let tokens = name.split("_");
    //     let _sst = tokens.next();
    //     let rank = tokens.next();
    //     let size = tokens.next();
    //
    // }

    fn delete() {


    }
    pub fn blocks(&mut self) -> usize {
        self.size / FS_BLOCK_SIZE
    }

    pub fn read_block(&self, block_no: usize) -> io::Result<[(usize, usize); BLOCK_ENTRIES]>{
        self.fp.seek(io::SeekFrom::Start(block_no *FS_BLOCK_SIZE));
        
        let mut sector [u8; BLOCK_];


        



    }

    pub fn scan() -> Vec<(usize, usize)> {
        


    }

    pub fn merge() {

    }
}

pub fn test_read_write() {

    let kv: Vec<(usize, usize)> = vec![(10 as usize, 10 as usize), (12 as usize, 12 as usize), (24 as usize,24 as usize)];
    let m = MemSST {
        rank: 0,
        kv,
    };
    let d: DiskSST = m.write().expect("failure");
    


}




enum SST {
    Disk(DiskSST),
    Mem(MemSST),
}



