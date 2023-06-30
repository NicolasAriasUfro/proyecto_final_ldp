use dialoguer::Confirm;

pub fn panel_login() {

    let confirmed = Confirm::new()
    .with_prompt("Are you sure you want to proceed?")
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

