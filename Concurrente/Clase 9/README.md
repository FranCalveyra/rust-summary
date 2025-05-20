Programación Asíncrona - Continuación

### En JavaScript se los llama `Promises`

```javascript
let coffeeNumber = 0;

function makeCoffeePromise(): Promise<string> {
    return new Promise((resolve) => {
        console.log("Making Coffee");
        setTimeout(() => {
            coffeeNumber += 1;
            resolve(`coffee ${coffeeNumber}`);
        }, 2000);
    });
}

function coffeeBreak() {
    const f: Promise<string> = makeCoffeePromise();
    f.then(coffee => {
        drink(coffee);
    })
    chatWithColleagues();
}
```

#### Syntax Sugar para el Async/Await

```javascript
async function coffeeBreak(): Promise<void> {

    const f: Promise<string> = makeCoffeePromise();
    chatWithColleagues();

    const coffee = await f;
    // The code below will be executed when the promised is fullfilled
    drink(coffee);

    // Promised is propagated! 
}
```

### JavaScript es Single-Threaded

- Originado en navegadores: javascript fue diseñado para manipular el DOM en navegadores web.
- El modelo single-threaded previene conflictos e inconsistencias.
- Event Loop: javascript opera sobre un modelo basado en eventos. El bucle de eventos verifica tareas como entradas de
  usuario, solicitudes de red y temporizadores.
- Asegura la capacidad de respuesta procesando un evento a la vez.