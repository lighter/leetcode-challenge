fn main() {
    // println!("Hello, world!");
    let word:String = String::from("apple");
    let prefix:String = String::from("app");
    let search_word:String = String::from("apple");
    let mut obj = Trie::new();
    obj.insert(word);
    let ret_2: bool = obj.search(search_word);
    println!("{}", ret_2);
    let ret_3: bool = obj.starts_with(prefix);
    println!("{}", ret_3);
}

#[derive(Default)]
struct Trie {
    end: bool,
    nodes: [Option<Box<Trie>>; 26]
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Default::default()
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let mut current = self;
        for i in word.chars().map(|c| (c as u8 - 'a' as u8) as usize) {
            current = current.nodes[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        current.end = true;
    }
    
    /** Returns if the word is in the trie. */
    fn search(&self, word: String) -> bool {
        let mut current = self;
        for i in word.chars().map(|c| (c as u8 - 'a' as u8) as usize) {
            match current.nodes[i].as_ref() {
                Some(node) => { current = node; },
                None => { return false; }
            }
        }

        current.end
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&self, prefix: String) -> bool {
        let mut current = self;
        for i in prefix.chars().map(|c| (c as u8 - 'a' as u8) as usize) {
            match current.nodes[i].as_ref() {
                Some(node) => { current = node; },
                None => { return false; }
            }
        }
        true
    }
}
