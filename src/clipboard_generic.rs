use clipboard::{ClipboardProvider, ClipboardContext};

/// Esta funcion copia un str al portapapeles de windows
/// # Arguments
/// * `texto` es la cadena que se copiará al portapapeles
pub fn copiar_al_portapapeles(texto: &str) {
    let mut ctx:ClipboardContext=ClipboardProvider::new().unwrap();
    // Copiar texto al portapapeles
    if ctx.set_contents(texto.to_owned()).is_ok() {
        match ctx.get_contents(){
            Ok(_)=>println!("[GENERIC] Texto copiado al portapapeles"),
            Err(_)=>println!("COULD NOT COPY")
        }
    } else {
        println!("No se pudo copiar el texto al portapapeles");
    }
}

/// Esta función intenta retornar el texto guardado en el portapapeles
/// funciona solo en windows
/// # Returns
/// el texto guardado en el portapapeles
pub fn texto_del_portapapeles()-> String {
    //intenta obtener y devolver el texto almacenado en el portapapeles
    let mut ctx:ClipboardContext=ClipboardProvider::new().unwrap();
    if let Ok(clipboard_text) = ctx.get_contents() {
        println!("[GENERIC] Contenido del portapapeles: {}", clipboard_text);
        return  clipboard_text;
    
    } else {
        println!("No se pudo obtener el contenido del portapapeles");
        return "".to_string();
    }
}