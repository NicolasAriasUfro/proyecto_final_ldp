use std::slice::ChunksExact;

use dialoguer::Password;

use crate::controlador::*;
pub struct App {
}

impl App {
    pub fn panel_login() {
        let Password = password_validator();

        let password = Password::new()
            .with_prompt("Ingrese su contraseña:")
            .interact()
            .expect("Error al solicitar la contraseña, contraseña incorrecta");
    }

    // panel para crear la contraseña si la base de datos no existe, luego almacenarla en la base de datos y finalmente
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
        let default_choice = false;
        todo!();
        //el panel main debe manejar alguna estructura de datos que contenga las cuentas, este obviamente debe ser poblado mediante instrucciones
        //SELECT , luego se recorre toda la estructura de datos y se imprimen las cuentas con su numero
        // el panel en la zona inferior debe manejar los siguientes comandos:
        // debe haber un limite de caracteres por mostrar en el panel, con 8 basta

        // FUNCIONALIDADES:
        // 1. Crear una nueva cuenta
        // 2. Borrar una cuenta
        // 3. Listar todas las cuentas por nombre/fecha (cambia con cada seleccion)
        // 4. Salir
        // 5. poder elegir una cuenta para mostrar su información

        /*
        Ejemplo de vista del panel
        CUENTAS:
        1 titulo   user         url                       password 
        2 google   juan         https://www.google.com    ********
        3          a@gmail.com                            **************

        crear nueva cuenta: q borrar una cuenta:w salir: e
         */
        


    }

    pub fn sort_by_date() {
        todo!();
    }
    pub fn sort_by_title() {
        todo!();
    }

    // este panel debe mostrar el contenido de la contraseña una vez se escoja una
    pub fn panel_contenido() {
        todo!()

        //esta funcion muestra el contenido de la cuenta
        //debe permitir volver atras, 
        //formato de ejemplo:
        //cuenta
        //Title:  // user  // url // password(*******)
        //volver atras: A mostrar contraseña: D Copiar contraseña: C  

        /*
        Ejemplo de muestra de contraseña
        //Title:  // user  // url // password(12345678)
        //volver atras: A Ocultar contraseña: D Copiar contraseña: C  
         */

    }


    
}
