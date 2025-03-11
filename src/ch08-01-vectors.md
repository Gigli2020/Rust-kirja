## Arvolistojen tallentaminen vektoreilla

Ensimmäinen kokoelmatyyppi, jota tarkastelemme, on `Vec<T>`, eli **vektori**. Vektorit mahdollistavat useamman arvon tallentamisen yhteen tietorakenteeseen, jossa kaikki arvot sijaitsevat vierekkäin muistissa. Vektorit voivat tallentaa vain saman tyyppisiä arvoja. Ne ovat hyödyllisiä esimerkiksi silloin, kun sinulla on lista kohteista, kuten tiedoston tekstirivit tai ostoskorin tuotteiden hinnat.

### Uuden vektorin luominen

Uuden tyhjän vektorin luomiseen käytetään `Vec::new`-funktiota, kuten alla olevassa esimerkissä:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-01/src/main.rs:here}}
```

Huomaa, että tässä käytetään tyyppimääritystä. Koska emme lisää vektoriin heti mitään arvoja, Rust ei voi päätellä sen sisältämää tyyppiä. Vektorit on toteutettu geneerisillä tyypeillä; käsittelemme geneerisiä tyyppejä tarkemmin luvussa 10. Tässä tapauksessa olemme kertoneet Rustille, että `Vec<T>` vektorissa `v` tallennetaan `i32`-tyyppisiä arvoja.

Useimmiten vektorit luodaan kuitenkin alustusarvoilla, jolloin Rust päättelee arvon tyypin automaattisesti. Rust tarjoaa `vec!`-makron, joka luo uuden vektorin annetuilla arvoilla:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-02/src/main.rs:here}}
```

Koska alustusarvoina on `i32`-tyyppisiä lukuja, Rust päättelee `v`-vektorin tyypiksi `Vec<i32>`, eikä erillistä tyyppimääritystä tarvita.

### Vektorin päivittäminen

Vektoriin voidaan lisätä uusia arvoja `push`-metodilla, kuten alla olevassa esimerkissä:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-03/src/main.rs:here}}
```

Kuten kaikissa muuttujissa, vektorin arvon muuttaminen edellyttää sen määrittämistä `mut`-avainsanalla.

### Vektorin alkioiden lukeminen

Vektorin arvoihin voidaan viitata kahdella tavalla: käyttämällä indeksiä tai `get`-metodia. Alla oleva esimerkki havainnollistaa molempia tapoja:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-04/src/main.rs:here}}
```

Rust tarjoaa nämä kaksi tapaa, jotta voit hallita ohjelman toimintaa, jos yritetään käyttää olematonta indeksiä. Jos esimerkiksi viitataan vektorin 100. alkioon, kun vektorissa on vain viisi alkiota, ohjelma kaatuu `[]`-syntaksin käytön yhteydessä, mutta `get`-metodi palauttaa `None` eikä aiheuta virhettä:

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-05/src/main.rs:here}}
```

Joskus on hyödyllistä antaa käyttäjälle toinen mahdollisuus syöttää oikea indeksi sen sijaan, että ohjelma kaatuisi.

### Vektorin iteroiminen

Vektorin arvoja voidaan käydä läpi `for`-silmukalla. Alla oleva esimerkki käy läpi `i32`-tyyppisen vektorin arvot ja tulostaa ne:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-07/src/main.rs:here}}
```

Jos haluamme muuttaa jokaisen arvon, voimme käyttää `for`-silmukkaa yhdessä muuttuvien viittausten kanssa:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-08/src/main.rs:here}}
```

Koska `i` on viittaus vektorin arvoon, meidän täytyy käyttää `*`-operaattoria (*dereference operator*) päästäksemme käsiksi todelliseen arvoon.

### Enumien käyttö vektoreissa eri tyyppisten arvojen tallentamiseen

Vektorit voivat tallentaa vain saman tyyppisiä arvoja, mutta joskus haluamme tallentaa erilaisten tietotyyppien arvoja samaan listaan. Tällöin voimme käyttää `enum`-tietotyyppiä, joka mahdollistaa useiden eri tietotyyppien yhdistämisen:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-09/src/main.rs:here}}
```

Koska `SpreadsheetCell`-enum on yksi tietotyyppi, voimme tallentaa sitä sisältäviä arvoja vektoriin. Rust varmistaa käännösaikana, että kaikki tapaukset on käsitelty oikein.

### Vektorin vapauttaminen

Kuten kaikki `struct`-tyyppiset tietorakenteet, vektori vapautetaan muistista, kun se poistuu laajuudestaan. Alla oleva esimerkki havainnollistaa tätä:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-10/src/main.rs:here}}
```

Kun vektori vapautetaan, myös sen sisältämät arvot vapautetaan muistista.

Seuraavaksi tutustumme toiseen yleiseen kokoelmaan: `String`-tietotyyppiin!
