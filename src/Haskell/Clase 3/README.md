# Tipos definidos por usuario, manejo de errores, type classes

Para definir un tipo, se usa el keyword `data`.
```haskell
data Weekday = Sunday | Monday | Tuesday | Wednesday | Thursday 
             | Friday | Saturday
```
El identificador después de `data (Weekday)` es el **nombre** del nuevo tipo. Los nombres a la derecha (Sunday, etc) se llaman constructores.
```haskell
isWeekend :: Weekday -> Bool
isWeekend Sunday   = True
isWeekend Saturday = True
isWeekend _        = False

isWeekend Friday -- False
```
```haskell
data Weekday = Sunday | Monday | Tuesday | Wednesday | Thursday 
             | Friday | Saturday deriving (Eq)
                    
isWeekend2 :: Weekday -> Bool
isWeekend2 w = w == Sunday || w == Saturday -- Más corto porque derivan el Trait Eq

isWeekend2 Sunday
```
## Deriving
Haskell puede hacer que cualquier user-defined type derive cualquiera de los siguientes:
`Eq`, `Ord`, `Show`, `Read`, `Enum`, `Bounded`.

```haskell
data Weekday = Sunday | Monday | Tuesday | Wednesday | Thursday | Friday | Saturday
                    deriving (Eq, Ord)
                    
isWeekend2 :: Weekday -> Bool
isWeekend2 w = w == Sunday || w == Saturday -- Eq

-- isWeekend2 Tuesday

Tuesday > Monday -- Ord (es que se los puede comparar por <, >)
Sunday < Monday
```
Como bien dijimos antes, `Show` transforma el tipo a un String. `Read`, por su parte, lee un String y lo transforma al tipo custom.

```haskell
plural::Weekday -> String
plural day = show day ++ "s"

isWeekend :: Weekday -> Bool
isWeekend w = w == Sunday || w == Saturday

(read "Sunday")::Weekday

plural (read "Sunday")

isWeekend (read "Friday")
```

### Enum y Bounded
Enum Defines operations on sequentially ordered types: succ, pred, toEnum, fromEnum, operations to support ranges (enumFromTo, etc)

```haskell
data Weekday = Sunday | Monday | Tuesday | Wednesday | Thursday | Friday | Saturday 
                deriving (Eq, Ord, Show, Read, Enum, Bounded)

succ Sunday

pred Friday

fromEnum Tuesday

(toEnum 2)::Weekday


[Monday, Wednesday .. ]

-- Output:
Monday
Thursday
2
Tuesday
[Monday,Wednesday,Friday]
```
`Bounded` Defines the limits of a type (minBound and maxBound)

```haskell
x = minBound::Weekday -- Sunday
```

## Product Types
Instead of using tuples we can use `data` to create new types combining existing ones.
```haskell
data Point = Pt Double Double deriving (Eq, Show)

Pt 10.0 3.5
origin = Pt 0 0
origin
-- Output
Pt 10.0 3.5
Pt 0.0 0.0
```
`Point` is the name of the new type. `Pt` is the constructor.

This is also valid: `data Point = Point Double Double`.

_Type names_ and _Constructors_ are in different namespaces, so then don't interfere.

Constructors are actually functions