use std::{env, process::exit};

use base64::{engine::general_purpose, Engine};
use controlador::manipular_info::crypto_base::crear_nonce;

use crate::controlador::manipular_info::{crypto_base::{hash_contra_maestra, derivar_llave}, info_almacenada::{recrear_nonce}, iniciar_nueva_base_de_datos};
use crate::controlador::manipular_info::info_almacenada::Entrada;


mod clipboard_generic;
mod panel;
mod controller_sql;
mod controlador;

fn main() {
    
    for argument in env::args(){
        match argument.as_str(){
            "--test"=>{
                tests();
                exit(0);
            },
            _=>{}

        }
    }
    
        test_panel();
    
    controller_sql::set_database();
    test_nico();    
    controller_sql::agregar_cuenta(&Entrada::new_creado(None,"twitter".to_owned(),"12345678".to_owned(),*b"abcdefghijkl",None)).unwrap();

    let clave="contra".to_owned();
    let mut sal=[0u8;16];
    let mut llave=[0u8;32];
    derivar_llave(&clave, &mut llave, &mut sal);
    let hash_clave=hash_contra_maestra(&clave.as_bytes(), &sal);
    println!("key={:?}\n key_hash={:?}",llave,hash_contra_maestra(&clave.as_bytes(), &sal));
    assert_ne!(hash_contra_maestra(&clave.as_bytes(), &sal),llave);
    assert_eq!(hash_clave,hash_contra_maestra(&clave.as_bytes(), &sal));

}
fn test_nico(){
    let cuenta_1:Entrada = Entrada::new_creado(
        Some(String::from("Codeforces")),
        "nico_arias".parse().unwrap(),
        "asd24355##".parse().unwrap(),
        [0,0,0,0,00,0,0,0,0,0,0,1],
        Some(String::from("https://codeforces.com/"))
    );
    print!("mdg 2");
    let error = controller_sql::agregar_cuenta(&cuenta_1);

    println!("{:#?}",error);
    println!("{:?}", controller_sql::listar_cuentas());


}

fn tests(){
    println!("testing.");
    let clave="contra".to_owned();
    let mut sal=[0u8;16];
    let mut llave=[0u8;32];
    derivar_llave(&clave, &mut llave, &mut sal);
    //let mut sal_loging=sal.clone();
    //sal_loging[2]=0u8;
    println!("key={:?}\n key_hash={:?}",llave,hash_contra_maestra(&llave, &sal));
    assert_ne!(hash_contra_maestra(&llave, &sal),llave);
    reconstruir_nonce();
    iniciar_nueva_base_de_datos();
}

fn reconstruir_nonce(){
    let nonce=crear_nonce();
    println!("nonce: {:?}",nonce);

    let nonce_b64=general_purpose::STANDARD_NO_PAD.encode(&nonce);
    let decoded=recrear_nonce(nonce_b64);
    println!("decoded: {:?}",decoded);
    assert_eq!(nonce,decoded);
}

fn test_panel(){
    //panel::App::panel_login();
    panel::panel_main();
}