use std::io;
use muslimdb::memati::KurtlarVadisi;

fn main() -> io::Result<()> {
    let mut kv_store = KurtlarVadisi::new();

    kv_store.add("username", "user123");
    kv_store.add("email", "user@example.com");

    kv_store.save("kv_data.bin")?;
    println!("Veri başarıyla kaydedildi!");

    let loaded_kv_store = KurtlarVadisi::load("kv_data.bin")?;
    println!("Yüklenen Veri: {:?}", loaded_kv_store);
    
    Ok(())
}
