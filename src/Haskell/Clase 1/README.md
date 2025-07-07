# Haskell

## Características principales
- Tipado estático
- Inferencia de tipos
- Puramente funcional
  - **Transparencia referencial** (puedes razonar sobre el comportamiento del programa como un sistema de reescritura).
  - **No** muta el estado.
  - Las operaciones de E/S con efectos secundarios viven en su propio espacio y son solo una descripción de lo que se debe hacer basándose en el núcleo funcional
- _Evaluación lazy_: las funciones y expresiones no se evalúan hasta que son necesarias.

## Primeros pasos
```haskell
3 + 4
5 * 10
(50 * 100) - 4999  / 3
2^2 + 2**1.2

3^3

4 ** 0.5
```
Salida:
```
7
50
3333.666666666667
6.29739670999407
27
2.0
```

```haskell
10 == 20
5 /= 2
15 > 2^2 && 3 > 1/2 || 7 > 2
not ("Hello" == "He" ++ "llo")
```
Salida:
```
False
True
True
False
```

```haskell
-- div vs '/' rem. infix, vs prefix

(/) 2 3

5 `div` 3
```

## Primeras funciones
```haskell
succ n = n+1
square n = n*n
double n = 2*n
```

### Función para ver un tipo
```haskell
:t square
```
Salida: `square :: forall {a}. Num a => a -> a`

> Tener en cuenta que la parte de `forall {a} Num a` es la declaración del "trait" que tiene que seguir el tipo que se le pasa  
> Las firmas de las funciones en Haskell son de la siguiente forma:  
> `<Nombre de la función> :: <Trait opcional => > <Tipo del parámetro 1> -> <Tipo del parámetro 2> ... -> <Tipo del valor de retorno>`  
>  
> Los parámetros son claramente opcionales. Yo le digo "trait" por el sesgo hacia `Rust`, y porque es más fácil entenderlo así.

## Condicionales

### If
```haskell
max:: Int -> Int -> Int
max a b = if a > b then a else b

factorial:: Int -> Int
factorial n = if n <= 1 then 1 else n * factorial (n-1)
```

### Guards
```haskell
max a b
    | a > b     = a
    | otherwise = b
 
factorial n 
    | n <= 1    = 1
    | otherwise = n * factorial (n-1)
```

### Where
```haskell
max4:: Int -> Int -> Int -> Int -> Int
max4 i j k n = max max12 max34
    where max12 = max i j 
          max34 = max k n
max4 1 2 4 3
```

### Let
```haskell
max4 i j k n =
    let 
        max12 = max i j 
        max34 = max k n
    in  max max12 max34
max4 1 2 4 3
```

### Pattern Matching
Las funciones de Haskell tienen pattern matching built-in, lo que te permite abarcar casos por defecto.
```haskell
fibo:: Int -> Int
fibo 1 = 1
fibo 2 = 2
fibo n = fibo (n-1) + fibo (n-2)  

fibo 7
```

## Tuplas
```haskell
oranges:: (String, Int, Float)
oranges = ("Oranges", 10, 0.25)
oranges -- Output: ("Oranges",10,0.25)
```

**Tupla vacía / Unit**
```haskell
x :: ()
x = ()
x -- Output: ()
```

Existe pattern matching con tuplas:
```haskell
price:: (String, Int, Float) -> Float
price (_, _, p) = p

price ("Oranges", 10, 0.25) -- Output: 0.25. Sólo agarra el precio
```

### Ejemplo: ecuación cuadrática
$$
ax^2 + bx + c = 0 
$$

$$
x_{1,2} = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
$$

Solución en Haskell:
```haskell
quadratic:: Double -> Double -> Double -> Double
quadratic a b c = (-b + sqrt (b^2 - 4 * a * c)) / (2 * a)
```

Pequeño tweak:
```haskell
quadratic:: Double -> Double -> Double -> Double
quadratic a b c = (-b + sq) / (2 * a)
    where 
        sq   = sqrt (b^2 - 4 * a * c)
```

Validar el radicando negativo
```haskell
quadratic:: Double -> Double -> Double -> Double
quadratic a b c 
        | r < 0     = error "Not real roots"
        | otherwise = (-b + sq) / (2 * a)
    where 
        r   = b^2 - 4 * a * c 
        sq  = sqrt r
```

### Devolver ambos resultados
```haskell
quadratic:: Double -> Double -> Double -> (Double, Double)
quadratic a b c 
        | r < 0     = error "Not real roots"
        | otherwise = ((-b + sq) / a2, (-b - sq) / a2)
    where 
        r   = b^2 - 4 * a * c 
        sq  = sqrt r
        a2  = 2 * a

quadratic 1 0 (-1)
```

Verificar casos degenerados:
```haskell
quadratic:: Float -> Float -> Float -> (Float, Float)
quadratic a b c
    | a == 0 && b == 0 = error "Invalid a == 0, b == 0"
    | r < 0            = error "Imaginary root"
    | a == 0           = (cb, cb)
    | otherwise        = (t1 + t2, t1 - t2)
    where
        r = b**2 - 4 * a * c
        t1 = -b / (2 * a)
        t2 = sqrt r / (2*a)
        cb = (-c) / b
        
quadratic 0 1.0 (-1)
```