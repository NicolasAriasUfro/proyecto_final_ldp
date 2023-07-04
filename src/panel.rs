mod conexionbd.rs;
use dialoguer::Confirm;


pub fn panel_login() {
    // llamada a la base  de datos
    let confirmed = Confirm::new()
    .with_prompt("Bienvenido")
    .interact()
    .unwrap();

if confirmed {
    println!("Proceeding...");
    // Add your code here to handle the confirmed action
} else {
    println!("Cancelled.");
    // Add your code here to handle the cancelled action
}
}

