mod crypto_base;

#[cfg_attr(target_family = "unix", path = "clipboard_generic.rs")]
#[cfg_attr(target_family ="windows", path = "clipboard.rs")]
mod clipboard;
mod panel;
mod sql_playground;



fn main() {
    //panel::panel_login();
    
    

}
