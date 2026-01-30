fn main(){
    let v = vec![1,2,3];
    let result = v.iter().find(|&&x| x > 1);
    println!("{:?}", result);
}