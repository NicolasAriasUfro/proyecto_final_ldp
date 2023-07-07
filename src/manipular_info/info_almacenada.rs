pub struct Entrada{
    id:u64,
    titulo:String,
    nombre_usuario:Option<String>,
    contrasena:String,
    nonce:[u8;12],
    fecha_creacion:u64,
    url:Option<String>
}

impl Entrada{
    pub fn new_desde_bd(id:u64,titulo:String,nombre_usuario:Option<String>,contrasena:String,nonce:[u8;12],fecha_creacion:u64,url:Option<String>)->Self{
        Self{id,titulo,nombre_usuario,contrasena,nonce,fecha_creacion,url}
    }

    pub fn new_creado(titulo:String,nombre_usuario:Option<String>,contrasena:String,nonce:[u8;12],fecha_creacion:u64,url:Option<String>)->Self{
        Self{id:0,titulo,nombre_usuario,contrasena,nonce,fecha_creacion,url}
    }

    pub fn set_id(&mut self,nueva_id:u64){
        self.id=nueva_id;
    }
    pub fn set_titulo(&mut self,nuevo_titulo:String)->bool{
        if nuevo_titulo.trim()==""{return false;}
        self.titulo=nuevo_titulo;
        true
    }

    pub fn set_contra(&mut self,nueva_contra:String)->bool{
        if nueva_contra.trim()==""{return false;}
        self.contrasena=nueva_contra;
        true
    }

    

}
