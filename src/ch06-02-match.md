## `match`-ohjausrakenne

Rust tarjoaa erittäin tehokkaan ohjausrakenteen nimeltä `match`, jonka avulla voit verrata arvoa useisiin kuvioihin ja suorittaa niihin liittyvää koodia. Kuvioissa voi olla kirjaimellisia arvoja, muuttujien nimiä, jokerimerkkejä ja monia muita elementtejä. Luvussa 19 käsittelemme kaikki mahdolliset kuviotyypit tarkemmin.

Voit ajatella `match`-rakennetta kolikoiden lajittelukoneena: kolikot liukuvat radalla, jossa on erikokoisia aukkoja, ja jokainen kolikko putoaa ensimmäiseen sopivaan aukkoon. Samalla tavalla `match` käy läpi vaihtoehtoja ja suorittaa ensimmäisen sopivan lohkon.

### `match`-ilmaus käytännössä

Seuraava ohjelma tunnistaa Yhdysvaltain kolikon ja palauttaa sen arvon senteissä:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

Tässä `match`-rakenne vertailee `coin`-arvoa eri `Coin`-enumin varianteihin ja palauttaa kolikon arvon.

Voimme myös suorittaa lisätoimintoja yksittäisissä haaroissa:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

Tässä `Coin::Penny`-variantti tulostaa “Lucky penny!” ennen arvon palauttamista.

### Kuvioiden sidonta arvoihin

Voimme käyttää `match`-rakenteessa myös muuttujien sitomista varianttien sisäisiin arvoihin. Esimerkiksi vuosina 1999–2008 Yhdysvaltain 25 sentin kolikoihin painettiin osavaltiokohtaisia kuvioita. Voimme mallintaa tämän lisäämällä `UsState`-tyypin `Quarter`-varianttiin:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

Sitten voimme sitoa `state`-muuttujan kolikon variantin osavaltiotietoon:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

### `Option<T>`-käsittely `match`-ilmaisulla

`Option<T>`-enumia voidaan käsitellä samalla tavalla kuin muita enum-tyyppejä. Esimerkiksi seuraava funktio lisää yhden `Some(i32)`-arvoon, mutta palauttaa `None`, jos arvo puuttuu:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

Rust pakottaa käsittelemään kaikki mahdolliset tapaukset, mikä estää unohtamasta `None`-varianttia ja auttaa välttämään virheitä.

### `match`-lauseiden täydellisyysvaatimus

Rust vaatii, että `match` kattaa kaikki mahdolliset vaihtoehdot:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

Tämä ohjelma ei käänny, koska `None`-tapaukselle ei ole käsittelyä.

### Yleisvastineet ja `_`-paikkamerkki

Jos haluamme käsitellä tiettyjä arvoja erityisesti, mutta kaikki muut yleisesti, voimme käyttää `_`-paikkamerkkiä:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-15-binding-catchall/src/main.rs:here}}
```

Tässä `other`-muuttuja tallentaa kaikki muut arvot, joita ei ole erikseen käsitelty.

Jos emme tarvitse arvoa, voimme käyttää `_`-merkkiä, joka ei sido mitään muuttujaa:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-16-underscore-catchall/src/main.rs:here}}
```

Lopuksi voimme käyttää `()`-yksikköarvoa merkkinä siitä, ettei tarvitse suorittaa mitään koodia:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-17-underscore-unit/src/main.rs:here}}
```

Seuraavaksi tutustumme `if let`-rakenteeseen, joka tarjoaa yksinkertaisemman vaihtoehdon `match`-ilmaisun käyttöön tietyissä tilanteissa.
