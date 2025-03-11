## Viitteiden validointi elinikien avulla

Elinikä (*lifetime*) on toinen geneerinen käsite, jota olemme jo käyttäneet. Sen sijaan, että se määrittelisi, mitä toimintaa tyypillä on, se varmistaa, että viittaukset pysyvät voimassa niin kauan kuin niitä tarvitaan.

Rustissa **jokaisella viittauksella on elinikä**, joka määrittää, kuinka pitkään viittaus on kelvollinen. Useimmiten elinikä määritytään automaattisesti, kuten tyyppien kanssa tapahtuu. Rust vaatii elinikäannotointia vain silloin, kun viittausten elinikien suhde ei ole yksiselitteinen. 

Monissa ohjelmointikielissä elinikien hallintaa ei tarvita, joten tämä saattaa tuntua aluksi oudolta. Tässä osiossa käsittelemme tyypillisiä tilanteita, joissa Rustin elinikäsyntaksi tulee vastaan.

### Viittausten elinikien hallinta

Elinikien pääasiallinen tarkoitus on estää **roikkuvat viittaukset** (*dangling references*), jotka osoittavat olemattomaan dataan. Katsotaan esimerkkiä, jossa esiintyy tällainen virhe:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-16/src/main.rs}}
```

Tässä ohjelmassa muuttuja `r` määritellään ennen `x`:ää, mutta `r` viittaa `x`:ään, joka poistuu muistista ennen `r`-muuttujan käyttöä. Tämä johtaa Rustin kääntäjän virheeseen, joka estää ohjelman ajamisen.

Rust tunnistaa tämän ongelman **lainantarkistimen** (*borrow checker*) avulla.

### Lainantarkistimen toiminta

Rustin lainantarkistin vertailee viitteiden elinikää ja estää viittaukset, jotka voivat johtaa virheisiin. Seuraava koodi sisältää elinikäannotaatiot:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs}}
```

Tässä `r`:llä on elinikä `'a` ja `x`:llä elinikä `'b`. Koska `'b` on lyhyempi kuin `'a`, viittaus `r`:ssä olisi virheellinen ohjelman suorituksen aikana. 

Seuraavassa esimerkissä korjaamme ongelman niin, että `x` elää tarpeeksi pitkään:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs}}
```

Nyt `r` on kelvollinen niin kauan kuin `x` on olemassa.

### Geneeriset elinikäparametrit funktioissa

Seuraavaksi tarkastelemme funktiota, joka palauttaa pisimmän merkkijonon viipaleen:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs:here}}
```

Rust ei voi kääntää tätä, koska se ei tiedä, viittaako palautettava arvo `x`:ään vai `y`:hyn. Korjaamme tämän lisäämällä elinikäparametrit:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

Tämä kertoo Rustille, että **palautettava viipale on voimassa yhtä kauan kuin lyhytikäisin syötetty viipale**.

### Elinikien vaikutus funktioihin

Jos kutsumme `longest`-funktiota eri elinikäisillä muuttujilla, Rust varmistaa, että tulos pysyy kelvollisena:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

Tässä `string1` pysyy voimassa ohjelman loppuun, mutta `string2` on kelvollinen vain sisemmän lohkon ajan. `result` saa arvon, joka on voimassa vain niin kauan kuin lyhytikäisin parametri on voimassa.

Seuraava esimerkki **ei käänny**, koska `result` yrittää viitata `string2`:een sen eliniän jälkeen:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

Tämä johtaisi **roikkuvaan viittaukseen**, joten Rust estää sen.

### Elinikien käyttö rakenteissa

Jos rakenteessa on viittauksia, sen täytyy sisältää elinikäannotaatio:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs}}
```

Tämä varmistaa, että `ImportantExcerpt`-instanssi **ei voi elää pidempään** kuin sen sisältämä viittaus.

### Elinikien päätteleminen (*Lifetime elision*)

Rust pystyy päättelemään monia elinikäannotaatioita automaattisesti. Esimerkiksi tämä funktio kääntyy ilman annotaatioita:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs:here}}
```

Tämä toimii, koska Rust noudattaa **elinikäsääntöjä**:

1. Jokaiselle parametrille annetaan oma elinikä (`'a`, `'b`, jne.).
2. Jos parametreja on vain yksi, se periytyy myös palautusarvoon.
3. Jos funktio on metodi ja yksi parametri on `&self`, `self`-viittauksen elinikä periytyy palautusarvoon.

Näiden sääntöjen ansiosta meidän ei tarvitse lisätä elinikäannotaatioita joka paikkaan.

### Elinikäannotaatiot metodeissa

Kun määritämme metodeja rakenteille, joissa on elinikäannotaatioita, käytämme samaa syntaksia:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

Rust osaa automaattisesti päätellä `&self`-viittauksen eliniän, joten meidän ei tarvitse lisätä sitä käsin.

### Staattinen elinikä (`'static`)

Erityinen elinikä `'static` tarkoittaa, että viittaus on voimassa koko ohjelman suorituksen ajan:

```rust
let s: &'static str = "Minulla on staattinen elinikä.";
```

Kaikki **merkkijonoliteralit** ovat `'static`, koska ne tallennetaan ohjelman binaariin ja ovat aina saatavilla.

### Geneeriset tyypit, traitit ja elinikäparametrit yhdessä

Lopuksi katsotaan funktiota, joka yhdistää **geneeriset tyypit, trait-rajoitukset ja elinikäannotaatiot**:

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

Tämä `longest`-funktio hyväksyy **geneerisen tyypin `T`**, joka toteuttaa `Display`-traitin, ja **eliniän `'a`**, joka varmistaa viitteiden voimassaolon.

---

## Yhteenveto

Tässä luvussa opimme:

- **Geneeristen elinikien** käytön viitteiden validointiin.
- **Lainantarkistimen** ja elinikien hallinnan merkityksen.
- **Elinikäannotaatiot** funktioissa, rakenteissa ja metodeissa.
- **Staattisen eliniän `'static`**, joka säilyy koko ohjelman ajan.

Rustin elinikätarkistus auttaa kirjoittamaan **turvallista ja tehokasta** koodia ilman muistivirheitä. Seuraavaksi tutustumme **Rustin testausjärjestelmään**, joka auttaa varmistamaan, että koodi toimii odotetulla tavalla!
