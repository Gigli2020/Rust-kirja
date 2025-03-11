## Ympäristömuuttujien käyttö

Parannamme `minigrep`-ohjelmaa lisäämällä uuden ominaisuuden: mahdollisuuden
suorittaa **kirjainkoon huomioimaton haku** ympäristömuuttujan avulla.
Voisimme toteuttaa tämän myös komentoriviargumenttina, mutta ympäristömuuttujan
käyttö antaa käyttäjälle mahdollisuuden asettaa kerran `IGNORE_CASE`-muuttuja
ja käyttää sitä kaikissa hauissa saman istunnon aikana.

### Kirjoitetaan epäonnistuva testi `search_case_insensitive`-funktiolle

Lisäämme uuden `search_case_insensitive`-funktion, joka suorittaa
kirjainkoon huomioimattoman haun. Jatkamme testivetoista kehitystä (TDD),
joten ensimmäinen askel on kirjoittaa epäonnistuva testi. Lisäksi nimeämme
olemassa olevan testin `one_result` uudelleen muotoon `case_sensitive`,
jotta testien erot ovat selvemmät. **Listing 12-20** näyttää uuden testin:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-20/src/lib.rs:here}}
```

Muokkasimme myös aiemman testin `contents`-arvoa lisäämällä uuden rivin
`"Duct tape."`, jossa `D` on isolla. Tämä varmistaa, että aiemmin toteutettu
kirjainkoon huomioiva haku ei vahingossa rikkoudu. Sen tulisi edelleen löytää
vain `"duct"` eikä `"Duct"`-muotoa.

Uusi testi käyttää hakusanana `"rUsT"`, joka `search_case_insensitive`-funktion
tulee löytää sekä `"Rust:"`-rivin että `"Trust me."`-rivin, huolimatta siitä,
että niiden kirjainkoko eroaa kyselausekkeesta.

Tämä testi epäonnistuu nyt käännöksessä, koska `search_case_insensitive`-funktiota
ei ole vielä määritelty. Voit halutessasi lisätä sille väliaikaisen toteutuksen,
joka palauttaa tyhjän vektorin, kuten teimme `search`-funktiolle **Listing 12-16**,
ja tarkistaa, että testi kääntyy mutta epäonnistuu.

### `search_case_insensitive`-funktion toteutus

`search_case_insensitive`-funktio toimii lähes samalla tavalla kuin `search`.
Ainoa ero on, että se **muuttaa sekä hakulausekkeen että jokaisen rivin
pieniksi kirjaimiksi** ennen vertailua. **Listing 12-21** näyttää tämän:

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-21/src/lib.rs:here}}
```

Ensimmäisenä muutamme `query`-arvon pieniksi kirjaimiksi ja varastoimme sen
uuteen muuttujaan **varjostaen** alkuperäisen muuttujan. `to_lowercase`-kutsun
käyttö varmistaa, että hakusana `"rust"`, `"RUST"`, `"Rust"` tai `"rUsT"`
tulkitaan aina samaksi `"rust"`-arvoksi.

Koska `to_lowercase` palauttaa uuden `String`-arvon (eikä muokkaa olemassa olevaa
viipaletta), meidän täytyy nyt käsitellä `query`-muuttujaa `String`-tyyppinä
eikä viipaleena (`&str`).

Seuraavaksi muutamme myös `line`-muuttujan pieniksi kirjaimiksi. Nyt sekä
hakusana että hakutulokset ovat pienaakkosiksi muunnettuja, ja voimme löytää
osumia riippumatta alkuperäisestä kirjainkoosta.

Testataan toiminnallisuutta:

```console
{{#include ../listings/ch12-an-io-project/listing-12-21/output.txt}}
```

Testit onnistuvat! Nyt voimme kutsua `search_case_insensitive`-funktiota
`run`-funktiosta.

### Konfigurointikentän lisääminen `Config`-rakenteeseen

Lisäämme `Config`-rakenteeseen uuden kentän `ignore_case`, joka kertoo,
tuleeko haku suorittaa kirjainkoon huomioivana vai ei.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:here}}
```

`ignore_case`-kenttä on `bool`-tyyppinen. Seuraavaksi `run`-funktio tarkistaa
tämän arvon ja valitsee käytettävän hakufunktion **Listing 12-22** mukaisesti:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-22/src/lib.rs:there}}
```

Tämä ei vielä käänny, koska emme ole alustaneet `ignore_case`-kenttää.

### Ympäristömuuttujan tarkistaminen

Lopuksi lisätään `IGNORE_CASE`-ympäristömuuttujan tarkistus. **Listing 12-23**
esittää tämän muutoksen:

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-23/src/lib.rs:here}}
```

Tässä luomme uuden muuttujan `ignore_case` ja asetamme sen arvon kutsumalla
`env::var`-funktiota `IGNORE_CASE`-ympäristömuuttujalle. Jos muuttuja on
asetettu, `env::var` palauttaa `Ok`, ja `is_ok`-kutsu palauttaa `true`,
mikä tarkoittaa, että haun tulee olla kirjainkoon huomioimaton.

Jos muuttujaa **ei ole asetettu**, `env::var` palauttaa `Err`, jolloin
`is_ok`-kutsu palauttaa `false` ja ohjelma suorittaa **kirjainkoon huomioivan haun**.

Lopuksi välitämme `ignore_case`-arvon `Config`-instanssille, jotta `run`-funktio
voi valita oikean hakutavan.

### Testaaminen ympäristömuuttujalla

Kokeillaan ensin hakua ilman ympäristömuuttujaa:

```console
{{#include ../listings/ch12-an-io-project/listing-12-23/output.txt}}
```

Kaikki toimii odotetusti! Nyt asetetaan `IGNORE_CASE=1` ja kokeillaan uudelleen:

```console
$ IGNORE_CASE=1 cargo run -- to poem.txt
```

PowerShellissä ympäristömuuttujan asettaminen vaatii erillisen komennon:

```console
PS> $Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

Poista muuttuja käytöstä PowerShellissä näin:

```console
PS> Remove-Item Env:IGNORE_CASE
```

Tuloksen pitäisi näyttää tältä:

```console
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

Nyt `minigrep` voi suorittaa kirjainkoon huomioimattomia hakuja ympäristömuuttujan avulla!
Voimme myös yhdistää **komentoriviargumentit ja ympäristömuuttujat**, jolloin ohjelma
voi antaa etusijan jommallekummalle riippuen siitä, kumpi on asetettu.

Rustin `std::env`-moduuli tarjoaa monia muita hyödyllisiä ympäristömuuttujien hallinnan
ominaisuuksia – tutustu sen dokumentaatioon oppiaksesi lisää!
