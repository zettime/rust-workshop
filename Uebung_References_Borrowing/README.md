# Übung References und Borrowing

In dieser Übungsaufgabe wird das Konzept References und Borrowing vertieft. Dafür sollen in der Datei [tic_tac_toe_lib/src/game/field.rs](tic_tac_toe_lib/src/game/field.rs) die Funktionen `display_field` und `do_move` implementiert werden.

Die Funktion `display_field` hat als Übergabeparameter ein zweidimensionales Integer Array und soll dieses bei der Ausführung in der Konsole ausgeben (println!):

``` console
0 | 0 | 0
0 | 0 | 0
0 | 0 | 0
```

Die Funktion `do_move` besitzt als Übergabeparameter ein zweidimensionales Integer Array, eine x und eine y Position sowie eine Player Id. Bei der Ausführung soll die entsprechende Kachel auf den Wert der Player Id gesetzt werden. Zusätzlich besitzt die Funktion keinen Rückgabewert!
Beim Ausführen der Funktion mit x=1, y=1 und player=1 soll das Feld wie folgt aussehen (Ausgabe mit der Funktion `display_field`):

``` console
0 | 0 | 0
0 | 1 | 0
0 | 0 | 0
```

Zusätzlich soll innerhalb der Datei [tic_tac_toe_lib/src/main.rs](tic_tac_toe_lib/src/main.rs) der Funktionsablauf implementiert werden (siehe Kommentare).
Zur Überprüfung kann das Programm mit `cargo run` ausgeführt werden.
