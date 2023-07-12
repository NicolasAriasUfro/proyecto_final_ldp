use std::time::{self, UNIX_EPOCH};

use crate::{controlador::manipular_info::crypto_base::{hash_contra_maestra, Criptografia}, controller_sql::agregar_master};

use self::{crypto_base::{crear_nonce, crear_llave, hash_entregado_valido, derivar_llave}, info_almacenada::Entrada};

pub mod crypto_base;
pub mod generador_contra;
pub mod info_almacenada;


pub fn iniciar_nueva_base_de_datos(contra_maestra:&String)->Criptografia{
    let mut llave_maestra=[0u8;32];
    let mut sal:[u8;16]=[0u8;16];
    crear_llave(&contra_maestra,&mut llave_maestra, &mut sal);
    let hash_maestro=hash_contra_maestra(&contra_maestra.as_bytes(), &sal);
    println!("len llave maestra: {}, contenidos: {:?}",llave_maestra.len(),llave_maestra);
    match agregar_master(&hash_maestro, &sal){
        Ok(())=>Criptografia::new(&llave_maestra),
        Err(_)=>panic!("no se pudo insertar datos de la llave maestra a la base de datos")
    }

    

}

pub fn comprobar_contra_maestra(contra_entregada:&String,sal:&[u8;16],hash_almacenado:&String)->bool{
    //let sal: &[u8; 16]=sal.try_into().expect("sal no tenia 16 caracteres o no era byte");
    let hash_contra_entregada=hash_contra_maestra(contra_entregada.as_bytes(), sal);
    hash_entregado_valido(&hash_contra_entregada, hash_almacenado.as_bytes())

}

pub fn iniciar_base_de_datos_existente(contra_maestra:&String,sal:&[u8;16])->Criptografia{
    let llave_maestra=derivar_llave(contra_maestra, sal);
    Criptografia::new(&llave_maestra)


}
