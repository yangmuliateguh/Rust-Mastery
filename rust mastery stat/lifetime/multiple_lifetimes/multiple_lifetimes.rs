fn select_first<'a, 'b>(
    x: &'a str,
    y: &'b str,
    pick_x: bool
) -> &'a str {
    if pick_x { x }
    else { panic!("hanya x yang boleh") }
}

fn main(){
    let s1 = String::from("hidup lama");
    let result;
    {
        let s2 = String::from("hidup bentar");
        result = select_first(&s1, &s2, false);
    }
    println!("{}", result);
}