# Malphas / Experiencia

[← Inicio](../README.md)

## Intención

Una consola de fantasía ofrece un entorno reconocible para crear y jugar. Malphas investiga cómo llevar esa idea a herramientas modernas: una interfaz Flutter y una base nativa, manteniendo cerca el trabajo del creador y la experiencia de ejecución.

## El recorrido

### 1. Abrir un proyecto

Exploración de un espacio para organizar proyectos y recursos de juego.

La revisión de esta fase observa si el contexto, la acción disponible y el resultado pueden entenderse sin perder el hilo del trabajo.

### 2. Preparar sus recursos

Investigación de una base Rust conectada a la experiencia visual.

La revisión de esta fase observa si el contexto, la acción disponible y el resultado pueden entenderse sin perder el hilo del trabajo.

### 3. Explorar la ejecución

Experimentación con la comunicación entre Dart y el entorno nativo.

La revisión de esta fase observa si el contexto, la acción disponible y el resultado pueden entenderse sin perder el hilo del trabajo.

### 4. Revisar la experiencia

Visión de un entorno común para crear, cargar y ejecutar juegos propios.

La revisión de esta fase observa si el contexto, la acción disponible y el resultado pueden entenderse sin perder el hilo del trabajo.

## Criterios de interacción

- **Un experimento con una dirección clara.** La consola de fantasía es la visión de producto; no se presenta como una plataforma terminada para terceros.
- **Crear y ejecutar en un mismo contexto.** El interés está en conectar el trabajo sobre recursos con la experiencia del juego.
- **Investigar el límite entre interfaz y motor.** Dart FFI permite explorar esa relación, sin publicar contratos ni implementación interna.

## Accesibilidad como criterio de diseño

La densidad de información debe conservar la lectura y el control de la interfaz. Los criterios de revisión son:

- Texto escalable y jerarquía legible, sin depender de una única resolución.
- Foco visible, orden de navegación predecible y alternativas a gestos complejos.
- Estados con texto o forma además del color; avisos que indiquen cómo continuar.
- Movimiento reducido, contraste y áreas de interacción adecuadas al contexto.
- Contenido disponible en texto junto a las láminas visuales de este repositorio.

Estos son criterios de diseño y validación; no equivalen a una auditoría de conformidad completada.

[Ver componentes](COMPONENTES.md) · [Consultar estado](ESTADO.md)
