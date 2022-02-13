# Übung References und Borrowing

In dieser Übungsaufgabe wird das Konzept References und Borrowing vertieft. Dafür sollen in der Datei [tic_tac_toe_lib/src/game/field.rs](tic_tac_toe_lib/src/game/field.rs) die Funktionen `display_field` und `do_move` implementiert werden.

`display_field` hat als übergabe Parameter ein zwei dimensionales integer array und soll dieses bei der Ausführung in der Konsole ausgeben (println!):

``` console
0 | 0 | 0
0 | 0 | 0
0 | 0 | 0
```

`do_move` besitzt als Übergabeparameter ein zwei dimensionales integer array, eine x und eine y position sowie die player id. Bei der Ausführung soll die entsprechende Kachel auf den Wert der player id gesetzt werden. Zusätzlich besitzt die Funktion keinen Rückgabewert!
Beim Ausführen der Funktion mit x=1, y=1 und player=1 soll das Feld wie folgt aussehen (Ausgabe mit display_field):

``` console
0 | 0 | 0
0 | 1 | 0
0 | 0 | 0
```

Um zu sehen ob die Implementierung korrekt ist, müssen die Funktionen innerhalb der Datei [tic_tac_toe_lib/src/main.rs](tic_tac_toe_lib/src/main.rs) aufgerufen werden (siehe Kommentare). Zusätzlich kann dan Programm mit `cargo run` ausgeführt werden.