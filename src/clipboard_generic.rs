use clipboard::{ClipboardProvider, ClipboardContext};


pub fn copiar_al_portapapeles(texto: &str) {
    let mut ctx:ClipboardContext=ClipboardProvider::new().unwrap();
    if ctx.set_contents(texto.to_owned()).is_ok() {
        match ctx.get_contents(){
            Ok(_)=>println!("Texto copiado al portapapeles"),
            Err(_)=>println!("COULD NOT COPY")
        }
    } else {
        println!("No se pudo copiar el texto al portapapeles");
    }
}