use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    name: String,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: String) -> Self {
        Node {
            name,
            children: Vec::new(),
        }
    }
    
    fn display(&self, depth: usize) {
        let indent = "  ".repeat(depth);
        println!("{}- {} \n", indent, self.name);
        
        for child in &self.children {
            child.borrow().display(depth + 1);
        }
    }
}

fn main() {
    println!("=== Contoh Binary Tree Keluarga Jokowi ===\n");
    
    let jokowi = Rc::new(RefCell::new(Node::new("jokowi".to_string())));
    let gibran = Rc::new(RefCell::new(Node::new("gibran".to_string())));
    let kahiyang_ayu = Rc::new(RefCell::new(Node::new("kahiyang ayu".to_string())));
    let kaesang = Rc::new(RefCell::new(Node::new("kaesang".to_string())));
    let jan_ethes = Rc::new(RefCell::new(Node::new("jan ethes".to_string())));
    
    println!("Strong counts setelah create:");
    println!("  {}: {}", jokowi.borrow().name, Rc::strong_count(&jokowi));
    println!("  {}: {}", gibran.borrow().name, Rc::strong_count(&gibran));
    println!("  {}: {}", kahiyang_ayu.borrow().name, Rc::strong_count(&kahiyang_ayu));
    println!("  {}: {}", kaesang.borrow().name, Rc::strong_count(&kaesang));
    println!("  {}: {}", jan_ethes.borrow().name, Rc::strong_count(&jan_ethes));
    
    jokowi.borrow_mut().children.push(Rc::clone(&gibran));
    jokowi.borrow_mut().children.push(Rc::clone(&kahiyang_ayu));
    jokowi.borrow_mut().children.push(Rc::clone(&kaesang));
    gibran.borrow_mut().children.push(Rc::clone(&jan_ethes));
    
    println!("\nStrong counts setelah building:");
    println!("  {}: {}", jokowi.borrow().name, Rc::strong_count(&jokowi));
    println!("  {}: {}", gibran.borrow().name, Rc::strong_count(&gibran));
    println!("  {}: {}", kahiyang_ayu.borrow().name, Rc::strong_count(&kahiyang_ayu));
    println!("  {}: {}", kaesang.borrow().name, Rc::strong_count(&kaesang));
    println!("  {}: {}", jan_ethes.borrow().name, Rc::strong_count(&jan_ethes));
    
    println!("\nStruktur Tree:");
    jokowi.borrow().display(0);
}