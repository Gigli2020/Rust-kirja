## Traitteja: Yhteisen käyttäytymisen määrittely

**Trait** määrittää toiminnallisuuden, jonka tietty tyyppi voi jakaa muiden tyyppien kanssa. Traitteja käytetään yhteisen käyttäytymisen määrittelemiseen abstraktilla tavalla. **Trait-rajauksilla** (*trait bounds*) voidaan rajoittaa, että geneerinen tyyppi voi olla vain sellainen tyyppi, jolla on tietty käyttäytyminen.

> **Huom:** Traitteja voi ajatella vastaavaksi kuin **rajapinnat** (*interfaces*) muissa kielissä, mutta niissä on eroja.

### Traitin määrittely

Tietotyypin käyttäytyminen koostuu metodeista, joita sille voidaan kutsua. Jos eri tyypeillä voidaan kutsua samoja metodeja, ne jakavat saman käyttäytymisen. **Trait-määrittelyt** mahdollistavat tällaisten metodien ryhmittelyn yhdeksi käyttäytymiskokonaisuudeksi.

Kuvitellaan, että meillä on useita rakenteita, jotka sisältävät erilaisia tekstisisältöjä: `NewsArticle`, joka pitää sisällään uutisartikkelin tiedot, ja `SocialPost`, joka voi sisältää enintään 280 merkkiä sekä metatiedon, kuten onko kyseessä uusi viesti, uudelleenpostaus vai vastaus.

Haluamme tehdä media-aggregointikirjaston `aggregator`, joka voi näyttää tietojen yhteenvedot riippumatta siitä, onko kyseessä `NewsArticle` tai `SocialPost`. Tämä onnistuu pyytämällä kutakin tyyppiä toteuttamaan `summarize`-metodin. Alla oleva esimerkki (Listing 10-12) määrittelee `Summary`-traitin tämän käyttäytymisen kuvaamiseksi:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-12/src/lib.rs}}
```

Tässä määrittelemme traitin käyttäen `trait`-avainsanaa ja annamme sille nimen `Summary`. Koska trait on määritelty `pub`-avainsanalla, muut ohjelmakokonaisuudet voivat käyttää sitä.

Kaikki tyypit, jotka toteuttavat tämän traitin, **täytyy** toteuttaa `summarize`-metodi, jonka palautusarvo on `String`. Koska metodin runko puuttuu ja se päättyy puolipisteeseen, kukin toteutus määrittelee sen omalla tavallaan.

### Traitin toteuttaminen tyypille

Kun olemme määritelleet `Summary`-traitin, voimme toteuttaa sen `NewsArticle`- ja `SocialPost`-rakenteille. Alla oleva koodi (Listing 10-13) näyttää, miten `summarize`-metodi voidaan toteuttaa molemmille rakenteille:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-13/src/lib.rs:here}}
```

Traitin toteuttaminen toimii kuten normaalien metodien toteuttaminen, mutta `impl`-avainsanan jälkeen annetaan traitin nimi, jonka jälkeen käytetään `for`-avainsanaa ja määritetään tyyppi, jolle trait toteutetaan.

Kun tämä on tehty, voimme kutsua `summarize`-metodia `NewsArticle`- ja `SocialPost`-instansseille:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-01-calling-trait-method/src/main.rs}}
```

Tämä tulostaa:
```text
1 new post: horse_ebooks: of course, as you probably already know, people
```

Toiset ohjelmakokonaisuudet voivat myös käyttää `Summary`-traitia ja toteuttaa sen omille tyypeilleen.

### Oletustoteutukset

Voimme määrittää traiteille **oletustoteutuksia**, jolloin kaikki traitin toteuttavat tyypit **perivät** oletustoteutuksen, elleivät ne erikseen ylikirjoita sitä.

Alla oleva koodi (Listing 10-14) näyttää `Summary`-traitin, jolla on oletustoteutus `summarize`-metodille:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-14/src/lib.rs:here}}
```

Tämän jälkeen `NewsArticle` voi toteuttaa traitin ilman omaa `summarize`-metodia:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-02-calling-default-impl/src/main.rs}}
```

Tulostus:
```text
New article available! (Read more...)
```

Oletustoteutukset voivat myös kutsua muita metodin toteutuksia traitin sisällä:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:here}}
```

Tässä `summarize`-metodi kutsuu `summarize_author`-metodia, joka täytyy toteuttaa erikseen:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-03-default-impl-calls-other-methods/src/lib.rs:impl}}
```

### Traitin käyttö parametrina

Voimme määrittää funktioita, jotka voivat ottaa vastaan **mitä tahansa** tyyppiä, joka toteuttaa tietyn traitin. Käytämme `impl Trait` -syntaksia:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-04-traits-as-parameters/src/lib.rs:here}}
```

Tämä funktio hyväksyy minkä tahansa tyypin, joka toteuttaa `Summary`-traitin.

### Traitin rajaukset geneerisille tyypeille

`impl Trait`-syntaksi on lyhyt tapa määrittää traitin rajaus, mutta voimme kirjoittaa sen myös pidemmällä muodolla:

```rust,ignore
pub fn notify<T: Summary>(item: &T) {
```

Voimme myös asettaa **useita trait-rajauksia** yhdelle geneeriselle tyypille käyttämällä `+`-syntaksia:

```rust,ignore
pub fn notify<T: Summary + Display>(item: &T) {
```

Jos trait-rajauksia on paljon, ne voidaan kirjoittaa `where`-lausekkeen avulla:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-07-where-clause/src/lib.rs:here}}
```

### Paluuarvona traitin toteuttava tyyppi

Voimme käyttää `impl Trait` -syntaksia myös funktioiden paluuarvoina:

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-05-returning-impl-trait/src/lib.rs:here}}
```

Tämä mahdollistaa paluuarvon tyypin piilottamisen, mutta kaikki palautettavat arvot **täytyy olla samaa tyyppiä**.

### Ehdollinen traitin toteutus

Voimme asettaa **ehdollisia toteutuksia** tyypeille käyttäen trait-rajauksia:

```rust,noplayground
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-15/src/lib.rs}}
```

Lisäksi voimme käyttää **yleisiä toteutuksia** (*blanket implementations*), jotka Rustin standardikirjasto hyödyntää paljon:

```rust,ignore
impl<T: Display> ToString for T {
}
```

Tämän ansiosta `to_string`-metodia voidaan kutsua kaikille `Display`-traitin toteuttaville tyypeille.

---

Seuraavaksi opimme **elinikätarkastelusta**, joka auttaa hallitsemaan viitteiden käyttöikää Rustissa.
