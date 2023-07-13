use std::{env, process::exit, hash};

use base64::{engine::general_purpose, Engine};
use controlador::manipular_info::crypto_base::crear_nonce;
use controller_sql::{agregar_master, recuperar_datos_master};

use crate::{controlador::manipular_info::{crypto_base::{hash_contra_maestra, crear_llave, derivar_llave}, info_almacenada::{recrear_nonce}, iniciar_nueva_base_de_datos}, controller_sql::set_database};
use crate::controlador::manipular_info::info_almacenada::Entrada;


mod clipboard_generic;
mod panel;
mod controller_sql;
mod controlador;

fn main() {
    controlador::load_app();
}