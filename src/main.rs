use crate::manipular_info::crypto_base::{hash_llave, derivar_llave};
mod manipular_info;

mod controller_sql;
mod clipboard_generic;
mod panel;
mod ejemplo_sql;


fn main() {
    //panel::panel_login();
    controller::set_database();
    let clave=b"contra";
    let mut sal=[0u8;16];
    let mut llave=[0u8;32];
    derivar_llave(clave, &mut llave, &mut sal);
    let mut sal_loging=sal.clone();
    sal_loging[2]=0u8;
    assert_ne!(hash_llave(&llave, &sal_loging),llave);

}
