## Kuinka kirjoittaa testejä

Testit ovat Rust-funktioita, jotka varmistavat, että muu kuin testikoodi toimii odotetulla tavalla. Testifunktioiden rungot suorittavat yleensä seuraavat vaiheet:

- Asetetaan tarvittavat tiedot tai tila.
- Suoritetaan testattava koodi.
- Varmistetaan, että tulokset ovat odotetut.

Tarkastelemme Rustin tarjoamia ominaisuuksia, jotka on tarkoitettu testien kirjoittamiseen, kuten `test`-attribuuttia, muutamia makroja ja `should_panic`-attribuuttia.

### Testifunktion rakenne

Yksinkertaisimmillaan Rust-testit ovat funktioita, jotka on merkitty `#[test]`-attribuutilla. Attribuutit ovat Rust-koodin metatietoa; esimerkiksi `derive`-attribuuttia käytettiin rakenteissa luvussa 5. Kun testin edessä on `#[test]`, Rust tietää, että se on testifunktio.

Kun suoritat testit komennolla `cargo test`, Rust kokoaa testejä suorittavan ohjelman, joka ajaa testifunktiot ja raportoi, onnistuivatko ne vai eivät.

Kun luomme uuden kirjasto-projektin Cargo-komennolla, _src/lib.rs_-tiedostoon luodaan automaattisesti testimoduuli. Tämä moduuli tarjoaa pohjan testien kirjoittamiseen, jotta syntaksi ja rakenne ovat helposti käytettävissä.

Kokeillaan testien toimivuutta ennen kuin testaamme oikeaa koodia. Luodaan uusi kirjasto nimeltä `adder`:

```console
$ cargo new adder --lib
     Created library `adder` project
$ cd adder
```

Tämän kirjaston _src/lib.rs_-tiedoston sisältö näyttää seuraavalta:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

Tiedosto sisältää yksinkertaisen `add`-funktion, jotta meillä on jotain testattavaa.

Tarkastellaan `it_works`-funktiota. `#[test]`-attribuutti kertoo testirunnerille, että kyseessä on testi. Funktiossa käytetään `assert_eq!`-makroa, joka tarkistaa, että `result`, joka saadaan kutsumalla `add` funktiolla arvoja `2` ja `2`, on yhtä kuin `4`. Tämä toimii esimerkkinä testin perusrakenteesta.

Ajetaan testit komennolla `cargo test`:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-01/output.txt}}
```

Testirunner näyttää, että yksi testi ajettiin ja se onnistui. 

### Tarkastellaan epäonnistunutta testiä

Lisätään testifunktio, joka tarkoituksella epäonnistuu:

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-03/src/lib.rs}}
```

Ajetaan testit uudelleen:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-03/output.txt}}
```

Testitulokset osoittavat, että `exploration` onnistui, mutta `another` epäonnistui, koska se paniikkiutui `panic!`-makron vuoksi. 

### Tulosten tarkistaminen `assert!`-makrolla

`assert!`-makro tarkistaa, että annettu ehto on `true`. Jos se ei ole, funktio aiheuttaa paniikin. Käytämme `assert!`-makroa varmistaaksemme, että `can_hold`-metodi toimii oikein:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-06/src/lib.rs:here}}
```

Kun tämä testi ajetaan, se onnistuu:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-06/output.txt}}
```

### `assert_eq!` ja `assert_ne!` -makrot

Usein haluamme varmistaa, että testattava funktio palauttaa juuri sen arvon, jonka odotamme. Tätä varten Rust tarjoaa `assert_eq!` ja `assert_ne!` -makrot:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-07/src/lib.rs}}
```

Tässä testissä `add_two(2)` palauttaa `4`, mikä vastaa odotettua arvoa.

Jos lisäisimme bugisen version:

```rust,not_desired_behavior,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/src/lib.rs:here}}
```

Testitulokset osoittaisivat virheen:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-04-bug-in-add-two/output.txt}}
```

Tämä auttaa nopeasti paikantamaan virheen.

### Testien mukauttaminen virheilmoituksilla

Voimme lisätä testivirheisiin mukautettuja viestejä käyttämällä `format!`-syntaksia:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/src/lib.rs:here}}
```

Nyt testitulokset sisältävät lisätietoja:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-07-custom-failure-message/output.txt}}
```

### Panicin odottaminen `should_panic`-attribuutilla

Jos haluamme varmistaa, että funktio **paniikkiutuu** tietyissä tilanteissa, voimme käyttää `#[should_panic]`-attribuuttia:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-08/src/lib.rs}}
```

Kun tämä testi ajetaan ja `Guess::new(200)` paniikkiutuu, testi onnistuu.

Voimme myös tarkentaa `should_panic`-attribuuttia, jotta se odottaa tiettyä virheviestiä:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-09/src/lib.rs:here}}
```

Jos virheviesti ei vastaa odotettua, testi epäonnistuu.

### `Result<T, E>` testifunktioissa

Testifunktiot voivat myös palauttaa `Result<T, E>` eikä vain paniikkiutua:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-10-result-in-tests/src/lib.rs:here}}
```

Tässä `Result<(), String>` mahdollistaa testin epäonnistumisen palauttamalla `Err`-arvon.

---

Tässä osiossa opimme:

- **Miten kirjoittaa testejä** Rustissa.
- **Miten tarkistaa tulokset** `assert!`, `assert_eq!` ja `assert_ne!` -makroilla.
- **Miten testata paniikin esiintymistä** `should_panic`-attribuutilla.
- **Miten käyttää `Result<T, E>` testien kirjoittamiseen**.

Seuraavaksi tarkastelemme, **miten testejä voidaan ajaa ja hallita tehokkaasti** Rustin testausjärjestelmässä!
