# Rust Project
Ejercicio de Rust: Leer data desde un archivo csv

Reto para Kanterita World Wide:
Implementación de proceso de migración de datos de personas en formato CSV a base de datos PostgreSQL con Rust

Con el código que creé, logré leer el archivo csv, el cual estaba delimitado con punto y coma (;). Pude almacenar el contenido del archivo en una variable String para luego separar y categorizar la información en una Struct data type. Hasta esa instancia pude llegar en el reto.   

Para ejecutar las lineas de código que creé, se debe usar el commando especificado a continuación. data.csv representa el documento csv de ejemplo con información de las personas. 
--> cargo run data.csv

Fuentes:
- https://github.com/bradtraversy/rust_sandbox
- https://doc.rust-lang.org/book/ch12-02-reading-a-file.html
- https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html
- https://levelup.gitconnected.com/working-with-csv-data-in-rust-7258163252f8
- https://docs.rs/csv/1.0.0/csv/tutorial/index.html
- https://github.com/gma2th/csvpsql
