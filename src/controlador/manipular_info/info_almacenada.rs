use core::panic;
use std::time::{self, UNIX_EPOCH};

use super::crypto_base::{Criptografia, crear_nonce};
#[derive(Debug)]
pub struct Entrada {
    pub(crate) id: u64,
    pub titulo:String,
    pub nombre_usuario: String,
    pub contrasena: String,
    pub(crate) nonce: [u8; 12],
    pub(crate) fecha_creacion: u64,
    pub url: String,
}

impl Entrada {
    pub fn new_desde_bd(
        id: u64,
        titulo: String,
        nombre_usuario: String,
        contrasena: String,
        nonce: [u8; 12],
        fecha_creacion: u64,
        url: String,
    ) -> Self {
        
        Self {
            id,
            titulo,
            nombre_usuario,
            contrasena,
            nonce,
            fecha_creacion,
            url,
        }
    }

    pub fn new_creado(
        titulo: String,
        nombre_usuario: String,
        contrasena: String,
        url: String,
    ) -> Self {

        let nonce=crear_nonce();
        let fecha_creacion = time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
        Self {
            id: 0,
            titulo,
            nombre_usuario,
            contrasena,
            nonce,
            fecha_creacion,
            url,
        }
    }


    pub fn cifrar_contra(&self,cifrador:&Criptografia)->Vec<u8>{
        let contra_cifrada=match cifrador.cifrar_bytes(&self.nonce, self.contrasena.as_bytes()){
            Some(cyphertext)=>cyphertext,
            None=>panic!("algo salio mal con cifrar contra {} y nonce {:?} ",self.contrasena,self.nonce)
        };
        //general_purpose::STANDARD_NO_PAD.encode(contra_cifrada)
        contra_cifrada
    }

    

}

pub fn descifrar_contra(contra_cifrada:Vec<u8>,nonce:Vec<u8>,cifrador:&Criptografia)->String{
    let contra_cifrada_bytes=contra_cifrada.as_slice();
    let nonce:&[u8;12]=nonce.as_slice().try_into().expect("nonce no era de 12 bytes o algo salio mal...");

    let contra_descifrada_bytes=match cifrador.descifrar_bytes(nonce, &contra_cifrada_bytes){
        Some(descifrado)=>descifrado,
        None=>panic!("algo salio mal con descifrar contra {:?} y nonce {:?} ",contra_cifrada_bytes,nonce)
    };
    match String::from_utf8(contra_descifrada_bytes.clone()){
        Ok(resultado)=>resultado,
        Err(_)=>panic!("{:?} no correspondia a un string",contra_descifrada_bytes)
    }
}
