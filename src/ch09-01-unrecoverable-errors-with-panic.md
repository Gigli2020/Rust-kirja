## Palauttamattomat virheet `panic!`-makrolla

Joskus koodissa tapahtuu jotain, mille emme voi tehdä mitään. Näissä tapauksissa Rust tarjoaa `panic!`-makron. `panic!` voidaan laukaista kahdella tavalla: joko suorittamalla virheellinen toiminto (kuten yrittämällä käyttää taulukon rajojen ulkopuolista indeksiä) tai kutsumalla `panic!`-makroa suoraan.

Molemmissa tapauksissa ohjelma **pysäytetään** ja tapahtuu paniikkitila (*panic state*). Oletusarvoisesti tämä tulostaa virheviestin, purkaa pinoon kertyneet kutsut (*stack unwinding*) ja lopettaa ohjelman. Ympäristömuuttujan avulla voidaan myös näyttää pinojäljitys (*backtrace*), mikä auttaa selvittämään, mistä virhe johtuu.

> ### Pinon purkaminen (`unwinding`) vs. ohjelman välitön keskeyttäminen (`abort`)
>
> Kun `panic!` tapahtuu, Rust suorittaa oletusarvoisesti **pinon purkamisen** (*unwinding*), eli käy funktiokutsut taaksepäin ja siivoaa resurssit. Tämä voi kuitenkin olla raskasta. Vaihtoehtoisesti voit määrittää ohjelman **keskeyttämään suoraan** (*abort*), jolloin Rust lopettaa ohjelman ilman siivousta.
>
> Keskeytystilan avulla ohjelman käyttämä muisti siivotaan käyttöjärjestelmän toimesta. Jos haluat tehdä binääritiedostosta mahdollisimman pienen, voit vaihtaa purkamisen keskeytykseen lisäämällä seuraavan asetuksen _Cargo.toml_-tiedostoon:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

### `panic!`-makron kutsuminen

Tarkastellaan yksinkertaista ohjelmaa, joka kutsuu `panic!`-makroa:

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-01-panic/src/main.rs}}
```

Kun ohjelma ajetaan, tulos näyttää tältä:

```console
{{#include ../listings/ch09-error-handling/no-listing-01-panic/output.txt}}
```

Viimeiset kaksi riviä sisältävät `panic!`-viestin ja virheen sijainnin (_src/main.rs:2:5_ tarkoittaa _src/main.rs_-tiedoston toista riviä, viidettä merkkiä).

Jos `panic!` tapahtuu kirjastokoodissa, virheviesti voi näyttää tuntemattomalta. Voimme käyttää **pinojäljitystä** (*backtrace*) selvittääksemme, missä virhe tapahtui.

### `panic!` ja pinojäljitys

Katsotaan esimerkkiä, jossa ohjelma yrittää käyttää taulukon rajojen ulkopuolista indeksiä:

```rust,should_panic,panics
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-01/src/main.rs}}
```

Tässä yritämme käyttää **100. alkiota**, mutta vektorissa on vain **kolme** alkiota. Tämä johtaa `panic!`-tilaan, koska Rust ei voi palauttaa kelvollista arvoa. Toisin kuin C-kielessä, jossa muistialueen ulkopuolinen luku voi aiheuttaa **määrittelemätöntä käyttäytymistä** (*undefined behavior*), Rust estää tällaiset tilanteet pysäyttämällä ohjelman.

Jos ajamme ohjelman, virhe näyttää tältä:

```console
{{#include ../listings/ch09-error-handling/listing-09-01/output.txt}}
```

Virheilmoitus osoittaa virheellisen indeksin käytön ohjelman _main.rs_-tiedoston **rivillä 4**.

Rust tarjoaa **pinojäljityksen**, jonka avulla voi nähdä, mitkä funktiot johtivat `panic!`-kutsuun. Voit ottaa sen käyttöön asettamalla ympäristömuuttujan `RUST_BACKTRACE=1`:

```console
$ RUST_BACKTRACE=1 cargo run
```

Tuloste näyttää jotain tämän kaltaista:

```console
stack backtrace:
   0: rust_begin_unwind
             at /rustc/.../library/std/src/panicking.rs:692:5
   1: core::panicking::panic_fmt
             at /rustc/.../library/core/src/panicking.rs:75:14
   2: core::panicking::panic_bounds_check
             at /rustc/.../library/core/src/panicking.rs:273:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/.../library/core/src/slice/index.rs:274:10
   4: panic::main
             at ./src/main.rs:4:6
```

Pinojäljityksessä tärkein rivi on **src/main.rs:4:6**, koska se osoittaa virheen sijainnin omassa koodissamme. Tämän tiedon avulla voimme korjata virheen estämällä taulukon rajojen ylityksen.

Rust estää useita muistivirheitä, joita esiintyy muissa kielissä, kuten **buffer overread** -haavoittuvuuksia. Jos yritämme käyttää olematonta taulukon kohtaa, Rust **pysäyttää ohjelman**, jotta virhe ei aiheuttaisi tietoturvaongelmia.

---

Seuraavaksi tarkastelemme, miten **palautettavat virheet** voidaan käsitellä käyttäen `Result<T, E>`-tyyppiä.
