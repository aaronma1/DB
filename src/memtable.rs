// use std::num::NonZeroU16;
use std::{mem::transmute, num::NonZeroU16};
use std::vec::Vec;
use crate::hash::{HashFn, sample};



const MEMTABLE_SIZE: usize = 1<<16;

type TreePtrNz = NonZeroU16;
type TreePtr = u16 ;

const fn to_usize(x: TreePtrNz) -> usize{
    unsafe {
        transmute::<TreePtrNz, TreePtr>(x) as usize
    }
}

const fn to_usize_o(x: Option<TreePtrNz>) -> usize{
    unsafe {
        transmute::<Option<TreePtrNz>, TreePtr>(x) as usize
    }
}

const fn max(x: u8, y:u8) -> u8{
    if x > y {
        return x;
    }
    return y;
}
#[derive(Copy, Clone, Debug)]
struct BstNode {
    key: usize,
    // lc and rc are 0
    lc: Option<TreePtrNz>, 
    rc: Option<TreePtrNz>,
}

impl BstNode {
    
    const fn lc(&self) -> Option<usize> {
        match self.lc {
            Option::None => Option::None,
            Option::Some(x) => unsafe { Option::Some((transmute::<TreePtrNz, TreePtr>(x)) as usize) },
        }
    }
    const fn rc(&self) -> Option<usize> {
        match self.rc {
            Option::None => Option::None,
            Option::Some(x) => unsafe { Option::Some((transmute::<TreePtrNz, TreePtr>(x)) as usize) },
        }
    }

    const fn set_rc(&mut self, rc: TreePtr) {
        // safe because if rc is root (0), we set rc=None
        self.rc =  unsafe { transmute::<TreePtr, Option<TreePtrNz>>(rc) }
    }
    const fn set_lc(&mut self, lc: TreePtr) {
        // safe because if lc is root (0), we set lc=None
        self.lc =  unsafe { transmute::<TreePtr, Option<TreePtrNz>>(lc) }
    }

}


impl BstNode {
    const fn new(key: usize) -> BstNode {
        BstNode {
            key,
            lc: Option::None,
            rc: Option::None,
        }
    }
}

const ROOT: usize = 0;
pub struct Bst {
    nextfree: usize,
    // 0 is a NULL node
    nodes: [BstNode; MEMTABLE_SIZE],
    vals: [usize; MEMTABLE_SIZE],
    depths: [u8; MEMTABLE_SIZE],
    parent: [TreePtr; MEMTABLE_SIZE],
}



impl Bst {
    pub fn new() -> Bst {
        Bst {
            nextfree: 0, 
            nodes: [BstNode::new(0); MEMTABLE_SIZE],
            vals: [0; MEMTABLE_SIZE],
            depths: [0; MEMTABLE_SIZE],
            parent: [0; MEMTABLE_SIZE],
        }
    }

    const fn swp(&mut self, a: usize, b:usize) {
        let tmp = self.nodes[a];
        self.nodes[a] = self.nodes[b];
        self.nodes[b] = tmp;
        let tmp = self.vals[a];
        self.vals[a] = self.vals[b];
        self.vals[b] = tmp;
    }


    // search bst for key 
    // returns (parent idx, idx to insert)
    const fn search(&self, key: usize) -> usize {
        if self.nextfree == 0 {
            return ROOT;
        }

        let mut cur = ROOT;
        let mut cur_key = self.nodes[cur].key;
        let mut next = Option::None;

        loop {
            if key == cur_key{
                return cur;
            } else if key < cur_key {
                next = self.nodes[cur].lc();
            } else if key > cur_key {
                next = self.nodes[cur].rc();
            }


            if let Option::Some(next_idx) = next{
                cur = next_idx;
                cur_key = self.nodes[next_idx].key;
            } else {
                return cur;
            }
        }
    }

    const fn depth(&self, x: Option<TreePtrNz>) -> u8{
        match x {
            Option::None => 0,
            Option::Some(x) => self.depths[to_usize(x)],
        }
    }

