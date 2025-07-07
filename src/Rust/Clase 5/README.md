# Smart Pointers
Son estructuras de datos que funcionan exactamente igual que los punteros que venimos manejando, pero que tienen funcionalidades extra.
## ¿Por qué los usaríamos?
- Memory safety sin garbage collector
- Cleanup automático a través de RAII (Resource Acquisition Is Initialization)
- Administración de Ownership y Borrowing de datos alocados en el Heap

Varios de los Smart Pointers que vamos a usar vienen de la librería estándar de Rust,
particularmente ``Box`` y `Rc`.

Para sorpresa de muchos, ``String`` y `Vec<T>` son Smart Pointers.

Podemos definir los nuestros propios
## Box
````rust
{
    let n = 101; // n goes into the stack
    let m = 102; // m also

    println!("&n: {:p}, &m: {:p}", &n, &m);
    
    let pn = &n;            // Pointer to n. So it points to the stack
    let bn = Box::new(m);   // bn points to data in the heap
    
    println!("pn: {:p}, *pn: {}", pn, *pn);
    println!("bn: {:p}, *bn: {}", bn, *bn);
} // <- Box is Drop here, and memory in the heap is released
````

### Box for recursive types
````rust
enum List {
    Cons(i32, List),
    Nil,
}
// Esto ni siquiera compila porque List tiene tamaño infinito, es un tipo recursivo
````
Esto se arregla usando un Box:
````rust
enum List {
    Cons(i32, Box<List>),
    Nil, 
}

let list = Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));

// O mejor:
fn cons(value: i32, tail: Box<List>) -> Box<List> { Box::new(Cons(value, tail)) }
let nil: Box<List> = Box::new(Nil);

let list = cons(1, cons(2, cons(3, nil)));
````
## ``Rc<T>``
Rc viene de Reference counter, el cual permite ownership múltiple (siempre y cuando se hable de referencias inmutables), manteniendo la cuenta de la cantidad de referencias a un dato.
Limpia el valor una vez el número de referencias llega a 0.

````rust
// Ejemplo
struct Person { name: String }
struct Account {
    number: i32, owner: Person
}
let p = Person { name: "John".to_string() };
let ac1 = Account { number: 1, owner: p };
let ac2 = Account { number: 2, owner: p };

println!("{} & {}", ac1.owner.name, ac2.owner.name);
// Esto no compila porque se mueve el ownership de p a ac1

// Corrección:
use std::rc::Rc;

struct Account {
    number: i32, owner: Rc<Person>
}
{
let p = Person { name: "John".to_string() };
let rp = Rc::new(p);
let ac1 = Account { number: 1, owner: rp.clone() };
let ac2 = Account { number: 2, owner: rp.clone() };


println!("{} & {}", ac1.owner.name, ac2.owner.name);
}
````

### Print Reference Count
````rust
fn print_count(p: &Rc<Person>) { println!("{}", Rc::strong_count(&p)) }

let rp = Rc::new(Person { name: "John".to_string() });
print_count(&rp);
{ 
    let ac1 = Account { number: 1, owner: rp.clone() };
    print_count(&rp);
    {
        let ac2 = Account { number: 2, owner: rp.clone() };
        print_count(&rp);
    }
    print_count(&rp);
}
print_count(&rp);
````
Pero, ¿qué es Strong Count y Weak Count?
- Strong Count es de una referencia fuerte, que es un ``Rc`` que se crea por default. Este tipo de referencia tiene ownership por sobre el dato, e importa y cuenta en cuanto a la cantidad de referencias
- Weak Count es todo lo contrario, no tiene ownership sobre el dato y no afecta a la cantidad de referencias. Este objeto se va a "droppear" o desalocar en cuanto las referencias fuertes sean 0 (es decir, no haya ownership desde ningún lado)

``Nota``: es muy dudoso que pregunten esto porque Emilio lo mencionó muy por arriba, pero lo incluyo por las dudas

## Arc (Atomic Reference Counted)
- Es como ``Rc``, pero es seguro de compartir entre threads
- Permite ownership compartido entre múltiples threads
- Es más caro debido a las cuestiones de sincronización de threads

En resumen, es un Rc para referencias concurrentes entre hilos

## RefCell: reference to a ``mutable`` cell
Es un smart pointer que permite mutabilidad ``interior``
- Se puede mutar el dato interno, el cual almacena
- Aun cuando el ``RefCell`` mismo es accedido a través de una referencia inmutable
- Fuerza las reglas de Borrowing de Rust en ``run-time`` en lugar de en ``compile-time``

