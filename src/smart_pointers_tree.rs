use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

// We want a Node to own its children, and we want to share that ownership with variables so we
// can access each Node in the tree directly. To do this, we define the Vec<T> items to be values
// of type Rc<Node>. We also want to modify which nodes are children of another node, so we have
// a RefCell<T> in children around the Vec<Rc<Node>>.

pub fn run(){
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node{
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}