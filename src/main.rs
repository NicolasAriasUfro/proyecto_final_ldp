mod crypto_base;
mod clipboard;

fn main() {
    println!("Hello, world!");
    clipboard::copiar_al_portapapeles("tula");
    println!("el sexo del portapapeles es: {}", clipboard::texto_del_portapapeles());
    
}