    const fn update_depth_1(&mut self, idx: usize) {
            let rc_depth = match self.nodes[idx].rc() {
                Option::None =>  0,
                Option::Some(x) => self.depths[x],
            };
            let lc_depth = match self.nodes[idx].lc() {
                Option::None =>  0,
                Option::Some(x) => self.depths[x],
            };
            self.depths[idx] = max(lc_depth, rc_depth) + 1;
    }
    
    const fn rr(&mut self, idx_a: usize) {
        if let Option::Some(idx_b) = self.nodes[idx_a].lc() {
            let idx_h = self.nodes[idx_a].rc;
            let idx_m = self.nodes[idx_b].rc;
            let idx_l = self.nodes[idx_b].lc;

            self.swp(idx_b, idx_a);
            self.nodes[idx_a].set_rc(idx_b as TreePtr);
            self.nodes[idx_a].lc = idx_l;
            self.nodes[idx_b].lc = idx_m;
            self.nodes[idx_b].rc = idx_h;
            
            //update parent
            self.parent[idx_b] = idx_a as TreePtr;
            self.parent[to_usize_o(idx_l)] = idx_a as TreePtr;
            self.parent[to_usize_o(idx_m)] = idx_b as TreePtr; 
            self.parent[to_usize_o(idx_h)] = idx_b as TreePtr; 


            self.parent[0] = 0;
            //update depths
            self.update_depth_1(idx_b);
            self.update_depth_1(idx_a);
        }
    }
    const fn rl(&mut self, idx_a: usize) {
        // println!("idx_a {}", idx_a);
        if let Option::Some(idx_b) = self.nodes[idx_a].rc() {
            let idx_h = self.nodes[idx_b].rc;
            let idx_m = self.nodes[idx_b].lc;
            let idx_l = self.nodes[idx_a].lc;

            self.swp(idx_b, idx_a);
            self.nodes[idx_a].set_lc(idx_b as TreePtr);
            self.nodes[idx_a].rc = idx_h;

            self.nodes[idx_b].lc = idx_l;
            self.nodes[idx_b].rc = idx_m;

            //update_parent
            self.parent[to_usize_o(idx_l)] = idx_b as TreePtr;
            self.parent[to_usize_o(idx_m)] = idx_b as TreePtr; 
            self.parent[to_usize_o(idx_h)] = idx_a as TreePtr; 
            self.parent[idx_b] = idx_a as TreePtr;

            self.parent[0] = 0;

            //update depths
            self.update_depth_1(idx_b);
            self.update_depth_1(idx_a);
        }
    }

    

    pub fn put(&mut self, key: usize, val: usize) {
        if self.nextfree == 0 {
            self.nextfree = 1;
            self.nodes[ROOT] = BstNode::new(key);
            self.depths[ROOT] =1;
            return;
        }
        let idx = self.search(key);
        // println!("{}", idx, self.nodes[idx].key);
        if self.nodes[idx].key == key {
            self.vals[idx] = val;
            return;
        }

        if key < self.nodes[idx].key {
            // assert!(self.nodes[idx].lc().is_none());
            self.nodes[idx].set_lc(self.nextfree as TreePtr);
            self.nodes[self.nextfree] = BstNode::new(key);
        } else {
            // assert!(self.nodes[idx].rc().is_none());
            self.nodes[idx].set_rc(self.nextfree as TreePtr);
            self.nodes[self.nextfree] = BstNode::new(key);
        }
        let mut cur: usize = self.nextfree;
        //initialize current node
        self.depths[cur] = 1;
        self.parent[cur] = idx as TreePtr;
        self.vals[cur] = val;
        //update depths and rebalance
        let mut parent: usize = idx;
        self.nextfree += 1;
        

        //iterate upwards 
        loop {
            // println!("{} {} {:?} {:?}", cur, parent, self.nodes[cur], self.nodes[parent]);

            let rc_depth = match self.nodes[parent].rc() {
                Option::None =>  0,
                Option::Some(x) => self.depths[x],
            };
            let lc_depth = match self.nodes[parent].lc() {
                Option::None =>  0,
                Option::Some(x) => self.depths[x],
            };
            self.depths[parent] = max(lc_depth, rc_depth) + 1;
            let balance: i16 = i16::from(lc_depth) - i16::from(rc_depth);

            let plc = self.nodes[parent].lc();
            let prc = self.nodes[parent].rc();
            
            if plc.is_some() && key < self.nodes[plc.unwrap()].key && balance > 1 {
                self.rr(parent);
            }
            if prc.is_some() && key > self.nodes[prc.unwrap()].key && balance < -1{
                self.rl(parent);
            }

            if plc.is_some() && key > self.nodes[plc.unwrap()].key && balance > 1{
                self.rl(cur);
                self.rr(parent);
            }
            if prc.is_some() && key < self.nodes[prc.unwrap()].key && balance < -1{
                self.rr(cur);
                self.rl(parent);
            }
        
            cur = parent;
            parent = self.parent[parent] as usize;
            if cur == parent {
                break;
            }
        }

        //get ready for next 
        // rebalance
    }
    

