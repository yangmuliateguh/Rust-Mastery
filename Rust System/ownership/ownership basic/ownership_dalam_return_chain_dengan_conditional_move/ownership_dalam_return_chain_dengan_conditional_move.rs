fn maybe_consume(s: String, keep: bool) -> Option<String> {
    if keep {
        Some(s) // kembalikan kepemilikan
    } else {
        println!("Dibuang: {}", s);
        None // `s` di-drop di sini
    }
}

fn main() {
    let s1 = String::from("opsi A");
    let s2 = String::from("opsi B");

    let kept = maybe_consume(s1, true);
    // maybe_consume(s2, false); // s2 di-drop di dalam fungsi

    if let Some(v) = kept {
        println!("Disimpan: {}", v);
    }
}