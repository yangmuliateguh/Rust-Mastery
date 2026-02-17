fn create_boxed_vector(values: Vec<i32>) -> Box<Vec<i32>> {
    Box::new(values)
}

fn main(){
    let vector = vec![1,2,3];
    let boxed_vector = create_boxed_vector(vector);
    println!("{:?}", boxed_vector);
    println!("{:p}", boxed_vector);
}