    pub fn get(&self, key: usize) -> Option<usize> {
        if self.nextfree == 0 {
           Option::None
        } else {
            let idx = self.search(key);
            if self.nodes[idx].key == key {
                Option::Some(self.vals[idx])
            } else {
                Option::None
            }
        }
    }


    

    //returns all key value pairs between lo and hi in o(hi-lo)
    pub fn scan(&self, lo:usize, hi:usize) -> Vec<(usize, usize)> {
        //leftmost leaf
        let mut cur = self.search(lo);
        
        if let Some(rc) =  self.nodes[cur].rc() {


        }


        //find lo
        
        // find hi

        // inorder traversal from lo to hi
        return Vec::new();


    }


    //check balance factor, depth parent
    pub fn print(&self) {
        for i in 0..self.nextfree {
            println!("{:?}, depth: {} parent: {}, val: {}",self.nodes[i], self.depths[i], self.parent[i], self.vals[i]);
        }
    }

    pub fn validate(&self) {
        let mut links = 0;
        for i in 0..self.nextfree {
            println!("{:?}, depth: {} parent: {}, val: {}",self.nodes[i], self.depths[i], self.parent[i], self.vals[i]);
        }
        for i in 0..self.nextfree {
            
            if self.nodes[i].rc().is_some() {
                links += 1;
            }
            if self.nodes[i].lc().is_some() {
                links += 1;
            }
        

            let lc_depth = self.depth(self.nodes[i].lc);
            let rc_depth = self.depth(self.nodes[i].rc);
            
            if self.depths[i] != max(lc_depth, rc_depth) + 1{
                panic!("depth_check");
            }

            if (i16::from(lc_depth) - i16::from(rc_depth)).pow(2) > 1 {
                panic!("balance check {}", (i16::from(lc_depth) - i16::from(rc_depth)));
            }
            if i!= 0 && self.nodes[self.parent[i] as usize].rc() != Option::Some(i) && self.nodes[self.parent[i] as usize].lc() != Option::Some(i) {
                panic!("parent-child check");
            }



            let lc_check = match self.nodes[i].lc() {
                None => true, 
                Some(lc) => self.nodes[i].key > self.nodes[lc].key, 
            };
            let rc_check = match self.nodes[i].rc() {
                None => true, 
                Some(rc) => self.nodes[i].key < self.nodes[rc].key, 
            };

            if !(rc_check && lc_check) {
                panic!("bst property check");

            }

        }
            
        //V = E-1 <=> valid tree
        if links != self.nextfree-1 {
            panic!("tree check");
        }


    }

}
pub fn test() {

    let x = sample(16);

    let mut memtable = Bst::new();
    let k = 400;
    // for i in 0..k {
    //     memtable.put(20-i, i);
    //     memtable.validate();
    //     println!();
    //
    // }
    // for i in 0..k {
    //     memtable.put(20-i, i);
    //     memtable.validate();
    //     println!();
    // }
    //
    //
    for i in 0..k {
        // println!("{}", x.hash(i));
        memtable.put(x.hash(i), i);
        // println!();

    }
    // memtable.validate();
    // memtable.rl(0);
    // memtable.rl(0);
    // memtable.put(4,0);

   // for i in 0..k {
   //      assert!(memtable.get(x.hash(i)).is_some());
   //      assert!(x.hash(memtable.get(x.hash(i)).unwrap()) == x.hash(i));
   //      // println!("{:?} {} {}", memtable.get(x.hash(i)), i, x.hash(i) );
   //  }



}

