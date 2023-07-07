use crate::manipular_info::crypto_base::{hash_llave, derivar_llave};

mod manipular_info;


mod clipboard_generic;
mod panel;
mod ejemplo_sql;


fn main() {
    //panel::panel_login();
    ejemplo_sql::test_3();
    let clave="contra".to_owned();
    let mut sal=[0u8;16];
    let mut llave=[0u8;32];
    derivar_llave(&clave, &mut llave, &mut sal);
    let mut sal_loging=sal.clone();
    //sal_loging[2]=0u8;
    println!("key={:?}\n key_hash={:?}",llave,hash_llave(&llave, &sal));
    assert_ne!(hash_llave(&llave, &sal),llave);

}
