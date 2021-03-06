pub struct BinPq {
    vec: Vec<usize>,
    compfn: Compfn,
}

type Compfn = fn(usize, usize) -> bool;

pub fn greater(a: usize, b: usize) -> bool {
    return a > b;
}

pub fn lesser(a: usize, b: usize) -> bool {
    return a < b;
}

fn getparent(indx: usize) -> usize {
    if indx % 2 == 0 {
        return (indx - 2) / 2;
    } else {
        return (indx - 1) / 2;
    }
}

// Public impl
impl BinPq {
    pub fn new(compfn: Compfn) -> Self {
        return Self {
            vec: Vec::new(),
            compfn: compfn,
        };
    }

    pub fn getroot(&self) -> Option<usize> {
        match self.vec.len() {
            0 => {
                return None;
            }
            _ => {
                return Some(self.vec[0]);
            }
        }
    }

    pub fn insert(&mut self, val: usize) {
        self.vec.push(val);
        // bbl up
        let mut curr = self.vec.len() - 1;
        while curr >= 0 {
            let parent = getparent(curr);
            if !(self.compfn)(curr, parent) {
                break;
            }
            self.vec.swap(curr, parent);
        }
    }

    pub fn remove(&mut self, val: usize) -> Option<usize> {
        let mut curr = self.vec.iter().position(|&x| x == val)?;
        let res = self.vec.swap_remove(curr);
        //bbl down
        while curr < self.vec.len() {
            let mut child = 2 * curr + 1;
            if (self.compfn)(curr, child) {
                child += 1;
                if (self.compfn)(curr, child) {
                    break;
                }
            }
            self.vec.swap(curr, child);
            curr = child;
        }

        return Some(res);
    }
}
