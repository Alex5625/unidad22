use std::fs::File;
use std::io::{ BufRead , BufReader , Error };

fn main() {

    match leer_archivo("notas.txt") {
        Ok(_) => println!("termino el codigo ctm"),
        Err(_) => panic!(),
    }
    
}



fn leer_archivo ( nombre_archivo : &str ) -> Result <() , Error> {
    let file = File::open ( nombre_archivo )?;
    let reader = BufReader::new ( file ) ;
    //ESTO ES POR CADA LINEA
    for line in reader.lines () {
        let l = match line {
            Ok ( text ) => text,
                Err ( err ) => panic!(),

        };

        let mut suma : f32 = 0.0;
        let mut nombre = String::new();
        //separar los datos con la distincion en este caso del ":"
        let split = l.split(":");

        
        // la funcion .split() TIRA UN ITERADOR (para recorrer).
        for linea in split {
            // dentro de la linea, si no es un f32 que se peued hacer parse(), pasa a la variable nombre.
            match linea.parse::<f32>() {
                Ok(num) => suma += num,
                Err(_) => nombre = linea.to_string(),
            }
        }

        let promedio :f32 = suma  / 6 as f32;
        if promedio <= 4.0 {
            println!("El alumno {} reprobó con una nota de {}\n",nombre,promedio);
        } else {
            println!("El alumno {} aprobó con una nota de {}\n",nombre,promedio);
        }

    }

    return Ok (())
}