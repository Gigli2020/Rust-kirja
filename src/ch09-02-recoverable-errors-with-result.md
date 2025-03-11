## Palautettavat virheet `Result`-tyypillä

Useimmat virheet eivät ole niin vakavia, että ohjelma täytyisi pysäyttää kokonaan. Joskus funktion epäonnistuminen on odotettavissa ja voidaan käsitellä. Esimerkiksi jos yritämme avata tiedoston ja operaatio epäonnistuu, koska tiedostoa ei ole olemassa, voimme luoda tiedoston sen sijaan, että ohjelma kaatuisi.

Luvussa 2 käsittelimme lyhyesti `Result`-tyyppiä. Se on Rustin tapa käsitellä palautettavia virheitä, ja sen määrittely näyttää tältä:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Tässä `T` edustaa onnistuneen operaation palauttamaa arvoa ja `E` virheen tyyppiä. Koska `Result` on geneerinen, voimme käyttää sitä monissa eri tilanteissa, joissa onnistuneen arvon ja virheen tyyppi voi vaihdella.

Katsotaan esimerkkiä, jossa kutsumme funktiota, joka palauttaa `Result`-tyypin. Seuraava koodinpätkä yrittää avata tiedoston:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-03/src/main.rs}}
```

Tiedoston avaus (`File::open`) voi onnistua ja palauttaa `std::fs::File`-tyyppisen tiedostokahvan tai epäonnistua ja palauttaa `std::io::Error`-tyyppisen virheen.

### `match`-lausekkeen käyttö `Result`-arvon käsittelyyn

Voimme lisätä koodiin `match`-lausekkeen, joka käsittelee molemmat tapaukset:

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-04/src/main.rs}}
```

Tässä `match`-lauseke tarkistaa `Result`-arvon. Jos tiedoston avaus onnistuu (`Ok`-haara), saamme tiedostokahvan. Jos se epäonnistuu (`Err`-haara), ohjelma kaatuu `panic!`-makrolla. 

Jos ajamme tämän ohjelman ilman `hello.txt`-tiedostoa, saamme seuraavanlaisen virheilmoituksen:

```console
{{#include ../listings/ch09-error-handling/listing-09-04/output.txt}}
```

Tämä kertoo meille tarkasti, mikä meni pieleen.

### Erilaisten virheiden käsittely eri tavoin

Edellisessä esimerkissä ohjelma kaatui riippumatta siitä, miksi tiedoston avaus epäonnistui. Haluaisimme kuitenkin käsitellä eri virhetilanteet eri tavoin. Jos tiedostoa ei ole, voimme luoda sen, mutta jos tiedoston avaus epäonnistuu esimerkiksi käyttöoikeusongelman takia, ohjelman tulisi kaatua.

Voimme muokata koodia sisäkkäisellä `match`-lausekkeella:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-05/src/main.rs}}
```

Tässä `match` käsittelee `Err`-tapauksen erikseen `error.kind()`-funktion avulla. Jos virhe on `ErrorKind::NotFound`, yritämme luoda tiedoston. Jos tiedoston luonti epäonnistuu, ohjelma kaatuu. Kaikissa muissa virhetilanteissa ohjelma kaatuu heti.

> ### `match`-lausekkeelle vaihtoehtoiset ratkaisut
>
> `match` on hyödyllinen, mutta joskus pitkä ja monimutkainen. Rust tarjoaa useita metodeja `Result<T, E>`-tyypille, joilla `match`-lausekkeita voidaan lyhentää.
>
> Esimerkiksi `unwrap_or_else`-metodia käyttäen sama logiikka voidaan kirjoittaa lyhyemmin:
>
> ```rust,ignore
> use std::fs::File;
> use std::io::ErrorKind;
>
> fn main() {
>     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("Ongelmia tiedoston luonnissa: {error:?}");
>             })
>         } else {
>             panic!("Ongelmia tiedoston avaamisessa: {error:?}");
>         }
>     });
> }
> ```
>
> Tämä koodi tekee täsmälleen saman asian kuin aiempi `match`, mutta on helpompi lukea.

### Lyhennetyt virheenhallintametodit: `unwrap` ja `expect`

Jos haluamme yksinkertaisemman ratkaisun, voimme käyttää `unwrap`- tai `expect`-metodia. 

`unwrap` toimii kuten `match`: jos `Result` sisältää `Ok`-arvon, se palautetaan. Jos `Result` sisältää `Err`-arvon, ohjelma kaatuu:

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-04-unwrap/src/main.rs}}
```

Jos tiedostoa ei ole, saamme seuraavan virheilmoituksen:

```text
thread 'main' panicked at src/main.rs:4:49:
called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

Parempi vaihtoehto on `expect`, joka toimii kuten `unwrap`, mutta antaa meille mahdollisuuden määritellä oma virheilmoitus:

```rust,should_panic
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-05-expect/src/main.rs}}
```

Jos tiedostoa ei ole, saamme selkeämmän virheilmoituksen:

```text
thread 'main' panicked at src/main.rs:5:10:
hello.txt pitäisi olla tässä projektissa: Os { code: 2, kind: NotFound, message: "No such file or directory" }
```

### Virheen siirtäminen kutsuvalle funktiolle (`?`-operaattori)

Jos haluamme, että funktio **ei itse käsittele virhettä**, vaan siirtää sen eteenpäin kutsuvalle funktiolle, voimme käyttää `?`-operaattoria:

```rust
{{#include ../listings/ch09-error-handling/listing-09-07/src/main.rs:here}}
```

Tämä on lyhyempi tapa kirjoittaa `match`, ja se toimii seuraavasti:
- Jos `File::open("hello.txt")` palauttaa `Ok`, `?` purkaa arvon ja jatkaa suoritusta.
- Jos `File::open("hello.txt")` palauttaa `Err`, `?` palauttaa virheen kutsuvalle funktiolle.

Tätä voidaan lyhentää entisestään:

```rust
{{#include ../listings/ch09-error-handling/listing-09-09/src/main.rs:here}}
```

Tässä käytetään `fs::read_to_string`-funktiota, joka yhdistää kaikki vaiheet: avaa tiedoston, lukee sen sisällön ja palauttaa `String`.

### `?`-operaattorin rajoitukset

`?`-operaattoria voi käyttää **vain funktioissa, joiden palautustyyppi on `Result` tai `Option`**. Jos yritämme käyttää sitä funktiossa, joka palauttaa `()`, saamme käännösvirheen:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-10/src/main.rs}}
```

Tämä voidaan korjata joko käyttämällä `match`-lausetta tai muuttamalla `main`-funktion palautustyyppi `Result`-muotoon:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-12/src/main.rs}}
```

---

Tässä osiossa opimme käsittelemään palautettavia virheitä `Result`-tyypillä ja `?`-operaattorilla. Seuraavaksi tarkastelemme, milloin kannattaa käyttää `panic!`-makroa ja milloin `Result` on parempi vaihtoehto.
