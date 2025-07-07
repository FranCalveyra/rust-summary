# Resumen Lenguajes de Programación
Este resumen va a recopilar todo lo visto en la materia Lenguajes de Programación, tanto lo visto de Haskell como de Rust.


## Haskell
Es un lenguaje funcional puro, fuertemente tipado y con evaluación lazy. Se utiliza mucho en el ámbito académico y para explorar conceptos avanzados de programación funcional.

Inspirado en:
- ``Miranda (sintaxis y conceptos)``
- ``ML (sistema de tipos, inferencia de tipos)``
- ``Lisp (funciones de orden superior, recursión)``
- ``Otros (Scala, F#, Clean)``

Ver unidades:
- [Clase 1](./src/Haskell/Clase%201/README.md)
- [Clase 2](./src/Haskell/Clase%202/README.md)
- [Clase 3](./src/Haskell/Clase%203/README.md)
- [Clase 4](./src/Haskell/Clase%204/README.md)
- [Clase 5](./src/Haskell/Clase%205/README.md)

## Rust
Es un lenguaje funcional basado en C (y otros lenguajes que se denotan más abajo), un poco atípico respecto al resto de lenguajes que veníamos viendo.

Inspirado en:
- ``C (Memory model)``
- ``C++ (References, RAII, smart pointers)``
- ``Haskell (ADTs, Type inference, Typeclasses, pattern matching)``
- ``Others (OCaml, Erlang, Swift, Scheme)``

Ver unidades:
- [Clase 1](./src/Rust/Clase%201/README.md)
- [Clase 2](./src/Rust/Clase%202/README.md)
- [Clase 3](./src/Rust/Clase%203/README.md)
- [Clase 4](./src/Rust/Clase%204/README.md)
- [Clase 5](./src/Rust/Clase%205/README.md)

> Nota: lo de la última clase no lo incluyo porque se entiende perfecto del slide, además de que lo de Unicode es innecesario incluirlo. Es simplemente leer de ahí.
>Lo de Macros está perfectamente explicado ahí

## Para developers:
Si querés contribuir a este repo, vas a necesitar:
- `cargo` instalado (si querés correr programas en Rust, obvio que lo necesitás).
- Instalar el `crate` `mdbook` de Rust.
```
cargo install mdbook-katex
```
- Instalar `mdbook katex`
```
cargo install mdbook-katex
```
- Que te cope el Markdown :).
