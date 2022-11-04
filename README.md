# SistemasDistribuidos_UAM_C
Este es el repositorio de la Materia Sistemas Distribuidos.

Alumnos: 
- Oscar David González Pintor
- Alan Keeven Bastid Cervantes
- Alan Uriel Martinez Sanchez

Este programa fue creado en lenguaje RUST en este se busca implementar una lista doblemente ligada, como primera instancia nos basamos en el video de toma casita pra tener un mejor entendimiento del lenguaje y posteriormente fuimos daptando deacuerdo a la logica de la programacion en c++ y para ello  

Inciamos implementando la estructura del NODO donde contamos con un apuntador PREVIO y  un apuntador SIGUIENTE.
Utilizando enlaces o links<T>, un enlace opcional al archivo NODE.
Posteriormente implementamos las estrucuturas, junto con los meetodos Push (Back y Front). 
Dentro de esta estrucutura de la lista doblemente ligada, debemos definir el Nodo de incio (Head) y Nodo Final (Tail).
También se definio el tamaño en el que se declara la Lista. Incializanda desde 0.

## DEFINCIÓN DE TAREAS 1
Posteriormente como líder distrubuí a mis compañeros y colaboradores: Uriel y Keeven. los métodos; Pop Back y Pop Front respectivamente.

## Definción de Tareas 2
Las distrubiones de las respectivas tareas restantes se le asignó a los colaboradores: 
1. Alan Keveen Bastida Cervantes
    - Función: is_empty
    - Funcón: push

2. Alan Uriel Martínez Sánchez
    - Función: Clear
    - Función: Erase

3. Oscar David González Pintor
    - Función: Size
    - Función: Find

se tomo como referencia el ideo del siguiente enlace: 

## Librerias
Para el uso de nuestro programa requerimos el uso de la libreria:
- Cell 
    - El modulo estandar cell es utilizada para implementar contenedores mutables compartibles.
- RefCell 
    - Los valores de los tipos Cell<T>y RefCell<T>se pueden mutar a través de referencias compartidas, mientras que la mayoría de los tipos de Rust solo se pueden mutar a través de referencias únicas (&mut). Cell<T> y RefCell<T> proporciona 'mutabilidad interior', en contraste con los tipos típicos de Rust que exhiben 'mutabilidad heredada'.
- rc 
    - Rc<T>, que es una abreviatura de conteo de referencias. 
    Rc<T> realiza un seguimiento del número de referencias a un valor para determinar si el valor todavía está en uso o no. Si hay cero referencias a un valor, el valor se puede limpiar sin que ninguna referencia se vuelve inválida.



