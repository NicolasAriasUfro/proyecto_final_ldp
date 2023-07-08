use std::time::{self, UNIX_EPOCH};

use self::{info_almacenada::Entrada, crypto_base::crear_nonce};

pub mod crypto_base;
mod info_almacenada;
mod generador_contra;


pub fn nueva_entrada(titulo:String,usuario:Option<String>,plaintext_contra:String,url:Option<String>)->Entrada{
    let fecha=time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(); //use unwrap porque tecnicamente nunca podria estar mal respecto a la epoch?
    //dice que da error si 'earlier' es mas tarde que self, la epoch es 1970 y self es fecha actual por lo que siempre estaria bien...
    let nonce=crear_nonce();

    Entrada::new_creado(titulo, usuario, plaintext_contra, nonce, fecha, url)
    
}

pub fn modificar_titulo(entrada:&mut Entrada,titulo:String){
    
}



