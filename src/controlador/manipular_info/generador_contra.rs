use rand::Rng;
use rand_chacha::{rand_core::{SeedableRng}, ChaChaRng};


type Tipo=TipoContra;
pub enum TipoContra{
    ALFABETICO,
    NUMERICO,
    ALFANUMERICO,
    ALFANUMERICOEXT
}

pub fn generar_contra(tipo:TipoContra,longitud:u16)->String{
    let mut caracteres_usados=String::new();
    let mut contra_generada=String::new();
    iniciar_caracteres_usados(tipo,&mut caracteres_usados);
    //let rng=rand::thread_rng();
    let mut rng=ChaChaRng::from_entropy();
    for _i in 0..longitud {
        let caracteres_usados:Vec<char>=caracteres_usados.chars().collect();
        let posicion_seleccionada=rng.gen_range(0..caracteres_usados.len());
        contra_generada.push(caracteres_usados[posicion_seleccionada]);
    };

    contra_generada
    

}

fn iniciar_caracteres_usados(tipo:TipoContra,buffer_usados:&mut String){
    let letras_minusculas="abcdefghijklmnopqrstuvwxyz";
    let letras_mayusculas=letras_minusculas.to_uppercase();
    let letras_mayusculas=letras_mayusculas.as_str();
    let numeros="1234567890";
    let extra="/,.#%";

    match tipo {
        Tipo::ALFABETICO=>
        {
            buffer_usados.push_str(letras_minusculas);
            buffer_usados.push_str(letras_mayusculas);
        },
        TipoContra::NUMERICO=>buffer_usados.push_str(numeros),
        TipoContra::ALFANUMERICO=>{
            buffer_usados.push_str(letras_minusculas);
            buffer_usados.push_str(letras_mayusculas);
            buffer_usados.push_str(numeros);
        },
        TipoContra::ALFANUMERICOEXT=>{
            buffer_usados.push_str(letras_minusculas);
            buffer_usados.push_str(letras_mayusculas);
            buffer_usados.push_str(numeros);
            buffer_usados.push_str(extra);
        }
        

        
    }

}