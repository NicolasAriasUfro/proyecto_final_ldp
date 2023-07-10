use std::time::{self, UNIX_EPOCH};

use self::{crypto_base::{crear_nonce, derivar_llave}, info_almacenada::Entrada};

pub mod crypto_base;
mod generador_contra;
pub mod info_almacenada;


pub fn modificar_titulo(entrada: &mut Entrada, titulo_nuevo: String) {
    let titulo_asignable;
    if titulo_nuevo.trim()==""{
        titulo_asignable=None;
    }
    else{
        titulo_asignable=Some(titulo_nuevo);
    }
    entrada.set_titulo(titulo_asignable);
}


pub fn iniciar_nueva_base_de_datos(){
    let contra_maestra=pedir_nueva_contra_maestra();
    let mut llave_maestra=[0u8;32];
    let mut sal:[u8;16]=[0u8;16];
    derivar_llave(&contra_maestra,&mut llave_maestra, &mut sal);
    println!("len llave maestra: {}, contenidos: {:?}",llave_maestra.len(),llave_maestra);

}

fn pedir_nueva_contra_maestra()->String{
    "password12345".to_owned()
}