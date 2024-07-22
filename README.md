# Sat Reporter

Aplicación de línea de comandos para realizar reportes de facturas del SAT, leyendo los xml.

## Uso

```
# Imprime el total de facturación (incluyendo impuestos) del emisor
$ satr report emisor <EMISOR_RFC> total <PATH>

$ satr report emisor <EMISOR_RFC> total -s <DATE_START> -e <DATE_END> <PATH>
$ satr report emisor <EMISOR_RFC> subtotal <PATH>
$ satr report emisor <EMISOR_RFC> iva <PATH>
$ satr report emisor <EMISOR_RFC> isr <PATH>
```

## Próximas funciones
```
# Imprime el desglose de todas las facturas que encuentre en la carpeta actual.
# Buscará de forma recursiva.
$ satr print .

# Imprime el desglose de todas la facturas que coincidan con el criterio de búsqueda
# en la carpeta indicada, de forma recursiva
$ satr find emisor --name <EMISOR_NAME> --date_start <DATE_START> --date_end <DATE_END> <PATH>

# fecha final de búsqueda implícita (hoy)
$ satr find emisor --rfc <EMISOR_RFC> --date_start <DATE_START> <PATH>

# path implícito (carpeta actual)
$ satr find receptor --rfc <RECEPTOR_RFC>
```

## Instalar
### Desde el código fuente
Descargar el repositorio.
```
git clone https://github.com/jonhteper/satr.git
```

Compilar.
```
make
```

Reducir tamaño del binario (opcional).
***Importante:** para este paso es necesario tener instalado [upx](https://github.com/upx/upx).*
```
make opt
```

Instalar (sistemas UNIX like).
```
sudo make install
```
