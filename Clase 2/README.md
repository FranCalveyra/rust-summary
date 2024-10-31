# References
Análogos a los pointers de C. Son prácticamente lo mismo. Si se usa el ``&``, se obtiene la dirección de memoria de lo que se está referenciando.
Si se usa el ``*``, se dereferencia el puntero y se obtiene el valor que almacena.
````rust
{
    let n = 100;
    let pn = &n;  // pn contains the memory address of n
    println!("n = {}; address of n = {:p}, value = {}", n, pn, *pn);
}

// Esto printea: n = 100; address of n = 0x16bdd575c, value = 100
````
El operador ``*`` fuerza la dereferencia. Usualmente, el compilador de Rust sabe que la variable es un puntero en sí, y en caso de querer printearlo printea el valor que almacena

## Mutabilidad de las referencias
En cuanto a este tema, pueden existir ``N`` referencias **INMUTABLES**, pero únicamente 1 **mutable**, la cual remueve el `ownership` de las inmutables.
Es un caso, o el otro. No pueden existir los 2 al mismo tiempo.

Nota: hablaremos de ownership más adelante.

## Pass by value
```rust
fn swap(mut a: i32, mut b: i32) {
let tmp = a;
a = b;
b = tmp;
}

let x = 10;
let y = 20;
println!("Before: {x}, {y}");
swap(x, y);
println!("After: {x}, {y}");

Output: 
Before: 10, 20
After: 10, 20
```

No funciona porque los valores de x e y se copian en a y b. El compilador de Rust suele dar warnings en estos casos.

Para solucionarlo, se pasan por _**referencia**_:

````rust
fn swap2(a: &mut i32, b: &mut i32) { // Nótese que se pasan los punteros
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

let mut x = 10;
let mut y = 20;
println!("Before: {x}, {y}");
swap2(&mut x, &mut y);
println!("After: {x}, {y}");

Output:
Before: 10, 20
After: 20, 10
````

# Structs - Continuación
## Funciones sobre Structs
Se pueden tomar funciones que reciban structs como tipo de dato, pero se debe pasar un pointer al Struct en caso de querer usarlo como tipo de dato, por cuestiones de ownership
````rust
struct Rectangle {
    width: u32,
    height: u32,
}
let r = Rectangle { width: 10, height: 20};

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

area(&r) // Esto es inmutable
````
## Métodos
Si se quisieran escribir métodos para un struct como si fuesen métodos de un objeto Java, se escribe de la siguiente manera:
````rust
// Tomando el ejemplo anterior del área
impl Rectangle {            // <-- I'm going to implement some methods for Rectangle
    fn area(&self) -> u32 { // <-- &self is a reference to a Rectangle
        self.width * self.height
    }
}
// Ahora es válido hacer lo siguiente:
r.area() // Siendo r el rectángulo definido en el scope anterior
````

### Funciones que mutan un struct
```rust
#[derive(Debug)]
struct Item { id: i32, stock: i32}

// Add some q to the Item
fn add_stock(item: &mut Item, q: i32) {
    item.stock += q
}
// a is declared as mutable so I can modify it
let mut a = Item { id: 1, stock: 100};
println!("{:?}", a);

// I pass a mutable reference to a 
add_stock(&mut a, 20);

println!("{:?}", a);
```

### Métodos que mutan un struct
````rust
#[derive(Debug)]
struct Item { id: i32, stock: i32}

impl Item {
    fn add_stock(&mut self, q: i32) { // self is a mutable reference
        self.stock += q
    }
}

let mut a = Item { id: 1, stock: 100};
println!("{:?}", a);

a.add_stock(20); // Rust knows that it needs to pass a mutable reference

println!("{:?}", a);
````

# Traits
La idea es la misma que las interfaces de Java, y son muy similares a las ``type classes`` de Haskell.

Exponen un contrato que tiene que seguir el struct que implementa dicho trait.
Véase un ejemplo:
````rust
trait Named {
    fn first_name(&self) -> &String;
    fn last_name(&self) -> &String;
    fn full_name(&self) -> String { format!("{} {}", self.first_name(), self.last_name()) }
}

struct Person(String, String);

impl Named for Person { 
    fn first_name(&self) -> &String { &self.0 }
    fn last_name(&self) -> &String  { &self.1 }
}

struct Student { name: String, last_name: String }

impl Named for Student {
    fn first_name(&self) -> &String { &self.name }
    fn last_name(&self) -> &String  { &self.last_name }
}
````
Similar a la herencia de interfaces en Java, existe herencia de traits.\
Supongamos que A es un trait. B es otro trait que hereda de A, y C hereda de B.
Si yo quiero hacer un struct que implemente C, debo implementar los métodos de A y B también. Véase el siguiente ejemplo:
````rust
trait Person {
    fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}
````

# Generics
Rust implementa Generics al usar ``monomorfización`` del código en tiempo de compilación. Genera el código para las implementaciones necesarias. \
Definición de ``monomorphization`` de la docu de Rust:

"Rust takes a different approach: it monomorphizes all generic types.
This means that compiler stamps out a different copy of the code of a generic function for each concrete type needed.
For example, if I use a ``Vec<u64>`` and a ``Vec<String>`` in my code, then the generated binary will have two copies of the generated code for ``Vec``: one for ``Vec<u64>`` and another for ``Vec<String>``.
The result is fast programs, but it comes at the cost of compile time (creating all those copies can take a while) and binary size (all those copies might take a lot of space).

Monomorphization is the first step in the backend of the Rust compiler."

## Trait bounds
Si yo genero el siguiente código:
````rust
fn wrong_max<T>(list: &[T]) -> &T { 
    let mut max = &list[0];

    for item in list {
        if item > max { max = item }
    }
    max
}
println!("{}", wrong_max(&[4, 1, 2, 3]));
````
Esto no compila. Porque no está especificado que el tipo ``T`` sea comparable usando `>`. El compilador de Rust mismo te tira una ayuda, diciéndote que especifiques que implementa ``PartialOrd``.

Literalmente todo el fix de ese código es cambiar la firma de la siguiente manera: ``fn max<T: Ord>(list: &[T]) -> &T``
### Complex bounds
En caso de querer hacer que implemente más de un trait o de dejar los traits explícitos en el código, podemos usar el keyword ``where``.
````rust
fn max1<T: Ord>(list: &[T]) -> &T { todo!() }

fn max2<T>(list: &[T]) -> &T
    where T : Ord 
{ todo!() }
````
Este keyword suele ser útil al querer escribir ``complex bounds``, es decir, definir más de un bound para un Generic.
````rust
fn example1<T: Ord + Clone, U: Eq + Copy>(t: &T, u: &U) -> i32 
{ 10 }

fn example2<T, U>(t: &T, u: &U) -> i32
where T: Ord + Clone,
      U: Eq + Copy

{ 10 }
````
### &impl
Otra forma de escribirlo es con ``&impl``
````rust
trait Shape {  fn area(&self) -> f64;}

fn print_area1<T: Shape>(x: &T) {
    println!("{}", x.area())
}

fn print_area2(x: &impl Shape) {
    println!("{}", x.area())
}
````