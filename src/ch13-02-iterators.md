## Iteraattorien käyttö: Sarjan käsittely tehokkaasti

Iteraattorimalli mahdollistaa tehtävien suorittamisen sarjalle kohteita yksi kerrallaan.
Iteraattori huolehtii logiikasta, joka määrittää, miten edetään seuraavaan kohteeseen ja
milloin sarja päättyy. Käyttämällä iteraattoreita voit välttää saman logiikan uudelleenkirjoittamisen.

Rustissa iteraattorit ovat **laiskoja**, eli ne eivät tee mitään ennen kuin kutsutaan
metodia, joka **kuluttaa** (*consume*) iteraattorin. Esimerkiksi alla oleva koodi
(*Listing 13-10*) luo iteraattorin `v1`-vektorin alkioista kutsumalla `iter`-metodia.
Tämä koodi ei vielä tee mitään hyödyllistä.

<Listing number="13-10" file-name="src/main.rs" caption="Iteraattorin luominen">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-10/src/main.rs:here}}
```

</Listing>

Kun iteraattori on luotu, voimme käyttää sitä monin tavoin. **Luvun 3 Listing 3-5** esimerkissä
iteroimme taulukon yli `for`-silmukalla. Rust loi automaattisesti iteraattorin ja kulutti sen,
mutta nyt tutkimme tätä prosessia tarkemmin.

Seuraavassa esimerkissä (*Listing 13-11*) erottelemme iteraattorin luonnin ja sen käytön
`for`-silmukassa. Kun silmukka kutsutaan `v1_iter`:llä, jokainen iteraattorin alkio käsitellään
silmukan sisällä.

<Listing number="13-11" file-name="src/main.rs" caption="Iteraattorin käyttö `for`-silmukassa">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-11/src/main.rs:here}}
```

</Listing>

Ilman iteraattoreita joutuisimme kirjoittamaan vastaavan logiikan käsin: alustaisimme muuttujan,
käyttäisimme sitä indeksinä vektoriin, lisäisimme sen arvoa ja lopettaisimme silmukan, kun
olemme käyneet kaikki alkiot läpi.

Iteraattorit tekevät tämän työn puolestamme ja tarjoavat joustavuutta, koska ne eivät rajoitu
vain tietorakenteisiin, joihin pääsee käsiksi indeksin avulla.

### `Iterator`-trait ja `next`-metodi

Kaikki iteraattorit toteuttavat standardikirjaston **`Iterator`**-traitin. Sen määritelmä näyttää tältä:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // muita oletusmetodeja
}
```

Tämä käyttää **liittyvää tyyppiä** (*associated type*), jota käsittelemme tarkemmin **luvussa 20**.
Tässä riittää tietää, että `Item` määrittää iteraattorin tuottamien alkioiden tyypin.

Ainoa pakollinen metodi `Iterator`-traitille on `next`, joka palauttaa yhden iteraattorin alkion
kerrallaan. Kun iteraattori loppuu, `next` palauttaa `None`.

Kutsumalla `next`-metodia suoraan voimme nähdä, miten iteraattori palauttaa arvoja:

<Listing number="13-12" file-name="src/lib.rs" caption="`next`-metodin kutsuminen">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-12/src/lib.rs:here}}
```

</Listing>

Huomaa, että `v1_iter` täytyy olla **muuttuva**, koska `next` muuttaa iteraattorin sisäistä tilaa.
Siksi `for`-silmukka ei tarvitse erikseen muuttuvaa iteraattoria – se ottaa omistajuuden ja käsittelee sen taustalla.

### Iteraattorin kuluttavat metodit

`Iterator`-trait sisältää monia metodeja, jotka kutsuvat `next`-metodia kuluttaen iteraattorin.
Näitä kutsutaan **kuluttaviksi sovittimiksi** (*consuming adapters*), koska ne käyttävät iteraattorin loppuun.

Yksi esimerkki on `sum`-metodi:

<Listing number="13-13" file-name="src/lib.rs" caption="`sum`-metodin käyttö">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-13/src/lib.rs:here}}
```

</Listing>

Kutsuttaessa `sum` omistajuus siirtyy ja iteraattoria ei voi enää käyttää sen jälkeen.

### Iteraattorin tuottavat metodit

**Sovittavat metodit** (*iterator adapters*) eivät kuluta iteraattoria vaan tuottavat uusia iteraattoreita.

Alla oleva `map`-esimerkki (*Listing 13-14*) ottaa sulkeisen ja soveltaa sitä jokaiseen iteraattorin alkioon:

<Listing number="13-14" file-name="src/main.rs" caption="`map`-metodin käyttö uuden iteraattorin luomiseen">

```rust,not_desired_behavior
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-14/src/main.rs:here}}
```

</Listing>

Tämä ei kuitenkaan tee mitään, koska iteraattorit ovat **laiskoja**. Saamme varoituksen:

```console
{{#include ../listings/ch13-functional-features/listing-13-14/output.txt}}
```

Kutsumme `collect`-metodia, joka kuluttaa iteraattorin ja muuntaa sen vektoriksi:

<Listing number="13-15" file-name="src/main.rs" caption="`map`-metodin tuloksen kerääminen vektoriksi">

```rust
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-15/src/main.rs:here}}
```

</Listing>

Koska `map` ottaa sulkeisen, voimme määritellä minkä tahansa muunnoksen.

### Sulkeiset ja iteraattorien sovittimet

Monet iteraattorien sovittimet käyttävät **sulkeisia**, jotka voivat tallentaa ympäristönsä muuttujia.

Esimerkiksi `filter`-metodi ottaa sulkeisen, joka palauttaa `true` tai `false`. Jos tulos on `true`,
alkio sisällytetään uuteen iteraattoriin.

Alla oleva esimerkki (*Listing 13-16*) käyttää `filter`-metodia suodattaakseen kengät kokotiedon perusteella:

<Listing number="13-16" file-name="src/lib.rs" caption="`filter`-metodin käyttö sulkeisen kanssa">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-16/src/lib.rs}}
```

</Listing>

Tässä `filter` käyttää sulkeista, joka vertaa kenkiä `shoe_size`-arvoon.

---

Tässä osiossa opimme:

- **Iteraattorien käytön ja edut**.
- **Kuluttavat metodit** (`sum`), jotka käyttävät iteraattorin loppuun.
- **Sovittavat metodit** (`map`, `filter`), jotka muokkaavat iteraattoria.
- **Sulkeisten yhdistämisen iteraattoreihin** joustavuuden lisäämiseksi.

Seuraavaksi tutustumme **oman iteraattorin toteuttamiseen**!
