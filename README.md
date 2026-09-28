![Malphas — Una consola de fantasía para crear.](assets/hero.svg)

# Malphas

**Una consola de fantasía para crear.**

Un experimento de consola de fantasía moderna que explora cómo conectar Flutter y un núcleo nativo Rust mediante Dart FFI, con la intención de ofrecer un entorno donde crear y ejecutar juegos propios.

**Stack:** Dart FFI · Flutter · Rust  
**Estado:** Experimento de consola de fantasía

[Portfolio](https://github.com/calinrus-dev/portfolio) · [Experiencia](docs/EXPERIENCIA.md) · [Componentes](docs/COMPONENTES.md) · [Diseño técnico](docs/ARQUITECTURA.md) · [Demostraciones](docs/DEMOSTRACIONES.md) · [Estado](docs/ESTADO.md)

## El problema que aborda

Una consola de fantasía ofrece un entorno reconocible para crear y jugar. Malphas investiga cómo llevar esa idea a herramientas modernas: una interfaz Flutter y una base nativa, manteniendo cerca el trabajo del creador y la experiencia de ejecución.

## Qué compone la experiencia

- **Entorno de creación.** Exploración de un espacio para organizar proyectos y recursos de juego.
- **Ejecución nativa.** Investigación de una base Rust conectada a la experiencia visual.
- **Puente Dart FFI.** Experimentación con la comunicación entre Dart y el entorno nativo.
- **Experiencia de consola.** Visión de un entorno común para crear, cargar y ejecutar juegos propios.

![Mapa conceptual de Malphas: Abrir un proyecto → Preparar sus recursos → Explorar la ejecución → Revisar la experiencia.](assets/experiencia.svg)

*Lámina explicativa con datos ficticios. Su contenido también está disponible como texto en [Componentes](docs/COMPONENTES.md).*

## Decisiones que definen el proyecto

- **Un experimento con una dirección clara.** La consola de fantasía es la visión de producto; no se presenta como una plataforma terminada para terceros.
- **Crear y ejecutar en un mismo contexto.** El interés está en conectar el trabajo sobre recursos con la experiencia del juego.
- **Investigar el límite entre interfaz y motor.** Dart FFI permite explorar esa relación, sin publicar contratos ni implementación interna.

## Explorar el caso

- [Experiencia y recorrido](docs/EXPERIENCIA.md): intención, interacción y criterios de revisión.
- [Componentes](docs/COMPONENTES.md): las piezas visibles y el papel de cada una.
- [Diseño técnico](docs/ARQUITECTURA.md): responsabilidades y compromisos de diseño.
- [Demostraciones](docs/DEMOSTRACIONES.md): qué enseñan las imágenes y cómo leer la evidencia.
- [Estado y siguientes pasos](docs/ESTADO.md): alcance actual, comprobaciones y trabajo pendiente.

## Sobre este repositorio

Caso de estudio público de un proyecto con implementación privada. Reúne documentación, diagramas e imágenes seleccionadas. Los detalles del motor, integraciones, datos operativos y código se mantienen en los repositorios privados.

Revisión editorial: 28 de septiembre de 2026. Autor: [Calin Rus](https://github.com/calinrus-dev).

[calinrus.com](https://calinrus.com) · [Instagram @c4linrus](https://www.instagram.com/c4linrus/) · [Todos los proyectos](https://github.com/calinrus-dev/portfolio)
