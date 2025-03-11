## Tiiviimpi ohjausrakenne `if let` ja `let else` -rakenteilla

`if let` -syntaksi yhdistää `if`- ja `let`-rakenteet tiiviimmäksi tavaksi käsitellä tapauksia, joissa haluamme suorittaa koodia vain yhden kuvion täsmätessä ja ohittaa muut. Tarkastellaan ohjelmaa, joka käyttää `match`-rakennetta `Option<u8>`-arvon käsittelyyn:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-06/src/main.rs:here}}
```

Koska haluamme tehdä jotain vain `Some`-variantin kanssa ja jättää `None` huomiotta, joudumme lisäämään `_ => ()` -haaran, mikä on ylimääräistä koodia.

Tämä voidaan kirjoittaa lyhyemmin `if let` -rakenteella:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-12-if-let/src/main.rs:here}}
```

Tässä `if let Some(max) = config_max` toimii kuten `match`, mutta se käsittelee vain `Some(max)`-tapauksen ja ohittaa muut. 

### `if let` ja `else`

Voimme myös yhdistää `if let`-rakenteeseen `else`-haaran:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-14-count-and-announce-if-let-else/src/main.rs:here}}
```

Tämä vastaa seuraavaa `match`-rakennetta:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-13-count-and-announce-match/src/main.rs:here}}
```

`if let` tekee koodista tiiviimpää, mutta toisin kuin `match`, se ei pakota käsittelemään kaikkia mahdollisia tapauksia.

### "Happy path" ja `let else`

Jos haluamme suorittaa laskennan, kun arvo on olemassa, ja käyttää oletusarvoa muuten, `let else` voi auttaa. Esimerkiksi voimme tarkistaa, kuinka vanha kolikossa oleva osavaltio on:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-07/src/main.rs:state}}
```

Tämän voi toteuttaa `if let` -rakenteella:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-08/src/main.rs:describe}}
```

Tässä `if let` joko tuottaa arvon tai päättää funktion suorittamisen, mutta looginen kulku on hieman vaikeampi seurata.

`let else` tekee tästä selkeämmän:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-09/src/main.rs:describe}}
```

Tässä ohjelman pääasiallinen suorituspolku pysyy selkeänä, ja `else`-haara hoitaa varhaisen poistumisen.

### Yhteenveto

Olemme nyt käsitelleet:

- `match`: pakottaa käsittelemään kaikki vaihtoehdot.
- `if let`: tekee koodista tiiviimpää silloin, kun tarvitsemme vain yhden haaran.
- `let else`: mahdollistaa sujuvamman koodin, kun haluamme varmistaa pääpolun selkeyden.

Seuraavaksi tarkastelemme Rustin moduuleja, jotka auttavat järjestämään koodia selkeästi.
