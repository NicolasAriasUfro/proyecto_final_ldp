
use crate::controlador::manipular_info::crypto_base::{hash_contra_maestra, Criptografia};
use self::crypto_base::{hash_entregado_valido, derivar_llave};
pub mod crypto_base;
pub mod generador_contra;
pub mod info_almacenada;


pub fn comprobar_contra_maestra(contra_entregada:&String,sal:&[u8;16],hash_almacenado:&Vec<u8>)->bool{
    let hash_contra_entregada=hash_contra_maestra(contra_entregada.as_bytes(), sal);
    hash_entregado_valido(&hash_contra_entregada, hash_almacenado.as_slice())
}

pub fn iniciar_base_de_datos_existente(contra_maestra:&String,sal:&[u8;16])->Criptografia{
    let llave_maestra=derivar_llave(contra_maestra, sal);
    Criptografia::new(&llave_maestra)
}