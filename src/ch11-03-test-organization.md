## Testien organisointi

Kuten tämän luvun alussa mainittiin, testaus on laaja aihe, ja eri kehittäjät käyttävät eri termejä ja tapoja testien järjestämiseen. Rust-yhteisössä testit jaetaan kahteen pääkategoriaan: **yksikkötestit** (*unit tests*) ja **integraatiotestit** (*integration tests*).

- **Yksikkötestit** ovat pieniä ja keskittyvät yhden moduulin testaamiseen erillään muusta koodista. Ne voivat testata myös yksityisiä rajapintoja.
- **Integraatiotestit** testaavat koodin julkista rajapintaa samaan tapaan kuin ulkopuolinen koodi käyttäisi sitä. Ne voivat testata useiden moduulien yhteistoimintaa.

Molemmat testityypit ovat tärkeitä, jotta voidaan varmistaa, että ohjelman eri osat toimivat sekä itsenäisesti että yhdessä odotetulla tavalla.

### Yksikkötestit

Yksikkötestien tarkoitus on testata koodin yksittäisiä osia erillään muusta ohjelmasta. Tämä auttaa nopeasti tunnistamaan, missä koodi toimii ja missä ei. Yksikkötestit sijoitetaan **_src_-hakemistoon** ja samalle tiedostolle, jossa testattava koodi on. Käytännön mukaisesti jokaisessa tiedostossa, jossa on testejä, luodaan **`tests`-moduuli**, joka sisältää testifunktiot, ja moduuli merkitään `#[cfg(test)]`-attribuutilla.

#### `#[cfg(test)]` ja testimoduuli

Attribuutti `#[cfg(test)]` kertoo Rustille, että testimoduuli tulee kääntää ja suorittaa **vain, kun ajetaan `cargo test`**, ei kun ajetaan `cargo build`. Tämä nopeuttaa kääntämistä ja pienentää lopullisen binääritiedoston kokoa, koska testikoodi ei sisällytetä tuotantokoodiin.

Kun loimme uuden `adder`-kirjastoprojektin, Cargo generoi _src/lib.rs_-tiedostoon seuraavan koodin:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-01/src/lib.rs}}
```

Moduulin `#[cfg(test)]`-attribuutti kertoo, että seuraava osa koodista sisällytetään vain, jos koodi käännetään testejä varten. Tämä pätee sekä testifunktioihin että mahdollisiin apufunktioihin testimoduulissa.

#### Yksityisten funktioiden testaaminen

Joissakin ohjelmointikielissä yksityisten funktioiden testaaminen on vaikeaa tai jopa mahdotonta. Rustissa yksityisten funktioiden testaaminen on mahdollista, koska testit ovat vain normaalia Rust-koodia, ja ne voivat käyttää vanhempien moduuliensa kohteita.

Esimerkiksi alla oleva _Listing 11-12_ sisältää yksityisen funktion `internal_adder`, jota testataan testimoduulissa:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

Tässä `internal_adder`-funktio ei ole `pub`, mutta se voidaan testata käyttämällä `use super::*` -määritystä, joka tuo vanhemman moduulin sisällön testimoduulin käyttöön.

### Integraatiotestit

Rustissa integraatiotestit sijoitetaan erilliseen **_tests_-hakemistoon**. Tämä tarkoittaa, että ne käyttävät kirjastoa kuten mikä tahansa ulkopuolinen koodi: ne voivat kutsua vain **julkisia** rajapintoja. Integraatiotestien tarkoitus on varmistaa, että ohjelman eri osat toimivat yhdessä oikein.

#### _tests_-hakemisto

Luomme **_tests_-hakemiston** projektin juurihakemistoon, samaan tasoon kuin _src_-hakemisto. Cargo osaa automaattisesti etsiä ja suorittaa kaikki tämän hakemiston tiedostot integraatiotesteinä.

Oletetaan, että _src/lib.rs_-tiedostossa on seuraava koodi (_Listing 11-12_):

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-12/src/lib.rs}}
```

Luo _tests_-hakemisto ja sinne tiedosto _integration_test.rs_, jolloin projektin hakemistorakenne näyttää tältä:

```text
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

Kirjoita _tests/integration_test.rs_-tiedostoon seuraava koodi (_Listing 11-13_):

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-13/tests/integration_test.rs}}
```

Koska _tests_-hakemiston jokainen tiedosto käännetään **omana pakettinaan** (*crate*), meidän täytyy erikseen tuoda kirjaston julkiset funktiot käyttöön:

```rust,ignore
use adder::add_two;
```

Testejä ei tarvitse merkitä `#[cfg(test)]`-attribuutilla, koska Cargo osaa käsitellä _tests_-hakemiston tiedostoja oikein.

Ajetaan testit:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-13/output.txt}}
```

Tulosteessa näkyvät erikseen:

- **Yksikkötestit**, jotka ovat _src/lib.rs_:n testimoduulissa.
- **Integraatiotestit**, jotka ovat _tests_-hakemistossa.
- **Dokumentaatiotestit** (*doc tests*), jotka suoritetaan automaattisesti, jos dokumentaatiossa on esimerkkikoodia.

Jos **yksikkötesti epäonnistuu**, integraatiotestejä ei ajeta.

### Integraatiotestien hallinta

#### Yksittäisen integraatiotestin ajaminen

Voit ajaa vain tietyn integraatiotestin antamalla sen tiedoston nimen `--test`-lipulla:

```console
$ cargo test --test integration_test
```

Tämä suorittaa vain _tests/integration_test.rs_-tiedoston testit.

#### Jaetun koodin käyttö integraatiotesteissä

Jos haluat käyttää **samoja apufunktioita** useissa integraatiotesteissä, älä sijoita niitä suoraan _tests_-hakemistoon, koska Cargo käsittelee jokaisen tiedoston erillisenä pakettina.

Jos esimerkiksi luomme _tests/common.rs_-tiedoston ja lisäämme sinne `setup`-funktion:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/tests/common.rs}}
```

Näemme testituloksissa erillisen **"common"**-osion:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-12-shared-test-code-problem/output.txt}}
```

Tämän estämiseksi käytä **_tests/common/mod.rs_** -nimeämiskäytäntöä:

```text
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs
```

Nyt _tests/integration_test.rs_ voi käyttää `setup`-funktiota:

```rust,ignore
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-13-fix-shared-test-code-problem/tests/integration_test.rs}}
```

---

## Yhteenveto

Rustin testausjärjestelmä mahdollistaa:

- **Yksikkötestit**, jotka tarkastelevat koodin osia erikseen.
- **Integraatiotestit**, jotka testaavat ohjelman osien yhteistoimintaa.
- **Testien hallinnan ja suodattamisen**, mikä tekee testauksesta tehokasta.

Seuraavaksi yhdistämme opitun ja sovellamme sitä **käytännön projektissa**!
