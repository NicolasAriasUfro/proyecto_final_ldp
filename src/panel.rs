use dialoguer::Password;

use crate::controlador::password_validator;
struct App {
    
}

impl App {
    pub fn panel_login() {
        let Password = password_validator();

        let password = Password::new()
            .with_prompt("Ingrese su contraseña:")
            .interact()
            .expect("Error al solicitar la contraseña, contraseña incorrecta");
    }

    // panel para crear la contraseña si no existe, luego almacenarla en la base de datos y finalmente
    pub fn panel_register() {
        // ingrese la contraseña maestra, esta debe tener como mínimo 8 caracteres.
        // ingrese la contraseña nuevamente.
        // 
        // si esta ok println!("contraseña creada con exito");
        // se dirige al panel_main

        todo!()
    }

    //panel principal de la aplicación, muestra todas las cuentas almacenadas con su titulo (si hay), user, url(si hay) y password
    pub fn panel_main() {
        todo!();
        //el panel main debe manejar alguna estructura de datos que contenga las cuentas, este obviamente debe ser poblado mediante instrucciones
        //SELECT , luego se recorre toda la estructura de datos y se imprimen las cuentas con su numero
        // el panel en la zona inferior debe manejar los siguientes comandos:

        // 1. Crear una nueva cuenta
        // 2. Borrar una cuenta
        // 3. Modificar una cuenta
        // 4. Listar todas las cuentas
        // 5. Salir

    }

    // este panel debe mostrar el contenido de la contraseña una vez se escoja una
    pub fn panel_contenido() {
        todo!()
        //formato de ejemplo:
        //cuenta
        //Title:  // user  // url // password(*******)
        //volver atras: 
        //mostrar contraseña
    }

    pub fn copiar_password() {
       //debe copiar la contraseña al portapapeles
        
    }

    pub fn 
}
