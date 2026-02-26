use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug)]
struct Item{
    id: u32,
    name: String,
    quantity: u32
}

struct Inventory {
    items: RefCell<Vec<Item>>
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            items: RefCell::new(Vec::new())
        }
    }

    fn tambah_item(&self, item: Item) {
        self.items.borrow_mut().push(item);
    }

    fn kurang_stok(&self, id: u32, jumlah: u32) -> Result<(), String>{
        let mut items = self.items.borrow_mut();
        for item in items.iter_mut(){
            if item.id == id {
                if item.quantity >= jumlah {
                    item.quantity -= jumlah;
                    return Ok(())
                } else {
                    return Err("stok tidak mencukupi".to_string());
                }
            }
        }
        return Err("id tidak ditemukan".to_string());
    }

    fn tampilkan_semua(&self) {
        for item in self.items.borrow().iter() {
            println!("{:?}", item);
        }
    }
}

fn main(){
    let inventory = Inventory::new();
    
    inventory.tambah_item(Item { id: 1, name: "Laptop".to_string(), quantity: 20 });
    inventory.tambah_item(Item { id: 2, name: "PC".to_string(), quantity: 30 });

    inventory.tampilkan_semua();

    match inventory.kurang_stok(1, 2) {
        Ok(_) => println!("berhasil mengurang stok"),
        Err(e) => println!("error: {}", e)
    }

    match inventory.kurang_stok(2, 2) {
        Ok(_) => println!("berhasil mengurang stok"),
        Err(e) => println!("error: {}", e)
    }

    println!("stok: ");
    inventory.tampilkan_semua();
}