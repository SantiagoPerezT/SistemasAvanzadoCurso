fn main() {

    let x = 5; 

    println!("Hello, world!");

    //x=10; <---esto no es valido

    //let mut y = 10;
    //y = 20;
    let x = 5;
    {
        let x = x + 1; // nueva variable
        println!("E valor de x es: {}",x);
    }
    println!("E valor de x es: {}",x);
    // VARIABLES //

    let entero: i32 = 42;
    let flotante: f64 = 3.1416;
    let booleano: bool = true;
    let caracter: char = 'a';
    //tupla
    let firulais: (i32, f64, char) = (43, 4.1416, 'b'); 
    //array
    let arreglo: [i32;3] = [1, 2, 3];

    println!("Tupla(firulais) forma 1: {:?}",firulais);

    println!("Tupla(firulais) forma 2: ({}, {}, {})",firulais.0,firulais.1,firulais.2);

}