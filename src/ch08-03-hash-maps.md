## Avaimien ja niihin liittyvien arvojen tallentaminen hajautustauluilla

Viimeinen yleinen kokoelmatyyppi, jota tarkastelemme, on **hajautustaulu** (*hash map*). `HashMap<K, V>`-tyyppi tallentaa avain-arvo -pareja käyttäen **hajautusfunktiota**, joka määrittää, miten avaimet ja arvot sijoitetaan muistiin. Monet ohjelmointikielet tukevat vastaavaa tietorakennetta, mutta sitä kutsutaan eri nimillä, kuten **hash**, **map**, **dictionary**, **associative array** tai **hash table**.

Hajautustaulut ovat hyödyllisiä silloin, kun tietoa halutaan hakea **avaimen** eikä **indeksin** perusteella. Esimerkiksi pelissä voidaan seurata joukkueiden pisteitä käyttämällä hajautustaulua, jossa avaimina ovat joukkueiden nimet ja arvoina pisteet.

Käymme tässä osiossa läpi hajautustaulujen perusominaisuudet, mutta Rustin standardikirjastosta löytyy monia hyödyllisiä lisäominaisuuksia.

### Uuden hajautustaulun luominen

Voimme luoda tyhjän hajautustaulun käyttämällä `new`-metodia ja lisätä siihen avain-arvo -pareja `insert`-metodilla. Seuraavassa esimerkissä tallennetaan kahden joukkueen pisteet:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-20/src/main.rs:here}}
```

Huomaa, että `HashMap`-tietorakenne tulee erikseen tuoda standardikirjaston `collections`-osasta. Hajautustaulut eivät ole yhtä yleisesti käytettyjä kuin vektorit ja merkkijonot, joten niitä ei tuoda automaattisesti esiin Rustin **prelude**-määrittelyllä.

Hajautustaulut muistuttavat vektoreita siinä mielessä, että ne tallennetaan **kekoon** (*heap*). Tässä esimerkissä `HashMap` sisältää **String**-tyyppisiä avaimia ja **i32**-tyyppisiä arvoja. Kuten vektorit, myös hajautustaulut ovat **homogeenisia**, eli kaikki avaimet ja kaikki arvot ovat aina samaa tietotyyppiä.

### Arvojen hakeminen hajautustaulusta

Voimme hakea arvoja hajautustaulusta `get`-metodilla antamalla sille avaimen:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-21/src/main.rs:here}}
```

Tässä `score` saa **Blue**-joukkueen arvon, joka on `10`. `get` palauttaa `Option<&V>`-tyypin: jos avainta ei löydy, palautetaan `None`. Tässä tapauksessa `copied`-metodilla muunnetaan `Option<&i32>` muotoon `Option<i32>`, ja `unwrap_or`-metodilla määritetään, että puuttuvien avainten kohdalla tuloksena on `0`.

Voimme myös iteroida avain-arvo -parien yli `for`-silmukalla:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-03-iterate-over-hashmap/src/main.rs:here}}
```

Tulostusjärjestys voi olla satunnainen:

```text
Yellow: 50
Blue: 10
```

### Hajautustaulut ja omistus

Jos avaimet ja arvot ovat tyyppejä, jotka toteuttavat `Copy`-piirteen (esim. `i32`), ne **kopioidaan** hajautustauluun. Jos taas tallennamme omistettuja arvoja, kuten `String`, ne **siirtyvät** hajautustaulun omistukseen:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-22/src/main.rs:here}}
```

Tämän jälkeen `field_name` ja `field_value` eivät ole enää käytettävissä, koska niiden omistajuus on siirtynyt hajautustaululle.

Jos haluamme välttää omistajuuden siirtymisen, voimme lisätä tauluun **viittauksia**. Tällöin viitatun tiedon täytyy pysyä olemassa vähintään niin kauan kuin hajautustaulu on käytössä.

### Hajautustaulun päivittäminen

Vaikka hajautustauluun voi lisätä uusia avain-arvo -pareja, **sama avain voi sisältää vain yhden arvon kerrallaan**. Arvon päivittäessä on valittava, miten olemassa oleva arvo käsitellään:

- **Vanha arvo korvataan uudella**
- **Uusi arvo lisätään vain, jos avainta ei ole vielä olemassa**
- **Vanha ja uusi arvo yhdistetään**

#### Arvon korvaaminen

Jos lisäämme avaimelle uuden arvon, vanha arvo ylikirjoitetaan:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-23/src/main.rs:here}}
```

Tuloste:

```text
{"Blue": 25}
```

Ensimmäinen `insert("Blue", 10)` korvautuu toisella `insert("Blue", 25)`, joten vain viimeisin arvo jää talteen.

#### Avain-arvo -parin lisääminen vain, jos avainta ei vielä ole

Rust tarjoaa `entry`-API:n, joka mahdollistaa uuden arvon lisäämisen vain, jos avainta ei vielä ole:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-24/src/main.rs:here}}
```

Tuloste:

```text
{"Yellow": 50, "Blue": 10}
```

Ensimmäinen `entry("Yellow")` lisää uuden arvon `50`, koska **Yellow**-avain puuttuu. **Blue**-avain on jo olemassa, joten sen arvoa ei muuteta.

#### Arvon päivittäminen vanhan arvon perusteella

Seuraavassa esimerkissä lasketaan, kuinka monta kertaa kukin sana esiintyy tekstissä:

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-25/src/main.rs:here}}
```

Tuloste:

```text
{"world": 2, "hello": 1, "wonderful": 1}
```

Ohjelma jakaa tekstin sanoiksi ja käyttää `entry().or_insert(0)` -kutsua varmistaakseen, että jokaiselle sanalle on alkulukema `0`. Sitten se kasvattaa sanan esiintymiskertaa.

### Hajautusfunktioiden käyttäminen

Rustin `HashMap` käyttää oletuksena **SipHash**-hajautusfunktiota, joka suojaa **palvelunestohyökkäyksiä** (*DoS attacks*) vastaan. Tämä ei ole nopein mahdollinen hajautusalgoritmi, mutta se tarjoaa hyvän tasapainon **turvallisuuden ja suorituskyvyn** välillä.

Jos ohjelmasi vaatii nopeampaa hajautusta, voit määrittää **eri hajautusfunktion**. Tätä varten on olemassa **`BuildHasher`-rajapinta**, jota voidaan käyttää erilaisten hajautusalgoritmien kanssa. Näitä löytyy esimerkiksi Rustin [crates.io](https://crates.io/) -kirjastosta.

---

## Yhteenveto

Tässä luvussa opimme käyttämään **vektoreita, merkkijonoja ja hajautustauluja** tiedon tallentamiseen, hakemiseen ja päivittämiseen.

### Harjoitustehtävät

1. Anna joukko kokonaislukuja vektorina ja laske **mediaani** (järjestettynä keskimmäinen arvo) ja **moodi** (yleisin arvo). Hajautustaulu voi olla hyödyllinen tähän.
2. Muunna merkkijonot **pig latin** -kielelle: siirrä sanan ensimmäinen konsonantti loppuun ja lisää _ay_, esim. **"first" → "irst-fay"**. Jos sana alkaa vokaalilla, lisää _hay_, esim. **"apple" → "apple-hay"**.
3. Luo **tekstikäyttöliittymä**, jonka avulla käyttäjä voi lisätä työntekijöitä osastoille ja hakea heitä hajautustaulusta. Tulosta työntekijät aakkosjärjestyksessä.

Seuraavaksi käsittelemme **virheidenhallintaa**, koska alamme siirtyä monimutkaisempiin ohjelmiin, joissa operaatioita voi epäonnistua!
