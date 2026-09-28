# Origen y alcance de la muestra pública

[← Proyecto](../README.md)

## Qué se publica

Referencia nueva e independiente creada para publicación. Ilustra una frontera Rust/Dart de framebuffer prestado; no extrae código del engine ni publica la ABI real de Malphas.

[Inspeccionar la pieza](../samples/ffi/src/lib.rs). La primera publicación de estas muestras es del 28 de septiembre de 2026. Esa fecha no pretende representar la fecha de creación del producto ni actividad de desarrollo histórica.

## Qué puede comprobar otra persona

El código público, las pruebas y el workflow están en este mismo repositorio. Se pueden clonar, ejecutar y discutir. La procedencia desde archivos privados es una declaración del autor: un lector externo no tiene acceso a ese historial para contrastarla. Las adaptaciones se describen arriba para no confundir una muestra con el producto completo.

## Límites

Propiedad y lifetime se rigen por el contrato unsafe documentado. Se necesita serializar lecturas, escrituras y liberación. La alineación se comprueba; no se ha demostrado un beneficio de rendimiento. No hay presentación GPU ni benchmark de Flutter.

La publicación de estas piezas no abre el núcleo del producto. Las APIs internas, secretos, claves, datos de usuario, contenido comercial y demás implementaciones privadas quedan fuera.

## Derechos

Copyright © 2026 Calin Rus. Código visible para evaluación técnica. No se incorpora una licencia general de reutilización al producto privado.

## Referencias del contrato

[FFI en Rust](https://doc.rust-lang.org/nomicon/ffi.html) y [C interop en Dart](https://dart.dev/interop/c-interop): documentación oficial para revisar propiedad, punteros y llamadas entre lenguajes.
