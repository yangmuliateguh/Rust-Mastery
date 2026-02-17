#[derive(Debug)]
enum LinkedList {
    Cons(i32, Box<LinkedList>),
    Nil
}

impl LinkedList {
    fn new(values: &[i32]) -> Self {
        let mut list = Self::Nil;
        for &value in values.iter().rev() {
            list = Self::Cons(value, Box::new(list));
        }
        list
    }

    fn print_list(&self) {
        let mut current = self;
        while let Self::Cons(value, next) = current {
            println!("{}", value);
            current = next.as_ref()
        }
    }
}

fn main(){
    let list = LinkedList::new(&[1,2,3]);
    list.print_list();
    println!("{:?}", list);
}