## Kommentit

Kaikki ohjelmoijat pyrkivät tekemään koodistaan helposti ymmärrettävää, mutta joskus tarvitaan lisäselityksiä. Näissä tapauksissa ohjelmoijat jättävät _kommentteja_ lähdekoodiinsa. Kääntäjä ohittaa ne, mutta koodia lukeva ihminen voi löytää ne hyödyllisiksi.

Tässä on yksinkertainen kommentti:

```rust
// hello, world
```

Rustissa idioomaattinen kommenttityyli alkaa kahdella kauttaviivalla, ja kommentti jatkuu rivin loppuun asti. Jos kommentti ulottuu usealle riville, sinun täytyy lisätä `//` jokaiselle riville, kuten tässä:

```rust
// Meillä on tässä jotain monimutkaista, niin pitkää, että tarvitsemme
// useita rivejä kommentteja selittämään sen! Huh! Toivottavasti tämä kommentti
// tekee asian ymmärrettäväksi.
```

Kommentteja voidaan myös sijoittaa koodirivin loppuun:

<span class="filename">Tiedostonimi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-24-comments-end-of-line/src/main.rs}}
```

Useammin niitä kuitenkin käytetään erillisellä rivillä ennen kommentoitavaa koodia:

<span class="filename">Tiedostonimi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-25-comments-above-line/src/main.rs}}
```

Rustissa on myös toisenlainen kommentti, dokumentaatiokommentti, josta keskustelemme tarkemmin luvun 14 osiossa [“Julkaisu Crates.io:hon”][publishing]<!-- ignore -->.

[publishing]: ch14-02-publishing-to-crates-io.html
