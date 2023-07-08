use std::env;

use crate::controlador::manipular_info::crypto_base::{hash_contra_maestra, derivar_llave};



mod clipboard_generic;
mod panel;
mod ejemplo_sql;
mod controlador;

fn main() {
    
    for argument in env::args(){
        match argument.as_str(){
            "--test"=>tests(),
            _=>{}

        }
    }
    
    ejemplo_sql::test_3();
    let clave="contra".to_owned();
    let mut sal=[0u8;16];
    let mut llave=[0u8;32];
    derivar_llave(&clave, &mut llave, &mut sal);
    let hash_clave=hash_contra_maestra(&clave.as_bytes(), &sal);
    println!("key={:?}\n key_hash={:?}",llave,hash_contra_maestra(&clave.as_bytes(), &sal));
    assert_ne!(hash_contra_maestra(&clave.as_bytes(), &sal),llave);
    assert_eq!(hash_clave,hash_contra_maestra(&clave.as_bytes(), &sal));

}

fn tests(){
    ejemplo_sql::test_3();
    let clave="contra".to_owned();
    let mut sal=[0u8;16];
    let mut llave=[0u8;32];
    derivar_llave(&clave, &mut llave, &mut sal);
    //let mut sal_loging=sal.clone();
    //sal_loging[2]=0u8;
    println!("key={:?}\n key_hash={:?}",llave,hash_contra_maestra(&llave, &sal));
    assert_ne!(hash_contra_maestra(&llave, &sal),llave);
}
