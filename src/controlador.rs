use dialoguer::Password;

use self::manipular_info::crypto_base::hash_contra_maestra;


pub mod manipular_info;
pub fn password_validator(password: &str) {//-> bool {
    let password: String = Password::new()
    .with_prompt("Enter password")
    .validate_with(|input: &String| -> Result<(), &str> {
        if input.len() > 8 {
            Ok(())
        } else {
            Err("Password must be longer than 8")
        }
    })
    .interact()
    .unwrap();
}

pub fn validar_llave_maestra(contra_recibida:&[u8],sal:&[u8;16])->bool{
    //recuperar de la base de datos
    let hash_guardado:String=String::new();
    let hash_guardado=hash_guardado.as_bytes().to_owned();
    let hash_calculado=hash_contra_maestra(contra_recibida, sal);
    hash_guardado==hash_calculado
}

pub fn get_password(){
    // TODO: add function to get password

}



