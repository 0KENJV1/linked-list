struct Nodo {
    dato: i32,
    puntero: Option<Box<Nodo>>,
}

struct ListaEnlazada {
    head: Option<Box<Nodo>>,
}
impl ListaEnlazada {
    //nueva lista vacia
    fn new() -> Self {
        ListaEnlazada { head: None }
    }

    //insertar elemento al inicio de la lista
    fn insetar_al_inicio(&mut self, dato: i32) {
        let nuevo_nodo = Box::new(Nodo {
            dato,
            puntero: self.head.take(),
        });
        self.head = Some(nuevo_nodo);
    }

    //insertar elemento al final de la lista
    fn insertar_al_final(&mut self, dato: i32) {
        let nuevo_nodo = Box::new(Nodo {
            dato,
            puntero: None,
        });

        if let Some(ref mut head) = self.head {
            let mut actual = head;
            while let Some(ref mut puntero) = actual.puntero {
                actual = puntero;
            }
            actual.puntero = Some(nuevo_nodo);
        } else {
            self.head = Some(nuevo_nodo);
        }
    }

    //eliminar un nodo
    fn eliminar_por_valor(&mut self, pdato: i32) {
        let mut actual = &mut self.head;

        loop {
            match actual {
                Some(ref mut nodo) if nodo.dato == pdato => {
                    *actual = nodo.puntero.take();
                    break;
                }
                Some(ref mut nodo) => {
                    actual = &mut nodo.puntero;
                }
                None => break,
            }
        }
    }

    //busqueda de un nodo
    fn buscar(&self, pdato: i32) -> bool {
        let mut actual = &self.head;

        while let Some(nodo) = actual {
            if nodo.dato == pdato {
                return true;
            }
            actual = &nodo.puntero;
        }
        false
    }

    //cambiar nodo
    fn cambiar_valor(&mut self, dato_viejo: i32, dato_nuevo: i32) {
        let mut actual = &mut self.head;

        while let Some(ref mut nodo) = actual {
            if nodo.dato == dato_viejo {
                nodo.dato = dato_nuevo;
                return;
            }

            actual = &mut nodo.puntero;
        }
    }

    //insertar nodo en una posicion especifica
    fn insertar_en_posicion(&mut self, pdato: i32, posicion: usize) {
        let mut actual = &mut self.head;
        let mut indice = 0;

        while let Some(ref mut nodo) = actual {
            if indice == posicion {
                let nuevo_nodo = Box::new(Nodo {
                    dato: pdato,
                    puntero: nodo.puntero.take(),
                });
                nodo.puntero = Some(nuevo_nodo);
                return;
            }
            actual = &mut nodo.puntero;
            indice += 1;
        }
        self.insertar_al_final(pdato);
    }

    //mostrar la lista
    fn mostrar(&self) {
        let mut actual = &self.head;
        while let Some(nodo) = actual {
            print!("{} -> ", nodo.dato);
            actual = &nodo.puntero;
        }
        println!("null");
    }
}

fn main() {
    let mut lista = ListaEnlazada::new();

    lista.insetar_al_inicio(4);
    lista.insetar_al_inicio(3);
    lista.insetar_al_inicio(1);

    lista.mostrar();

    lista.insertar_al_final(5);

    lista.mostrar();

    lista.insetar_al_inicio(6);

    lista.mostrar();

    lista.eliminar_por_valor(6);

    lista.mostrar();

    lista.cambiar_valor(3, 2);

    lista.mostrar();

    lista.insertar_en_posicion(3, 1);

    lista.mostrar();

    println!("esta el valor 3 en la lista? {}", lista.buscar(3));
    println!("esta el valor 6 en la lista? {}", lista.buscar(6));
}
