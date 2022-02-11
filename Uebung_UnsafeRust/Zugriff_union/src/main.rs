use std::mem::ManuallyDrop;
//Informationen für den Umgang mit Unions finden Sie unter: //https://doc.rust-lang.org/reference/items/unions.html


//Tipp: Für die Angabe des Datentyps Strings kann  std::mem::ManuallyDrop<String> verwendet werden.
//Initialisieren Sie hier Union.

fn write_union() {
   ...
}
//Schreiben Sie eine Funktion für das Lesen und Ausgeben der geänderten Werte auf der Console.
fn read_union_float(...) -> ... {
    ...
}
fn read_union_string(...) -> ... {
    ...
}


fn main() {
    let mut x = MyUnion {...};
    write_union(...);
    let x = read_union_string(..);
    println!("Bitte geben Sie unter dieser Zeile ihre Ergebnisse aus.");

}


