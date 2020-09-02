use std::collections::HashMap;

struct Pnode {
    slice: String,
    hmap: HashMap<char, Child>,
}

type Child = Box<Pnode>;

pub struct Ptree {
    root: Child,
    size: usize,
}

fn getlvl(s1: &str, s2: &str) -> usize {
    let mut count: usize = 0;
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();
    let mlen = if v1.len() < v2.len() {
        v1.len()
    } else {
        v2.len()
    };

    while count < mlen {
        if &v1[count] != &v2[count] {
            break;
        }
        count += 1;
    }

    return count;
}

impl Pnode {
    fn new(sin: Option<&str>) -> Self {
        return Self {
            slice: match sin {
                None => String::new(),
                Some(s) => String::from(s),
            },
            hmap: HashMap::new(),
        };
    }

    fn update_hmap(&mut self, o_sin: Option<&str>, slice: char, lvl: usize) {
        let sin: &str = match o_sin {
            None => &self.slice,
            Some(s) => s,
        };

        if let Some(node) = self.hmap.get_mut(&slice) {
            node.insert(&sin[lvl..]);
        } else {
            self.hmap
                .insert(slice, Box::new(Pnode::new(Some(&sin[lvl..]))));
        }
    }

    fn insert(&mut self, sin: &str) {
        let lvl: usize = getlvl(&self.slice, sin);
        if let Some(slice) = sin.chars().nth(lvl) {
            self.update_hmap(Some(sin), slice, lvl);
        }

        if let Some(slice) = self.slice.chars().nth(lvl) {
            self.update_hmap(None, slice, lvl);
        }
    }

    fn find(&self, sfin: &str) -> bool {
        let lvl: usize = getlvl(&self.slice, sfin);
        if let Some(slice) = sfin.chars().nth(lvl) {
            match self.hmap.get(&slice) {
                Some(node) => {
                    return node.find(&sfin[lvl..]);
                }
                None => {
                    return false;
                }
            }
        }

        return true;
    }
}

impl Ptree {
    pub fn new() -> Self {
        return Self {
            root: Box::new(Pnode::new(None)),
            size: 0,
        };
    }

    pub fn insert(&mut self, sin: &str) {
        self.root.insert(sin);
    }

    pub fn find(&self, sfin: &str) -> bool {
        return self.root.find(sfin);
    }
}
