use aes_gcm_siv::{aead::{generic_array::GenericArray, Aead}, Aes256GcmSiv, KeyInit};
use argon2::{Argon2,  password_hash::SaltString, PasswordHasher};
use rand_chacha::{ChaCha20Rng, rand_core::{SeedableRng, RngCore}, ChaChaRng};

pub struct Criptografia{
    cifrador:Aes256GcmSiv,
}

impl Criptografia{
    pub fn new(llave:&[u8;32])->Self{
        let llave=GenericArray::from_slice(llave);
        Self { cifrador: Aes256GcmSiv::new(llave) }
    }

    

    pub fn cifrar_bytes(&self,nonce:&[u8;12],plaintext:&[u8])->Option<Vec<u8>>{
        let nonce=GenericArray::from_slice(nonce);
        match self.cifrador.encrypt(nonce, plaintext){
            Ok(cifrado)=>Some(cifrado),
            Err(_)=>None
        }
    }



    pub fn descifrar_bytes(&self,nonce:&[u8;12],ciphertext:&[u8])->Option<Vec<u8>>{
        let nonce=GenericArray::from_slice(nonce);
        match self.cifrador.decrypt(nonce, ciphertext) {
            Ok(descifrado)=>Some(descifrado),
            Err(_)=>None
            
        }
    
    }



}

pub fn derivar_llave(clave:&String,salt:&[u8;16])->[u8;32]{
    let sha_key=sha256::digest(clave.clone());
    let mut output_buffer=[0u8;32];
    Argon2::default().hash_password_into(sha_key.as_bytes(), salt, &mut output_buffer).unwrap();
    output_buffer

}

pub fn crear_sal()->[u8;16]{
    let salt_out:[u8;16];
    let raw_salt=SaltString::generate(&mut ChaCha20Rng::from_entropy());
    let mut salt=[0u8;16];
    raw_salt.decode_b64(&mut salt).unwrap();
    salt_out=salt.try_into().expect("no se pudo crear sal");
    salt_out
}

pub fn crear_llave(clave:&String,buffer_salida_llave:&mut [u8;32],salida_sal:&mut [u8;16]){
    let sha_key=sha256::digest(clave.clone());
    let salt=crear_sal();
    Argon2::default().hash_password_into(sha_key.as_bytes(), &salt, buffer_salida_llave).unwrap();
    *salida_sal=salt;
}

pub fn hash_contra_maestra(contra:&[u8],sal:&[u8;16])->Vec<u8>{
        let sal=SaltString::encode_b64(sal).unwrap(); //esto no deberia ser?
        let hash_llave=Argon2::default().hash_password(contra, &sal);
        match hash_llave {
            Ok(hash_contra)=>hash_contra.hash.unwrap().as_bytes().to_owned(), //esto es muy jocoso.
            Err(_)=>panic!("no se pudo calcular hash de la contraseña maestra...")
            
        }

}

pub fn crear_nonce()->[u8;12]{
    let mut rand=ChaChaRng::from_entropy();
    let mut nonce=[0u8;12];
    for i in 0..12{
        let random_num=rand.next_u32() as u8;
        //print!("{}\\",random_num);
        nonce[i]=random_num;
    };

    return nonce;
}


pub fn hash_entregado_valido(hash_entregado:&[u8],hash_almacenado:&[u8])->bool{
    hash_entregado==hash_almacenado
}