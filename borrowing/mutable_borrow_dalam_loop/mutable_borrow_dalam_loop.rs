fn main(){
    let mut data = vec![1,2,3];

    for i in 0..data.len(){
        let val = &mut data[i];
        *val += 10;
    }

    println!("{:?}", data);
}