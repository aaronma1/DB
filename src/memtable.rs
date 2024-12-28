
const MEMTABLE_SIZE: usize = 2048;

#[derive(Copy, Clone)]
#[repr(packed)]
struct BstNode {
    key: usize,
    val: usize,
    // lc and rc are 0
    lc: u16, 
    rc: u16,
}

impl BstNode {
    fn new() -> BstNode {
        BstNode {
            key:0,
            val:0,
            lc: MEMTABLE_SIZE as u16,
            rc: MEMTABLE_SIZE as u16,
        }
    }
}


struct Bst {
    root: usize,
    nextfree: usize,
    // 0 is a NULL node
    nodes: [BstNode; MEMTABLE_SIZE+1],
}


impl Bst {

    fn new() -> Bst {
        Bst {
            root: MEMTABLE_SIZE,
            nextfree: 0, 
            nodes: [BstNode::new(); MEMTABLE_SIZE+1],

        }
    }


    // search bst for key 
    // returns the root of the smallest subtree containing key
    fn search(&self, key: usize) -> usize {
        if self.root == MEMTABLE_SIZE {
            return self.nextfree;
        }

        let mut cur = self.root;
        let mut next: usize;

        loop {
            if key > self.nodes[cur].key {
                next = self.nodes[cur].rc as usize;
            } else if cur == key {
                return cur
            } else {
                next = self.nodes[cur].lc as usize;
            }

            if next == MEMTABLE_SIZE {
                return cur;
            }else {
                cur = next;
            }
        }
    }

    fn rightmost_leaf(&self, index: usize) -> usize {
        let mut cur = index;
        let mut next: usize;

        loop {
            next = self.nodes[cur].rc as usize;
            if next == MEMTABLE_SIZE {
                return cur;
            }
            cur = next;
        }
    }

    fn leftmost_leaf(&self, index: usize) -> usize {
        let mut cur = index;
        let mut next: usize;

        loop {
            next = self.nodes[cur].lc as usize;
            if next == MEMTABLE_SIZE {
                return cur;
            }
            cur = next;
        }
    }


    fn put(&mut self, key: usize, val: usize) {
        
        
        let idx = self.search(key);



        if self.nodes[idx].key == key {
            self.nodes[idx].val = val;
            return;
        }


        if self.nodes[idx].key > key {

            if self.nodes[idx].lc == MEMTABLE_SIZE {

            }


        }

        if self.nodes[idx].key < key{
            
            if self.nodes[idx].rc == MEMTABLE_SIZE {

            }

        }

        self.nodes[self.nextfree] = BstNode {
            key,
            val,
            lc: MEMTABLE_SIZE as u16,
            rc: MEMTABLE_SIZE as u16,
        };


    }

    fn get(self, key: usize) -> Option<usize> {




    }

}
