// allows pretty print
#[derive(Debug)]

struct Item {
    pub count: usize
}

// value owners cannot change - functions must take in a reference
fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

pub fn borrow() {
    let mut item = Item { count: 1 };
    let mut items = vec![Item {count: 1}];
    let first = items.first_mut();
    println!("{:?}", first);
    let second = items.get_mut(1);
    println!("{:?}", second);

    
    println!("{:?}", item);
    add_one(&mut item);
    println!("{:?}", item);
    
    // println!("{:?}", first);

    print_all(&items);
}

