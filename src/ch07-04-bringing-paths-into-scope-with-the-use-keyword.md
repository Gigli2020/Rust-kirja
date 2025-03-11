## Polkujen tuominen laajuuteen `use`-avainsanalla

Funktioiden kutsuminen polkuja kirjoittamalla voi tuntua hankalalta ja toistavalta. Luvun 7-7 esimerkissä, riippumatta siitä, käytimmekö absoluuttista tai suhteellista polkua `add_to_waitlist`-funktioon, jouduimme aina erikseen mainitsemaan `front_of_house`- ja `hosting`-moduulit. Onneksi tähän on helpotus: voimme luoda polulle lyhenteen `use`-avainsanalla ja käyttää jatkossa lyhyempää nimeä koko laajuuden sisällä.

Alla olevassa esimerkissä tuomme `crate::front_of_house::hosting`-moduulin `eat_at_restaurant`-funktion laajuuteen, jolloin riittää kutsua `hosting::add_to_waitlist`:

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-11/src/lib.rs}}
```

`use`-avainsanan lisääminen polun eteen vastaa symbolisen linkin luomista tiedostojärjestelmässä. Kun lisäämme `use crate::front_of_house::hosting`, `hosting` on nyt käytettävissä siinä laajuudessa kuin se olisi määritetty ohjelmakokonaisuuden juuressa.

Huomaa, että `use` luo lyhenteen vain siihen laajuuteen, jossa se on määritelty. Alla `eat_at_restaurant` siirretään uuteen `customer`-moduuliin, joka on eri laajuudessa kuin alkuperäinen `use`, mikä aiheuttaa käännösvirheen:

```rust,noplayground,test_harness,does_not_compile,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-12/src/lib.rs}}
```

Tämän voi korjata joko lisäämällä `use` myös `customer`-moduulin sisään tai viittaamalla ylätason polkuun `super::hosting`.

### Idiomaattiset `use`-polut

Voit miettiä, miksi edellisessä esimerkissä käytettiin `use crate::front_of_house::hosting` eikä tuotu funktiota suoraan laajuuteen polulla `use crate::front_of_house::hosting::add_to_waitlist`, kuten alla:

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-13/src/lib.rs}}
```

Molemmat tavat toimivat, mutta ensimmäinen on idiomaattisempi tapa käyttää `use`-avainsanaa funktioiden kohdalla. Kun tuomme funktiota sisältävän moduulin laajuuteen, meidän on edelleen kutsuttava funktiota moduulin nimen kautta (`hosting::add_to_waitlist`), mikä tekee koodista selkeämpää. Jos tuotaisimme funktion suoraan, olisi epäselvää, mistä se on peräisin.

Toisaalta rakenteiden, luetteloiden ja muiden kohteiden kohdalla on tapana tuoda koko polku. Alla tuodaan standardikirjaston `HashMap`-rakenne oikein:

```rust
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-14/src/main.rs}}
```

Tämä on vain vakiintunut käytäntö, johon Rust-yhteisö on tottunut.

Poikkeuksena tähän sääntöön on tapaus, jossa kaksi samaa nimeä käyttävää kohdetta tuodaan samaan laajuuteen. Alla tuodaan kaksi `Result`-tyyppiä eri moduuleista ja pidetään ne erillään:

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-15/src/lib.rs:here}}
```

Jos olisimme tuoneet molemmat suoraan `use std::fmt::Result` ja `use std::io::Result`, Rust ei tietäisi, kumpaa `Result`-tyyppiä tarkoitetaan.

### Uusien nimien antaminen `as`-avainsanalla

Jos haluamme tuoda kaksi samaa nimeä käyttävää tyyppiä laajuuteen, mutta ilman epäselvyyttä, voimme käyttää `as`-avainsanaa antamaan tyypille uuden nimen:

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-16/src/lib.rs:here}}
```

Tässä `std::io::Result` on nimetty `IoResult`-aliasilla, jolloin se ei sekoitu `std::fmt::Result`-tyyppiin.

### Nimien uudelleenjulkaisu `pub use` -avainsanalla

Kun tuomme nimen laajuuteen `use`-avainsanalla, se on oletuksena yksityinen. Jos haluamme, että muu koodi voi käyttää tuomaamme nimeä, voimme yhdistää `pub` ja `use` -avainsanat. Tätä kutsutaan **uudelleenjulkaisuksi** (*re-exporting*).

Alla `use`-avainsana on muutettu `pub use` -muotoon:

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-17/src/lib.rs}}
```

Aiemmin ulkoisen koodin olisi pitänyt kutsua `add_to_waitlist`-funktiota polulla `restaurant::front_of_house::hosting::add_to_waitlist()`, mutta nyt `restaurant::hosting::add_to_waitlist()` riittää.

Tätä tekniikkaa voi käyttää, jos koodin sisäinen rakenne eroaa siitä, miten ohjelmoijat ajattelevat kirjaston toiminnallisuutta.

### Ulkoisten pakettien käyttäminen

Jos käytämme ulkoista pakettia kuten `rand`, lisäämme sen `Cargo.toml`-tiedostoon:

```toml
{{#include ../listings/ch02-guessing-game-tutorial/listing-02-02/Cargo.toml:9:}}
```

Sen jälkeen voimme tuoda tarvittavat määrittelyt laajuuteen `use`-avainsanalla:

```rust,ignore
{{#rustdoc_include ../listings/ch02-guessing-game-tutorial/listing-02-03/src/main.rs:ch07-04}}
```

Standardikirjasto `std` on myös ulkoinen paketti, mutta koska se toimitetaan Rustin mukana, sitä ei tarvitse määrittää `Cargo.toml`-tiedostossa. Silti tarvitsemme `use`-avainsanaa sen laajuuteen tuomiseen:

```rust
use std::collections::HashMap;
```

### Sisäkkäisten polkujen käyttö suurten `use`-listojen siistimiseen

Jos haluamme tuoda useita kohteita samasta moduulista, voimme välttää turhia rivejä käyttämällä sisäkkäisiä polkuja. Sen sijaan, että kirjoittaisimme:

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-01-use-std-unnested/src/main.rs:here}}
```

Voimme käyttää lyhyempää muotoa:

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-18/src/main.rs:here}}
```

Tätä voidaan käyttää myös eri tasojen yhdistämiseen. Seuraavassa `std::io` ja `std::io::Write` tuodaan samalla rivillä:

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-20/src/lib.rs}}
```

### Yleismerkkioperaattori `*`

Voimme tuoda _kaikki_ polussa määritetyt julkiset kohteet laajuuteen käyttämällä `*`-merkkiä:

```rust
use std::collections::*;
```

Tämä tuo kaikki `std::collections`-moduulin julkiset kohteet laajuuteen. Tätä on käytettävä varoen, koska se voi tehdä koodista vaikeaselkoisemman.

