## Esimerkki ohjelmasta, joka käyttää rakenteita

Ymmärtääksemme, milloin rakenteita kannattaa käyttää, kirjoitamme ohjelman, joka laskee suorakulmion pinta-alan. Aloitamme käyttämällä yksittäisiä muuttujia ja muokkaamme ohjelmaa vähitellen niin, että käytämme rakenteita.

Aloitetaan uusi binääriprojekti Cargo-ohjelmalla nimeltä _rectangles_, joka ottaa suorakulmion leveyden ja korkeuden pikseleinä ja laskee sen pinta-alan. Seuraava koodi näyttää yksinkertaisen tavan tehdä tämä:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/src/main.rs:all}}
```

Kun ohjelma ajetaan komennolla `cargo run`, saamme seuraavan tulosteen:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-08/output.txt}}
```

Vaikka tämä koodi toimii, voimme tehdä siitä selkeämmän ja luettavamman. Ongelmana on, että `area`-funktio ottaa kaksi erillistä parametria (`width` ja `height`), mutta niiden yhteys suorakulmioon ei ole selkeä.

### Refaktorointi käyttämällä tuplia

Voimme ryhmitellä leveyden ja korkeuden tuplaksi:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-09/src/main.rs}}
```

Vaikka tämä lisää hieman rakennetta, se ei ole täysin selkeää. Koska tuplan osilla ei ole nimiä, joudumme käyttämään indeksejä (`.0` ja `.1`), mikä voi johtaa sekaannuksiin.

### Rakenteen lisääminen parantamaan koodin ymmärrettävyyttä

Rakenteet mahdollistavat datan nimeämisen, jolloin koodi on helpompi ymmärtää:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-10/src/main.rs}}
```

Nyt meillä on `Rectangle`-rakenne, jossa on `width`- ja `height`-kentät. `area`-funktio ottaa rakenteen viitteenä (`&Rectangle`), jolloin `main` säilyttää omistajuuden `rect1`-muuttujaan.

### Rakenteen tulostaminen virheenkorjaukseen

Rust ei oletusarvoisesti tue rakenteiden suoraa tulostamista `println!`-makrolla, koska tulostusmuoto voi vaihdella. Jos yritämme tulostaa rakenteen:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-11/src/main.rs}}
```

saamme virheilmoituksen. Rust vaatii, että lisätään `#[derive(Debug)]`, jotta voimme käyttää `{:?}`-muotoilua:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/src/main.rs}}
```

Tulostus näyttää tältä:

```console
{{#include ../listings/ch05-using-structs-to-structure-related-data/listing-05-12/output.txt}}
```

Voimme myös käyttää `dbg!`-makroa, joka tulostaa muuttujan arvon ja sen sijainnin koodissa:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-05-dbg-macro/src/main.rs}}
```

Tämä voi olla hyödyllistä debuggaamisessa, sillä se tulostaa myös tiedoston ja rivin, jossa `dbg!` kutsuttiin.

### Yhteenveto

Olemme nähneet, kuinka rakenteet parantavat koodin luettavuutta ja käyttökelpoisuutta. Seuraavaksi siirrämme `area`-funktion rakenteen omaksi metodikseen.

