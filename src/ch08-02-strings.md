## UTF-8-koodatun tekstin tallentaminen merkkijonoihin

Käsittelimme merkkijonoja luvussa 4, mutta nyt tarkastelemme niitä syvällisemmin. Rustia opettelevat kehittäjät saattavat kohdata vaikeuksia merkkijonojen kanssa kolmesta syystä: 

1. Rust paljastaa mahdollisia virheitä muita kieliä herkemmin.
2. Merkkijonot ovat monimutkaisempia tietorakenteita kuin usein ajatellaan.
3. Rust käyttää UTF-8-koodausta, joka tuo omat haasteensa.

Käsittelemme merkkijonoja osana kokoelmia, koska ne ovat käytännössä tavujen kokoelmia, joihin liittyy hyödyllisiä metodeja tekstin käsittelemiseksi. Tässä osiossa käymme läpi `String`-tyypin perustavanlaatuisia operaatioita, kuten luomista, päivittämistä ja lukemista. Lisäksi selvitämme, miksi `String` eroaa muista kokoelmista, erityisesti indeksöinnin osalta.

### Mikä on merkkijono?

Rustin kieliytimeen kuuluu vain yksi merkkijonotyyppi: **merkkijonoviipale** (*string slice*, `&str`). Se viittaa UTF-8-koodattuun merkkijonodataan, joka on tallennettu muualle, esimerkiksi ohjelman binaaritiedostoon.

Rustin standardikirjasto tarjoaa `String`-tyypin, joka on **kasvava, muuttuva ja omistettu** UTF-8-merkkijono. Kun Rust-kehittäjät puhuvat merkkijonoista, he saattavat tarkoittaa joko `String`- tai `&str`-tyyppiä.

### Uuden merkkijonon luominen

Monet `Vec<T>`-tyypin metodit toimivat myös `String`-tyypillä, koska `String` on pohjimmiltaan vektorillinen tavuja, joihin on lisätty sääntöjä ja rajoituksia. Esimerkiksi uuden `String`-olion luonti onnistuu seuraavasti:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

Voimme myös luoda `String`-olion valmiilla sisällöllä käyttämällä `to_string`-metodia:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

Vastaava tulos saadaan myös `String::from`-funktiolla:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

Molemmat tavat tekevät saman asian, ja valinta riippuu lähinnä tyylistä ja luettavuudesta.

Koska merkkijonot ovat UTF-8-koodattuja, voimme tallentaa niihin monikielistä tekstiä:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

### Merkkijonon päivittäminen

Kuten `Vec<T>`, myös `String` voi kasvaa ja sen sisältöä voidaan muuttaa. Merkkijonoihin voidaan lisätä tekstiä `push_str`- ja `push`-metodeilla sekä `+`-operaattorilla ja `format!`-makrolla.

#### Merkkijonon jatkaminen `push_str`- ja `push`-metodeilla

Voimme lisätä `String`-olioon toisen merkkijonoviipaleen `push_str`-metodilla:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

Jos haluamme säilyttää alkuperäisen merkkijonoviipaleen myöhempää käyttöä varten, voimme tehdä näin:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

Yksittäisiä merkkejä voidaan lisätä `push`-metodilla:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

#### Merkkijonojen yhdistäminen `+`-operaattorilla ja `format!`-makrolla

Merkkijonoja voidaan yhdistää `+`-operaattorilla:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

Tässä `s1` siirtyy `add`-funktion omistukseen ja `s2` lisätään siihen viitteenä. Tämä tekee yhdistämisestä tehokkaampaa, koska se ei kopioi `s2`:ta.

Jos yhdistettäviä merkkijonoja on useita, `format!`-makro on selkeämpi vaihtoehto:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

Tämä luo `String`-olion, mutta ei omista yhtäkään syötteistään, joten alkuperäiset merkkijonot säilyvät muuttumattomina.

### Merkkijonojen indeksöinti

Toisin kuin monissa ohjelmointikielissä, Rust ei salli `String`-indeksöintiä:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

Rust ei salli tätä, koska merkkijonot tallennetaan tavuina, eikä yksittäinen indeksi välttämättä vastaa kokonaista merkkiä.

#### Sisäinen edustus

Merkkijonot ovat `Vec<u8>`-pohjaisia. Esimerkiksi `"Hola"` koostuu neljästä tavusta, mutta `"Здравствуйте"` vie 24 tavua, koska jokainen sen merkki vie enemmän kuin yhden tavun.

```rust
let hello = "Здравствуйте";
let s = &hello[0..4]; // "Зд"
```

Koska UTF-8 voi sisältää monen tavun merkkejä, väärän indeksin käyttäminen voi aiheuttaa ajonaikaisen virheen.

### Merkkijonon läpikäynti

Rust tarjoaa kaksi tapaa iteroida merkkijonoja:

- `chars()`, joka palauttaa Unicode-merkit:
  
  ```rust
  for c in "Зд".chars() {
      println!("{c}");
  }
  ```

- `bytes()`, joka palauttaa raakabittiset tavut:

  ```rust
  for b in "Зд".bytes() {
      println!("{b}");
  }
  ```

Koska UTF-8-merkistössä yksittäinen merkki voi koostua useista tavuista, indeksoinnin sijaan on suositeltavaa käyttää näitä metodeja.

### Merkkijonot eivät ole yksinkertaisia

Merkkijonot ovat monimutkaisia tietorakenteita, ja eri ohjelmointikielet käsittelevät niitä eri tavoin. Rust korostaa oikeanlaista merkkijonokäsittelyä oletuksena, mikä tarkoittaa, että kehittäjien täytyy miettiä enemmän UTF-8-tietojen käsittelyä jo alussa. Tämä saattaa paljastaa monimutkaisuuksia, joita muissa kielissä ei ole, mutta estää virheitä myöhemmässä kehitysvaiheessa.

Onneksi Rustin standardikirjasto tarjoaa monia hyödyllisiä metodeja, kuten `contains` (tekstin etsimiseen) ja `replace` (tekstin korvaamiseen). 

Seuraavaksi siirrymme yksinkertaisempaan kokoelmatyyppiin: hajautustauluihin!
