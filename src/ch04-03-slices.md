## Viipaleet

_Viipaleiden_ avulla voit viitata peräkkäisiin alkioihin [kokoelmassa](ch08-00-common-collections.md) ilman, että tarvitset koko kokoelmaa. Viipale on eräänlainen viittaus, joten sillä ei ole omistajuutta.

### Merkkijonojen käsittely viipaleilla

Tarkastellaan ohjelmointiongelmaa: kirjoita funktio, joka ottaa merkkijonon, jossa sanat on erotettu välilyönneillä, ja palauttaa ensimmäisen sanan. Jos välilyöntejä ei löydy, koko merkkijono on yksi sana, ja koko merkkijono tulisi palauttaa.

Kokeillaan ensin toteuttaa funktio ilman viipaleita:

```rust,ignore
fn first_word(s: &String) -> ?
```

Koska emme tarvitse omistajuutta, voimme ottaa `&String`-viittauksen parametrina. Mutta miten meidän tulisi palauttaa ensimmäinen sana? Voisimme palauttaa indeksin, joka osoittaa sanan loppuun välilyönnin kohdalla:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-07/src/main.rs:here}}
```

Tämä ratkaisu toimii, mutta siinä on ongelma: palautamme `usize`-indeksin, mutta se ei ole sidottu `String`-arvoon. Tämä voi johtaa virheisiin, jos `String` muuttuu sen jälkeen, kun olemme tallentaneet indeksin.

Tämän ongelman ratkaisemiseksi Rust tarjoaa _merkkijonoviipaleet_.

### Merkkijonoviipaleet

_Merkkijonoviipale_ on viittaus `String`-osaan:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-17-slice/src/main.rs:here}}
```

Tässä `hello` on viittaus osaan `s`-merkkijonosta. Syntaksi `[0..5]` tarkoittaa, että viipale alkaa indeksistä 0 ja päättyy indeksiin 5 (mutta ei sisällä sitä).

Viipaleiden avulla voimme kirjoittaa `first_word`-funktion turvallisemmin:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-18-first-word-slice/src/main.rs:here}}
```

Nyt `first_word` palauttaa viipaleen `&str`, joka on sidottu `String`-arvoon. Tämä tarkoittaa, että emme voi vahingossa käyttää virheellistä indeksiä, koska kääntäjä estää sen.

Rust estää myös koodin, jossa yrittäisimme muokata merkkijonoa viittauksen ollessa vielä käytössä:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-19-slice-error/src/main.rs:here}}
```

Kääntäjä estää tämän virheen jo käännösaikana, mikä tekee Rustista turvallisemman kielen.

### Merkkijonolitteraalit ja viipaleet

Merkkijonolitteraalit (`"Hello, world!"`) ovat itse asiassa viipaleita (`&str`), jotka osoittavat ohjelman binaaritiedostoon tallennettuun merkkijonoon. Tämä selittää, miksi ne ovat muuttumattomia.

### Viipaleet muille kokoelmille

Merkkijonoviipaleiden lisäksi voimme käyttää viipaleita myös taulukoille:

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

Tämän viipaleen tyyppi on `&[i32]`, ja se toimii samalla tavalla kuin merkkijonoviipaleet.

### Yhteenveto

Omistajuus, lainaaminen ja viipaleet varmistavat Rust-ohjelmien muistiturvallisuuden jo käännösvaiheessa. Rust tarjoaa muistinhallinnan, jossa omistaja vapauttaa datan automaattisesti, kun se poistuu näkyvyysalueelta. Tämä poistaa tarpeen kirjoittaa ylimääräistä koodia muistinhallintaan ja auttaa välttämään yleisiä virheitä.

Seuraavaksi siirrymme lukuun 5 ja tutkimme tapoja ryhmitellä tietoa `struct`-rakenteilla.

