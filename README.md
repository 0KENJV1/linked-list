---

# Lista Enlazada en Rust

Este proyecto es una implementación de una **lista enlazada simple** en Rust. La lista enlazada es una estructura de datos fundamental que permite almacenar y manipular una secuencia de elementos de manera eficiente.

---

## Características

- **Inserción al inicio**: Agrega un nodo al principio de la lista.
- **Inserción al final**: Agrega un nodo al final de la lista.
- **Inserción en posición específica**: Agrega un nodo en una posición dada.
- **Eliminación por valor**: Elimina un nodo que contenga un valor específico.
- **Cambiar valor de un nodo**: Modifica el valor de un nodo existente.
- **Reemplazar nodo**: Elimina un nodo viejo y lo reemplaza por uno nuevo.
- **Búsqueda de un valor**: Verifica si un valor existe en la lista.
- **Mostrar la lista**: Imprime la lista en formato `1 -> 2 -> 3 -> null`.

---

## Cómo usar

### Requisitos
- Rust instalado. Puedes descargarlo desde [rustup.rs](https://rustup.rs/).

### Ejecución
1. Clona este repositorio:
   ```bash
   git clone https://github.com/0KENJV1/linked-list
   cd linked-list
   ```

2. Compila y ejecuta el proyecto:
   ```bash
   cargo run
   ```


---

## Estructura del código

### `Nodo`
Representa un nodo en la lista enlazada. Cada nodo contiene:
- `dato`: El valor almacenado en el nodo.
- `puntero`: Una referencia al siguiente nodo en la lista.

```rust
struct Nodo {
    dato: i32,
    puntero: Option<Box<Nodo>>,
}
```

### `ListaEnlazada`
Representa la lista enlazada. Contiene un puntero al primer nodo (`cabeza`).

```rust
struct ListaEnlazada {
    cabeza: Option<Box<Nodo>>,
}
```

### Funciones implementadas
- `new()`: Crea una nueva lista vacía.
- `insertar_al_inicio(dato: i32)`: Inserta un nodo al inicio de la lista.
- `insertar_al_final(dato: i32)`: Inserta un nodo al final de la lista.
- `insertar_en_posicion(dato: i32, posicion: usize)`: Inserta un nodo en una posición específica.
- `eliminar_por_valor(valor: i32)`: Elimina un nodo con un valor específico.
- `cambiar_valor(valor_viejo: i32, valor_nuevo: i32)`: Cambia el valor de un nodo existente.
- `reemplazar_nodo(valor_viejo: i32, valor_nuevo: i32)`: Reemplaza un nodo viejo por uno nuevo.
- `buscar(valor: i32) -> bool`: Busca un valor en la lista.
- `mostrar()`: Imprime la lista en formato `1 -> 2 -> 3 -> null`.

---

## Licencia
Este proyecto está bajo la licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más detalles.


---

¡Espero que este proyecto te sea útil para aprender sobre listas enlazadas y Rust! 😊

---

