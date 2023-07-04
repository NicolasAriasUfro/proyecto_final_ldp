struct entrada{
    id:u64,
    titulo:String,
    nombre_usuario:Option<String>,
    contrasena:String,
    nonce:[u8;12],
    fecha_creacion:u64,
    url:Option<String>
}

impl entrada{
    fn new_desde_bd(id:u64,titulo:String,nombre_usuario:Option<String>,contrasena:String,nonce:[u8;12],fecha_creacion:u64,url:Option<String>)->Self{
        Self{id,titulo,nombre_usuario,contrasena,nonce,fecha_creacion,url}
    }

    fn new_creado(titulo:String,nombre_usuario:Option<String>,contrasena:String,nonce:[u8;12],fecha_creacion:u64,url:Option<String>)->Self{
        Self{id:0,titulo,nombre_usuario,contrasena,nonce,fecha_creacion,url}
    }

}
