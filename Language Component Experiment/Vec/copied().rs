fn main(){
    let v = vec![1,2,3];
    let doubled: Vec<i32> = v.iter()
        .copied()
        .map(|x|x*2)
        .collect();
    println!("{:?}", doubled);
}