### RefCell dentro de Rc
````rust
let person = Rc::new(Person {name: "John".to_string()});
person.name = "Sean".to_string();
// Esto revienta porque Rc es una referencia INMUTABLE, 
// no se debe poder afectar al valor original
````

En su lugar, se corrige de la siguiente manera:
````rust
let person = Rc::new(RefCell::new(Person {name: "John".to_string()}));
person.borrow_mut().name = "Sean".to_string();
println!("{}", person.borrow().name);
````

# Lifetimes
Son una forma de medir por cuánto tiempo las referencias son válidas. Aseguran `memory safety` al prevenir `dangling references`, que son referencias que no viven lo suficiente o que quedan "colgadas"

Cuando el compilador no las puede inferir automáticamente, es necesario usar ``annotations`` para explicitarlas

¿Por qué son importantes?
- El sistema de ownership garantiza ``memory safety``, y los lifetimes ayudan a reforzar dichas garantías
- Previene el uso de referencias que pueden apuntar a memoria inválida o libre (a la nada misma)

No hace falta que proporcione ejemplos de lifetimes inválidos porque ya los vimos a lo largo de las clases anteriores

````rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

let string1 = String::from("abcd");
let string2 = "xyz";

let result = longest(&string1, string2);
println!("The longest string is '{}''", result);
// esto explota porque no se sabe el lifetime que tiene que tener el valor de retorno
````
Fix:
````rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // el 'a es para marcar el lifetime, todos tienen que tener el mismo en este caso
    if x.len() > y.len() { x } else { y }
}
````

El lifetime es imperativo cuando los structs contienen referencias.
````rust
struct Name {
    first_name: &str, last_name: &str
} // Esto explota porque no se sabe el lifetime de ninguno de los 2 atributos

//Fix:
struct Name<'a> { first_name: &'a str, last_name: &str }
````

## Static lifetimes
El lifetime ``'static`` es uno especial para denotar que la referencia vive durante la duración de TODO el programa.

Es similar a una referencia "global"

````rust
// Los Strings literales tienen tiempo de vida estático porque están hardcodeados
let s: &'static str = "I live for the whole program!";
// Because John and Smith have 'static lifetime a Name
// constructed from them also has an 'static lifetime:
fn john_smith() -> &'static Name {
    &Name { first_name: "John", last_name: "Smith"}
} 
````

# Dynamic vs. Static Polymorphism
Vamos a usar los siguientes structs para los ejemplos a continuación:
````rust
trait Shape {  fn area(&self) -> f64;}

struct Circle(f64);
struct Square(f64);

impl Shape for Circle {
    fn area(&self) -> f64 { 3.14 * self.0 * self.0 }
}
impl Shape for Square {
    fn area(&self) -> f64 { self.0 * self.0 }
}
````

La siguiente es una función polimórfica:
````rust
fn print_area<T: Shape>(x: &T) {
    println!("{}", x.area())
}

print_area(&Square(4.0));
print_area(&Circle(4.0));
````
Hasta ahora no hay problema, ¿no? Porque ambos son Shapes y se aplica el monomorfismo del que hablamos antes.

Y la siguiente es también una función cuyo polimorfismo es dinámico:
````rust
fn sum_areas<T:Shape>(ts: Vec<T>) -> f64 
{
    ts.iter().map(|x| x.area()).sum()
}

let shapes = vec!(Square(2.0), Square(1.0));
let s = sum_areas(shapes);
println!("{s}")
````
Acá sigue sin pasar nada por el monomorfismo, ambos son Square

````rust
let shapes = vec!(Square(2.0), Circle(1.0));   
let s = sum_areas(shapes);
println!("{s}");
````
¿Y ahora? Esto explota porque esperaba un Square en lugar de un Circle. El vector asume su tipo a partir del tipo del primer elemento

Si lo quisiéramos declarar como un ``Vec<Shape>``, esto nos dará un error diciendo que no se conoce el tamaño del vector en tiempo de compilación.
Entonces, ¿cómo lo solucionamos?

¡Fácil! Con **referencias**.
````rust
let shapes: Vec<&Shape> = vec!(&Square(2.0), &Circle(1.0));
````

Pero eso no es todo. Si queremos cambiar la firma de la función ``sum_areas()``, vamos a tener un error refiriéndose a que los objetos de tipo trait deben incluir el keyword `dyn`

````rust
//Fix
fn sum_areas(ts: Vec<&dyn Shape>) -> f64
````