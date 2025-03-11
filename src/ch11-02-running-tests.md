## Testien suorittamisen hallinta

Samalla tavalla kuin `cargo run` kääntää koodisi ja ajaa lopullisen binääritiedoston, `cargo test` kääntää koodisi **testitilassa** ja suorittaa testiohjelman. Oletuksena `cargo test` suorittaa kaikki testit **rinnakkain** ja kaappaa testien aikana syntyneen tulosteen, jotta testitulosten lukeminen on helpompaa. Voit kuitenkin muuttaa tätä käyttäytymistä antamalla komentoriviparametreja.

Jotkut argumentit menevät suoraan `cargo test` -komennolle, ja toiset menevät testiohjelmalle. Erottamaan nämä toisistaan, listaa ensin `cargo test` -parametrit, lisää `--`-erotin ja sitten testiohjelman parametrit. Voit tarkastella `cargo test` -valintoja suorittamalla:

```console
$ cargo test --help
```

ja testiohjelman valintoja suorittamalla:

```console
$ cargo test -- --help
```

Näitä vaihtoehtoja on myös dokumentoitu [Rustc-kirjan testausosiossa](https://doc.rust-lang.org/rustc/tests/index.html).

### Testien suorittaminen rinnakkain tai peräkkäin

Oletuksena useita testejä ajetaan **rinnakkain** säikeitä hyödyntäen, jotta testit valmistuvat nopeammin. Koska testit suoritetaan samanaikaisesti, varmista, että testit eivät **riipu toisistaan** eivätkä jaetusta tilasta, kuten nykyisestä hakemistosta tai ympäristömuuttujista.

Jos esimerkiksi jokainen testi kirjoittaa tiedostoon `_test-output.txt_`, mutta jokainen testi odottaa eri sisältöä, testit voivat **häiritä toisiaan** rinnakkaissuorituksessa. Yksi ratkaisu on kirjoittaa eri tiedostoihin, toinen on suorittaa testit **yksi kerrallaan**.

Voit määrittää testisäikeiden määrän `--test-threads`-lipulla:

```console
$ cargo test -- --test-threads=1
```

Tällöin testit suoritetaan **yksi kerrallaan**, jolloin ne eivät vaikuta toisiinsa. Tämä vie enemmän aikaa kuin rinnakkainen suoritus.

### Tulosteen näyttäminen

Oletuksena Rustin testikirjasto **kaappaa** kaiken `println!`-tulosteen onnistuneilta testeilta. Jos testi epäonnistuu, tuloste näytetään virheilmoituksen yhteydessä.

Esimerkiksi seuraava testiohjelma:

```rust,panics,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-10/src/lib.rs}}
```

tuottaa seuraavan tulosteen:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-10/output.txt}}
```

Huomaa, että `I got the value 4` -tulostetta **ei näy**, koska testi onnistui. Sen sijaan epäonnistuneen testin `I got the value 8` näkyy.

Jos haluat nähdä **kaikkien testien tulosteen**, käytä `--show-output`-lippua:

```console
$ cargo test -- --show-output
```

Tällöin saat kaikki `println!`-tulosteet riippumatta testin lopputuloksesta.

### Testien osajoukon suorittaminen nimen perusteella

Jos sinulla on paljon testejä ja haluat ajaa vain tietyn osan niistä, voit antaa `cargo test` -komennolle yksittäisen testin **nimen** tai osan siitä.

Esimerkiksi meillä voi olla seuraavat testit:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/listing-11-11/src/lib.rs}}
```

Kaikkien testien suorittaminen:

```console
{{#include ../listings/ch11-writing-automated-tests/listing-11-11/output.txt}}
```

#### Yksittäisen testin ajaminen

Voit ajaa vain yhden testin antamalla sen nimen:

```console
$ cargo test one_hundred
```

Tulostus:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-02-single-test/output.txt}}
```

Huomaa, että vain `one_hundred` ajettiin, ja muut testit **suodatettiin pois**.

#### Useiden testien ajaminen suodattamalla

Voit myös suodattaa testejä osittaisella nimellä:

```console
$ cargo test add
```

Tämä ajaa **kaikki testit, joiden nimessä on "add"**, ja suodattaa pois muut:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-03-multiple-tests/output.txt}}
```

Lisäksi voit suorittaa **kaikki testit tietystä moduulista** suodattamalla moduulin nimen perusteella.

### Tiettyjen testien ohittaminen

Jos tietyt testit ovat **aikaavieviä**, voit merkitä ne `#[ignore]`-attribuutilla, jolloin ne **eivät ajeta oletuksena**:

```rust,noplayground
{{#rustdoc_include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/src/lib.rs:here}}
```

Kun suoritat `cargo test`, `expensive_test` merkitään ohitetuksi:

```console
{{#include ../listings/ch11-writing-automated-tests/no-listing-11-ignore-a-test/output.txt}}
```

Voit suorittaa **vain ohitetut testit** komennolla:

```console
$ cargo test -- --ignored
```

Tulostus:

```console
{{#include ../listings/ch11-writing-automated-tests/output-only-04-running-ignored/output.txt}}
```

Jos haluat ajaa **kaikki testit** (mukaan lukien ohitetut), käytä:

```console
$ cargo test -- --include-ignored
```

Tällä tavoin voit nopeuttaa testejä keskittymällä oleellisiin testejä kehitystyön aikana ja ajaa pidemmät testit vain tarvittaessa.

---

## Yhteenveto

Tässä osiossa opimme:

- **Kuinka hallita testien rinnakkaisuutta** `--test-threads`-valinnalla.
- **Miten näyttää testitulosteet myös onnistuneista testeistä** `--show-output`-valinnalla.
- **Kuinka suorittaa yksittäisiä testejä** nimeämällä ne `cargo test` -komennossa.
- **Kuinka merkitä testit ohitettaviksi** `#[ignore]`-attribuutilla ja suorittaa ne tarvittaessa.

Seuraavaksi tutkimme **integraatiotestien** käyttöä testikoodin järjestämiseen tehokkaasti!
