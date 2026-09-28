# Cómo comprobar esta muestra

[← Proyecto](../README.md) · [Origen](PROVENANCE.md)

## Repetir la comprobación

Rust con Cargo y Dart 3.8.1. Sin dependencias de crates.io o pub.dev. La integración local se ejecutó en Windows x64; el workflow prueba Linux x64. La compatibilidad con otros destinos necesita su propia ejecución.

Desde la raíz de este repositorio:

~~~sh
cd samples/ffi
cargo test --locked
cargo build --release --locked
dart analyze check.dart
dart run check.dart
~~~

Última ejecución local: **4 pruebas aprobadas**, más análisis estático de Dart y ejecución real contra la DLL compilada, 28 de septiembre de 2026. Este es un resultado fechado, no una promesa sobre cambios futuros.

[Workflow y ejecuciones públicas](https://github.com/calinrus-dev/malphas-showcase/actions/workflows/verify.yml). Abre una ejecución para ver el commit exacto y los logs; el badge del README sigue la rama actual.

## Qué no certifican estas pruebas

Propiedad y lifetime se rigen por el contrato unsafe documentado. Se necesita serializar lecturas, escrituras y liberación. La alineación se comprueba; no se ha demostrado un beneficio de rendimiento. No hay presentación GPU ni benchmark de Flutter.

Las pruebas nuevas ejercitan las piezas públicas. No se suman a las cifras históricas de tests del producto como si fueran la misma suite.
