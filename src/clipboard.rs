use clipboard_win::{get_clipboard_string, set_clipboard_string};


/// Esta funcion copia un str al portapapeles de windows
/// # Arguments
/// * `texto` es la cadena que se copiará al portapapeles
pub fn copiar_al_portapapeles(texto: &str) {
    // Copiar texto al portapapeles
    if set_clipboard_string(texto).is_ok() {
        println!("Texto copiado al portapapeles");
    } else {
        println!("No se pudo copiar el texto al portapapeles");
    }
}

/// Esta función intenta retornar el texto guardado en el portapapeles
/// funciona solo en windows
/// # Returns
/// el texto guardado en el portapapeles
pub fn texto_del_portapapeles()-> &str {
    //intenta obtener y devolver el texto almacenado en el portapapeles
    if let Ok(clipboard_text) = get_clipboard_string() {
        println!("Contenido del portapapeles: {}", clipboard_text);
    } else {
        println!("No se pudo obtener el contenido del portapapeles");
    }
    clipboard_text
}