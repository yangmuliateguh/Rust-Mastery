#[derive (Debug)]
struct Inner {
    value: String
}

#[derive (Debug)]
struct Outer {
    inner: Inner
}

fn update_value(o: &mut Outer) {
    o.inner.value.push_str(" -updated");
}

fn main(){
    let mut outer = Outer {
        inner: Inner {
            value: "asli".into()
        }
    };
    println!("{:?}", outer);
    update_value(&mut outer);
    println!("{:?}", outer);
}