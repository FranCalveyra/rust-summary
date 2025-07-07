# Variables
Las variables en `Rust` son **inmutables** por default, aunque se pueden volver mutables con el keyword `mut` en su declaración \
Aun así, se pueden re-declarar algunas variables inmutables a través del concepto de ``Shadowing``, que consta de pisar el valor de una variable dentro de un [scope]() diferente

``Nota importante``: Hay **_inferencia de tipos_**, aunque también soporta la declaración de tipos
```rust
//Inmutabilidad
let mut x = 5;
println!("X has value: {}", x);
x = 6;
println!("Now X has value: {x}");

//Shadowing
let x = -5;
println!("The value of x is: {x}");
let x: u32 = 6;
{
    let x = x * 2;
    println!("The value of x in the inner scope is: {x}");
}
println!("The value of x is: {x}");
```

# Tipos escalares
- Signed Integer: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (architecture default)
- Unsigned Integer: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
- Floating Point: `f32`, `f64`
- Boolean: `bool`
- Character: `char` (Unicode scalar, 32 bits)

# Tipos compuestos

## Tuplas
Se pueden declarar tuplas de los tipos que sean, a menos que se les especifique el tipo
````rust
// Esto es una tupla de tipos inferidos
let tup = (500, 1.4114, "hola") // Nótese que puedo seguir agregando cosas en la tupla si sigo escribiendo, y se le inferirá el tipo
// Esto es una tupla de tipos declarados
let triple: (i32, f32, &str) = (500, 1.4114, "hola") // Acá me tiraría un error de compilación si hago lo descrito en el caso anterior, porque está 
// limitado a 3 elementos que ADEMÁS respeten los tipos descritos a la izquierda
````
Los métodos `void` devuelven la tupla vacía ``Unit = ()``

## Arrays / Listas
Mismo caso que con las tuplas, pueden inferir tipos, pero tienen que ser todos los elementos del mismo tipo. \
Se pueden declarar el tipo y la capacidad del array, así como "rellenarlo" con una cierta cantidad de elementos.

````rust
let a = [1, 5, -3]; // Primer caso
let b: [i64; 3]  = [a[1], 10, a[0]]; // Segundo caso
let c = [0; 10]; // Tercer caso
````

# Control Flow

## Ifs
Los ``ifs`` son como en cualquier otro lenguaje, solo que no hace falta usar paréntesis para encerrar la condición.

No existe el operador ternario ``?``, en cuyo reemplazo surge el `if as an expression`. Nótese el siguiente ejemplo:
```rust
let number = 3;
let x = if number >= 0 { number } else { -number }; 
```
Nota: dependiendo del caso se puede dejar el valor de retorno como última línea o como ``return statement`` en caso de querer devolver algo, pero debe estar sin `;`. \
En caso contrario, se usa ``return {lo que se vaya a devolver} ;``, depende cómo se quiera escribir. En definitiva es indistinto.

## Switch
La keyword en este lenguaje para un switch es ``match``. Se puede dejar el caso default con un `_`.
````rust
let number = 5;
let s = match number {
    5 => "Five",
    7 => "Seven",
    _ => "Other"
};
````

# Loops
## While
El while se escribe exactamente igual que en la mayoría de lenguajes convencionales. No es necesario proporcionar un ejemplo.

## Loop
Aparece un keyword ``loop``, que es análogo a un `do {...} while (condición)`. La única "cuestión rara" es que requiere una condición de corte que llame al keyword `break` dentro del hilo de ejecución

````rust
let mut number = 2;

loop {
    println!("Before: {number}");
    if number == 0 {break; } // Nótese la condición de corte
    number -= 1; 
    println!("After: {number}");
}
````

## For
For loop común y corriente, no es muy distinto a los lenguajes convencionales.
Se puede iterar sobre una estructura o sobre un rango, alocando una variable temporal.
````rust
let a = [10, 20, 30, 40];

for v in a {
    println!("{v}");
}

for i in 0..4 {
    println!("a[{}] = {} ", i, a[i]); 
}
````

# Structs - Introduction
Al ser un lenguaje basado en C, no existe el concepto de objetos o clases, sino que se da el concepto de ``Structs``, que son parecidos.
````rust
#[derive(Debug)] // Esto es simplemente un macro para derivar comportamiento de que sea debuggeable (imprimibible). Más tarde hablaremos de macros
struct Student {
    id: i32,
    name: String,
    active: bool
}
let mut user1 = Student { id: 100, name: String::from("Curly"), active: true};
user1.active = false; // Nótese que esto no se podría si user1 no tuviera el keyword mut
println!("User = {user1:?}");
````

Se pueden declarar structs de tipo ``Tupla``, cuyos tipos se declaran en la tupla, y se acceden los atributos con `struct.índice`
````rust
struct Point(i64, i64);
let p1 = Point(0,1);
let x = p1.0; // Se obtiene el primer atributo
let y = p1.1; // Se obtiene el segundo atributo
````

# Enums
Parecidos a los de Java, la idea es la misma. Solamente cambia la sintaxis.
````rust
#[derive(Debug)]
enum Color { Red, Green, Blue }
let r = Color::Red;

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64)
}

Shape::Circle(10.5)
````

# Option type
Rust no maneja el concepto de ``null references``, por lo que introduce (como el ``Maybe`` en Haskell), el concepto de ``Option``, que a su vez es un enum.
````rust
// Así está declarado por default en la docu de Rust
enum Option<T> {
    None,
    Some(T)
}
let mut a = Some(10);
let v = a.unwrap() + 12; // Unwrap es una operación "unsafe" (aunque no es como el concepto de unsafe que vamos a ver más adelante) 
// ya que puede reventar al recibir un None, por lo que se usa, generalmente, el ejemplo de abajo
let x: Option<i32> = None;
let b = x.unwrap_or(5) + 10 // Unwrap or proporciona un valor default en caso de recibir un None.
````
Se pueden aplicar funciones a los valores internos de aquellos tipos que wrappean valores, como si fuese un Functor en Haskell.
````rust
fn divide(a: f64, b: f64) -> Option<f64> {
    return if b == 0.0 { None } else { Some(a / b) }
}
let v =  divide(15.0, 0.0).map(|x| x + 10.0); // Esto va a reventar, porque estás dividiendo por 0
let r = divide(15.0, 5.0).map(|x| x + 10.0); // Esto va a dar Some(13.0), porque puede hacer la división bien, 
// y el map le suma 10 al valor interno
````

## Result Type
Es para manejar errores, es análogo al ``Either`` de Haskell.
````rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[derive(Debug)]
enum MathError { // Esto es sólo la declaración del tipo de error MathError que existe en Rust
    DivideByZero,
    SqrtOfNegative
}

fn divide(a: f64, b: f64) -> Result<f64, MathError> {
    return if b == 0.0 { Err(MathError::DivideByZero) } else { Ok(a / b) }
}
divide(10.0, 0.0).unwrap_or(0.0) // Esto va a devolver 0.0 porque está queriendo dividir por 0
````

# Pattern Matching
## Match
````rust
let x = 10.0;

for y in [0.0, 5.0] {
    print!("{x} divide by {y} = ");
    match divide(x, y) {
       Ok(n) => println!("{n}"),
       Err(e) => println!("Error {e:?}") 
    }
}
````

## Let
````rust
let x = 10.0;

for y in [0.0, 5.0] {
    if let Ok(n) = divide(x,y) {
        println!("{x} divide by {y} = {n}");
    }
}
````