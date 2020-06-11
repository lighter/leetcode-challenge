fn main() {
    // println!("Hello, world!");

    let nums = vec![2,3,5];

    let mut q = FirstUnique::new(nums);

    // println!("{:?}", q);
    // let mut cur = fu.head;
    // println!("{:?}", cur.take());
    // println!("{:?}", cur.unwrap().next);

    // let mut current = fu.head;
    // loop {
    //     println!("current: {:?}", current);
    //     match current {
    //         None => {
    //             println!("None");
    //             break;
    //         },
    //         Some(node) => { 
    //             println!("node: {:?}", node.next);
    //             current = node.next;
    //         }
    //     }
    // }
    // println!("show first unique: {}", q.show_first_unique());
    q.add(5);
    // println!("show first unique: {}", q.show_first_unique());
    // q.add(2);
    // println!("show first unique: {}", q.show_first_unique());
    // q.add(3);
    // println!("show first unique: {}", q.show_first_unique());
}

#[derive(Debug)]
struct EntryNode {
    key: i32,
    next: Option<Box<EntryNode>>
}

#[derive(Debug)]
struct FirstUnique {
    head: Option<Box<EntryNode>>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FirstUnique {

    fn new(nums: Vec<i32>) -> Self {
        let mut cur = None;

        for &num in nums.iter().rev() {
            let mut new_node = EntryNode {
                key: num,
                next: None
            };

            new_node.next = cur;
            cur = Some(Box::new(new_node));
        }

        Self { head: cur }
    }
    
    // fn show_first_unique(&mut self) -> i32 {
    //     for &num in self.nums.iter() {
    //         if self.table.get(&num).unwrap() == &1 {
    //             return num;
    //         }
    //     }

    //     return -1;
    // }
    
    fn add(&mut self, value: i32) {
        // if self.table.contains_key(&value) {
        //     *self.table.get_mut(&value).unwrap() += 1;
        // } else {
        //     self.table.insert(value, 1);
        // }
        
        // self.nums.push(value);
        // println!("{}",value);

        let mut current = &self.head;

        loop {

            match current {
                None => {
                    // println!("None");
                    current.as_ref().unwrap().next = Some(Box::new(EntryNode {
                        key: value,
                        next: None
                    }));
                },
                Some(node) => { 
                    // println!("node: {:?}", &node);
                    
                    current = &node.next;

                    // println!("{:?}", current);
                }
            }
        }
    }
}