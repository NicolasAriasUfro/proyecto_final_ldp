mod crypto_base;
mod clipboard;

fn main() {
    println!("Hello, world!");
    clipboard::copiar_al_portapapeles("test");
    let bts = "V";
    println!("{}", bts);
}
