# Malphas / Una consola de fantasía, un límite real.

**Experimento de consola de fantasía moderna** para explorar un entorno donde crear y ejecutar juegos propios. Dart FFI, Flutter y Rust. Un laboratorio técnico secundario dentro de mi trabajo.

## Rust escribe. Dart lo ve.

La muestra pública asigna un framebuffer RGBA de **64 × 64**, lo conserva en Rust y lo observa desde Dart mediante una vista de memoria nativa. Dos llamadas de render, una misma dirección, un cambio verificable en el contenido. Sin Flutter y sin paquetes externos.

[Rust y contrato de memoria](samples/ffi/src/lib.rs) · [Consumidor Dart y comprobación cruzada](samples/ffi/check.dart)

[![Pruebas de la muestra](https://github.com/calinrus-dev/malphas-showcase/actions/workflows/verify.yml/badge.svg)](https://github.com/calinrus-dev/malphas-showcase/actions/workflows/verify.yml)

~~~sh
cd samples/ffi
cargo test --locked
cargo build --release --locked
dart analyze check.dart
dart run check.dart
~~~

Resultado esperado: `same_allocation: true`, `borrowed_view_observes_write: true`, checksums **1431552 → 1435648**. Las comprobaciones de Dart lanzan errores explícitos; no dependen de activar assertions.

## La frontera no perdona

- Rust conserva la propiedad. Dart toma una vista prestada y deja de usarla antes de liberar.
- Las llamadas son síncronas. No hay lectura concurrente con escritura ni liberación.
- El contrato define nulos, longitud, patrón y liberación única. No puede convertir un puntero arbitrario o un double-free en algo seguro.
- La alineación de 64 bytes es una elección del experimento. No demuestra una mejora de caché ni describe todos los procesadores.

No se copia el framebuffer al crear la vista de Dart. Eso **no** significa que una futura subida a GPU o una UI Flutter completa carezcan de copias. Aquí se demuestra exactamente el tramo publicado.

**Referencia nueva para publicación. No es la ABI, el runtime, el formato de cartuchos ni el sistema de firmas privados de Malphas.** No se publican cifras de FPS, comparaciones de rendimiento ni certificación multi-arquitectura a partir de este ejemplo.

[Intención de la consola](docs/EXPERIENCIA.md) · [Origen y límites](docs/PROVENANCE.md) · [Verificación](docs/VERIFICATION.md) · [Portfolio](https://github.com/calinrus-dev/portfolio)


[Instagram @c4linrus](https://www.instagram.com/c4linrus/) · [LinkedIn / calinrus](https://www.linkedin.com/in/calinrus/)
