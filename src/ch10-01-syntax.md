## Geneeriset tietotyypit

Käytämme geneerisiä tyyppejä määritellessämme funktioiden, rakenteiden ja muiden komponenttien määrityksiä, joita voimme käyttää monien erilaisten konkreettisten tietotyyppien kanssa. Tarkastelemme ensin, kuinka määritellä funktiot, rakenteet, luettelot (*enums*) ja metodit käyttämällä geneerisiä tyyppejä. Lopuksi keskustelemme, miten geneerisyys vaikuttaa koodin suorituskykyyn.

### Geneerisyys funktioissa

Kun määrittelemme geneeristä funktiota, lisäämme geneerisen parametrin funktion allekirjoitukseen. Tämä tekee koodistamme joustavampaa ja mahdollistaa laajemman käytön samalla, kun vältämme turhaa koodin kopiointia.

Jatkamme `largest`-funktion kehittämistä. Alla oleva Listing 10-4 esittää kaksi funktiota, jotka molemmat etsivät suurimman arvon viipaleesta:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

`largest_i32` etsii suurimman `i32`-arvon viipaleesta, ja `largest_char` tekee saman `char`-arvoille. Koska funktioiden runko on identtinen, voimme poistaa toiston yhdistämällä ne yhdeksi geneeriseksi funktioksi.

Voimme nimetä geneerisen tyypin kuten minkä tahansa parametrin, mutta Rustissa on tapana käyttää lyhyitä nimiä, usein vain yhtä kirjainta, kuten `T`. Rustin tyyppien nimeämiskäytännön mukaisesti käytämme **CamelCase**-muotoa.

Geneerisen funktion määrittelemiseksi lisäämme tyypin `T` kulmasulkeisiin `<>` funktion nimen jälkeen:

```rust,ignore
fn largest<T>(list: &[T]) -> &T {
```

Tämä tarkoittaa, että `largest` on geneerinen funktio, joka toimii minkä tahansa tyyppisen `T`-tyyppisen arvon kanssa. Se ottaa parametrina viittauksen `T`-tyyppisten arvojen viipaleeseen ja palauttaa viittauksen `T`-tyyppiseen arvoon.

Alla oleva Listing 10-5 yhdistää nämä funktiot yhdeksi geneeriseksi funktioksi:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

Tämä koodi ei vielä käänny, koska `T` voi olla mikä tahansa tyyppi, mutta `largest`-funktion runko edellyttää, että `T`:llä voi suorittaa vertailuoperaatioita. Ratkaisemme tämän lisäämällä `PartialOrd`-traitin, jota käsittelemme myöhemmin.

### Geneerisyys rakenteissa

Voimme myös määrittää rakenteita käyttämään geneerisiä tyyppiparametreja. Alla oleva Listing 10-6 määrittelee `Point<T>`-rakenteen, joka voi sisältää `x`- ja `y`-koordinaatit missä tahansa `T`-tyypissä:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

Tämä tarkoittaa, että sekä `x` että `y` ovat samaa `T`-tyyppiä. Jos yritämme määritellä `Point<T>`-instanssin eri tyyppisillä `x`- ja `y`-arvoilla, saamme virheen:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

Voimme kuitenkin käyttää kahta eri geneeristä tyyppiä `T` ja `U`, jolloin `x` ja `y` voivat olla eri tyyppiä:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

### Geneerisyys luetteloissa

Samoin kuin rakenteiden kohdalla, voimme määritellä geneerisiä luetteloita (*enums*). Rustin standardikirjaston `Option<T>`-luettelo on hyvä esimerkki:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Tässä `Option<T>` voi sisältää minkä tahansa tyyppisen arvon `T`, mutta voi myös edustaa puuttuvaa arvoa `None`.

Rustin `Result<T, E>`-tyyppi on toinen esimerkki, jossa käytetään kahta geneeristä tyyppiä `T` ja `E`:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Tämä mahdollistaa eri tyyppisten onnistuneiden tulosten ja virheiden hallinnan samassa tietorakenteessa.

### Geneerisyys metodeissa

Voimme myös määritellä metodeja, jotka käyttävät geneerisiä tyyppejä:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

Tässä `x`-metodi palauttaa `x`-kentän arvon `Point<T>`-rakenteesta.

Jos haluamme, että metodi toimii vain tietyllä tyypillä, voimme määrittää sen vain `Point<f32>`-tyypille:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

Voimme myös käyttää metodeissa eri geneerisiä tyyppejä kuin rakenteessa:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

Tässä `mixup`-metodi ottaa parametrina toisen `Point`-instanssin ja palauttaa uuden `Point`-rakenteen, jossa `x` tulee `self`-instanssista ja `y` toisesta `Point`-instanssista.

### Geneerisen koodin suorituskyky

Saatat miettiä, vaikuttaako geneeristen tyyppien käyttö ohjelman suorituskykyyn. Rust käyttää **monomorfisointia** (*monomorphization*), joka tarkoittaa, että geneerinen koodi muunnetaan konkreettiseksi koodiksi käännösaikana. 

Esimerkiksi tämä koodi:

```rust
let integer = Some(5);
let float = Some(5.0);
```

Muunnetaan käännösaikana seuraavan kaltaiseksi:

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

Koska Rust generoi erilliset versiot kullekin tyypille, geneerisyydestä ei aiheudu suorituskykyhaittaa.

Seuraavaksi tutkimme **traitteja**, joilla voidaan rajoittaa, millä tyypeillä geneerisiä funktioita ja rakenteita voidaan käyttää.
