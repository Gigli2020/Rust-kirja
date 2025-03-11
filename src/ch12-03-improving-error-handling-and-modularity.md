## Refaktorointi: parempi modulariteetti ja virheenkäsittely

Parantaaksemme ohjelmaamme korjaamme neljä rakennetta ja virheenkäsittelyä koskevaa ongelmaa. 

1. **`main`-funktio hoitaa liikaa tehtäviä**: Se käsittelee sekä argumenttien jäsentämisen että tiedostojen lukemisen. Kun ohjelma kasvaa, `main`:in tehtävät lisääntyvät, jolloin se on vaikeampi ymmärtää, testata ja ylläpitää.
2. **Konfiguraatioarvot ja ohjelman logiikka sekoittuvat**: `query` ja `file_path` ovat asetuksia, mutta `contents` liittyy ohjelman suorittamiseen. On parempi erottaa konfiguraatio omaksi rakenteekseen.
3. **Virheilmoitukset ovat epämääräisiä**: Käytämme `expect`-kutsua tiedostonlukemisen epäonnistumiseen, mutta viesti ei kerro käyttäjälle, mistä virhe johtuu (esim. puuttuva tiedosto tai puuttuvat oikeudet).
4. **Indeksivirheet komentoriviargumenteissa**: Jos käyttäjä ei anna tarpeeksi argumentteja, saadaan epäselvä *index out of bounds* -virhe. Virheiden käsittelyn tulisi olla yhdessä paikassa, jotta ohjelman käyttäjät saavat selkeämmät viestit.

Seuraavaksi refaktoroimme ohjelman korjataksemme nämä ongelmat.

### Vastuujen erottelu binääriohjelmissa

Rust-yhteisössä binääriohjelmat yleensä jaetaan kahteen tiedostoon:

- **_main.rs_**: Sisältää vain ohjelman käynnistyksen ja virheenkäsittelyn.
- **_lib.rs_**: Sisältää kaiken liiketoimintalogiikan.

**Main-funktion vastuut:**

- Kutsuu komentoriviargumenttien käsittelyä.
- Alustaa konfiguraation.
- Kutsuu `run`-funktiota (_lib.rs_:ssä).
- Käsittelee mahdolliset virheet.

Tämä mahdollistaa **testien kirjoittamisen liiketoimintalogiikalle**, koska Rust ei salli suoraa `main`-funktion testaamista.

#### Argumenttien jäsentämisen erottaminen

Ensimmäinen vaihe on siirtää komentoriviargumenttien käsittely erilliseen funktioon. **Listing 12-5** näyttää uuden `parse_config`-funktion.

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-05/src/main.rs:here}}
```

Nyt `main` vain **välittää** argumentit `parse_config`-funktiolle, joka palauttaa `query`- ja `file_path`-arvot.

#### Konfiguraatioarvojen ryhmittely

Seuraava askel on ryhmittää nämä arvot `Config`-rakenteeseen:

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-06/src/main.rs:here}}
```

Tämä selkeyttää koodia: `query` ja `file_path` kuuluvat yhteen.

#### Konstruktorin (`new`) lisääminen `Config`-rakenteelle

Voimme parantaa koodia muuttamalla `parse_config`-funktion metodiksi:

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-07/src/main.rs:here}}
```

Nyt voimme luoda `Config`-instanssin kutsumalla `Config::new(args)`, mikä on **idiomaattisempaa Rustia**.

### Virheenkäsittelyn parantaminen

Jos käyttäjä ei anna tarpeeksi argumentteja, ohjelma kaatuu epäselvällä virheellä:

```console
{{#include ../listings/ch12-an-io-project/listing-12-07/output.txt}}
```

Lisätään tarkistus, joka antaa selkeämmän virheilmoituksen:

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-08/src/main.rs:here}}
```

Nyt ohjelma antaa ymmärrettävän virheilmoituksen.

#### `panic!`-kutsun korvaaminen `Result`-palautusarvolla

Parempi tapa on käyttää `Result`-tyyppiä, joka palauttaa joko `Config`-instanssin tai virheviestin:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-09/src/main.rs:here}}
```

`Config::build` palauttaa `Err`-arvon, jos argumentteja ei ole tarpeeksi.

#### `Config::build`-metodin käsittely `main`-funktiossa

Lisäämme `unwrap_or_else`-kutsun `main`-funktioon käsittelemään virheen:

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-10/src/main.rs:here}}
```

Nyt ohjelma lopetetaan **siististi** virhekoodilla `process::exit(1)`, mikä on standardikäytäntö.

### `run`-funktion erottaminen

Seuraavaksi siirrämme päälogiikan erilliseen `run`-funktioon:

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-11/src/main.rs:here}}
```

Tämä erottaa **konfiguraation käsittelyn** ja **tiedoston käsittelyn**, mikä tekee `main`-funktiosta selkeämmän.

#### `run`-funktion virheenkäsittely

`run`-funktion virheiden käsittelyä voidaan parantaa käyttämällä `Result`-tyyppiä:

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-12/src/main.rs:here}}
```

Rust varoittaa, että `Result` jätetään huomiotta, joten lisätään virheenkäsittely `main`-funktioon:

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-01-handling-errors-in-main/src/main.rs:here}}
```

`if let Err(e)` -lohko tulostaa virheilmoituksen ja päättää ohjelman virhekoodilla.

### Koodin siirtäminen `lib.rs`-tiedostoon

Lopuksi siirrämme kaiken muun paitsi `main`-funktion koodin **_src/lib.rs_-tiedostoon**:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-13/src/lib.rs:here}}
```

Tämä mahdollistaa **modulaarisuuden ja testattavuuden**.

### `main.rs`-tiedoston päivittäminen

`main.rs` viittaa nyt `minigrep`-kirjastoon:

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-14/src/main.rs:here}}
```

Kaikki liiketoimintalogiikka on nyt `lib.rs`:ssä, ja `main.rs` vain **kutsuu sen** ja käsittelee virheet.

### Yhteenveto

Tässä refaktoroinnissa saavutimme:

- **Selkeämmän koodin**: Erottelimme argumenttien käsittelyn, liiketoimintalogiikan ja virheenkäsittelyn.
- **Paremmat virheilmoitukset**: Ohjelma ei enää kaadu epäselvillä viesteillä.
- **Modulaarisuuden**: Pystymme nyt **testaamaan** ohjelman osia erikseen.

Seuraavaksi hyödynnämme tätä rakennetta lisäämällä **testejä ohjelmalle**!

[ch13]: ch13-00-functional-features.html
[ch9-error-guidelines]: ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling
[ch9-result]: ch09-02-recoverable-errors-with-result.html
