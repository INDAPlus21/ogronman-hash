
use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for String {
    //sdbm hash-encoding for a given string
    fn hash(&self) -> usize {
        let mut hash: usize = 0;
        for _c in self.encode_utf16() {
            hash = usize::from(_c)
                .wrapping_add(hash << 6)
                .wrapping_add(hash << 16)
                .wrapping_sub(hash);
        }

        hash
    }

}

impl Hashable for usize {
    fn hash(&self) -> usize {
        *self
    }
}

pub fn hash_string(key: &String) -> usize {
    key.hash()
}


//Java like things in rust :)
#[derive(Default, Clone)]
pub struct HashNode<Key, Value> {
    key: Key,
    value: Value,
    taken: bool,
}


pub struct HashTableMapThing<Key, Value> {
    cells: Vec<HashNode<Key, Value>>,
    count: usize,
}

impl <Key, Value> HashTableMapThing<Key, Value> 
    where 
        Key: Clone + Default + Debug + Hashable + PartialEq,
        Value: Clone + Default + Debug,
    {

    //Creates a new HashTableMapThing with the initial capacity of initial
    pub fn new() -> Self {
        const INITIAL: usize = 5;
        Self { 
            cells: vec![HashNode::<_,_>::default(); INITIAL],
            count: 0,
        }
    }

    pub fn extend(&mut self) {
        let mut new = Self{
            cells: vec![HashNode::<_,_>::default(); self.cells.len() * 2 + 1],
            count: 0,
        };

        for cell in self.cells.iter() {
            if cell.taken == true {
                new.insert(cell.key.clone(), cell.value.clone());
            }
        }

        *self = new;

    }

    //Blir väldigt oeffektiv när hashtablemapthing blir väldigt stor men borde fungera för miiiig ;)
    pub fn print(&self){
        for c in &self.cells {
            if c.taken == true {    
                println!("{:#?}", c.value);
            }
        }
    }

    pub fn delete(&mut self, key: Key){


        let mut index = key.hash() % self.cells.len();

        while self.cells[index].key != key {
            index = (index + 1) % self.cells.len();
        }

        self.cells[index] = HashNode::<_,_>::default();
        self.count -= 1;

    }

    pub fn insert(&mut self, key: Key, value: Value) {
        if self.count >= self.cells.len() {
            self.extend();
        }

        let mut index = key.hash() % self.cells.len();


        //Key-stepping or something to avoid collisions :)
        while self.cells[index].taken {
            index = (index + 1) % self.cells.len();
        }

        self.cells[index].taken = true;
        self.cells[index].key = key;
        self.cells[index].value = value;
        self.count += 1;
        
    }

    pub fn contains(&self, key: &Key) -> bool{

        let mut index = key.hash() % self.cells.len();
        let first = index;
        while self.cells[index].taken{      //I know its kinda ugly but it works, and if i did not have to print to file then it would be easier
            if &self.cells[index].key == key {
                return true;
            }
            index = (index + 1) % self.cells.len();
            if index == first {
                break;
            }
        }
        false
    }

    pub fn get_index(&self, key: &Key) -> Option<usize> {
        let mut index = key.hash() % self.cells.len();

        for _ in 0..self.cells.len() {
            //If there are no cells with the given key return None
            if !self.cells[index].taken {
                return None;
            }
            //If the index gives the correct key return the index
            if self.cells[index].key == *key {
                return Some(index);
            }
            //Continue through whole tablemapthing
            index = (index + 1) % self.cells.len();
        }
        None
    }

    pub fn get(&self, key: &Key) -> Option<&Value> {
        if let Some(index) = self.get_index(key){
            Some(&self.cells[index].value)
        }else{
            None
        }
    }

}
