fn create_large_array<const N: usize>() -> Box<[i32; N]> {
    Box::new([0; N])
}

fn main(){
    let arr = create_large_array::<10>();
    println!("{:?}", arr);
}