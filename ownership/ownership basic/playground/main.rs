fn maybe_consume(s: String, keep: bool) -> Option<String> {
    if keep {
        Some(s)
    } else {
        println!("{} dibuang", s);
        None
    }
}

fn main (){
    let s1 = String::from("opsi a");
    let s2 = String::from("opsi b");

    let kept = maybe_consume(s1, false);

    if let Some(v) = kept {
        println!("{} disimpan", v);
    }
}