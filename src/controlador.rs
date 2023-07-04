

pub fn password_validator(password: &str) -> bool {
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

pub fn get_password(){
    // TODO: add function to get password

}



