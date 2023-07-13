use clipboard::{ClipboardProvider, ClipboardContext};

/// Esta funcion copia un str al portapapeles de windows
/// # Arguments
/// * `texto` es la cadena que se copiará al portapapeles
pub fn copiar_al_portapapeles(texto: &str) {
    let mut ctx:ClipboardContext=ClipboardProvider::new().unwrap();
    // Copiar texto al portapapeles
    if ctx.set_contents(texto.to_owned()).is_ok() {
        match ctx.get_contents(){
            Ok(_)=>println!("Texto copiado al portapapeles"),
            Err(_)=>println!("COULD NOT COPY")
        }
    } else {
        println!("No se pudo copiar el texto al portapapeles");
    }
}