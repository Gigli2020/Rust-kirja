## Moduulien määrittäminen laajuuden ja yksityisyyden hallitsemiseksi

Tässä osiossa käsittelemme moduuleja ja muita moduulijärjestelmän osia, kuten _polkuja_, joiden avulla voidaan nimetä kohteita; `use`-avainsanaa, jolla tuodaan polkuja laajuuteen; sekä `pub`-avainsanaa, jolla voidaan tehdä kohteita julkisiksi. Käsittelemme myös `as`-avainsanaa, ulkoisia paketteja ja yleismerkkioperaattoria (`glob`).

### Moduulien pikaopas

Ennen kuin syvennymme moduulien ja polkujen yksityiskohtiin, tässä on pikaopas siitä, miten moduulit, polut, `use`-avainsana ja `pub`-avainsana toimivat kääntäjässä sekä miten useimmat kehittäjät organisoivat koodinsa. Käymme läpi esimerkkejä jokaisesta säännöstä tämän luvun aikana, mutta tämä on hyvä muistiinpano moduulien toiminnasta.

- **Aloita ohjelmakokonaisuuden juuresta**: Kun kääntäjä käsittelee ohjelmakokonaisuutta, se etsii käännettävää koodia juuritiedostosta (yleensä _src/lib.rs_ kirjastolle tai _src/main.rs_ binäärille).
- **Moduulien määrittäminen**: Ohjelmakokonaisuuden juuritiedostossa voit määrittää uusia moduuleja. Jos esimerkiksi määrität `mod garden;`, kääntäjä etsii moduulin koodia seuraavista paikoista:
  - Koodin sisällä, jolloin lohkosulkeet (`{}`) korvaavat puolipisteen `mod garden` jälkeen.
  - Tiedostosta _src/garden.rs_
  - Tiedostosta _src/garden/mod.rs_
- **Alimoduulien määrittäminen**: Kaikissa muissa tiedostoissa kuin juuritiedostossa voidaan määrittää alimoduuleja. Esimerkiksi _src/garden.rs_-tiedostossa voidaan määrittää `mod vegetables;`. Kääntäjä etsii alimoduulin koodia seuraavista paikoista:
  - Koodin sisällä, jolloin lohkosulkeet korvaavat puolipisteen `mod vegetables` jälkeen.
  - Tiedostosta _src/garden/vegetables.rs_
  - Tiedostosta _src/garden/vegetables/mod.rs_
- **Polut moduuleissa olevaan koodiin**: Kun moduuli on osa ohjelmakokonaisuutta, siihen voidaan viitata mistä tahansa samassa ohjelmakokonaisuudessa yksityisyysasetusten mukaisesti. Esimerkiksi `Asparagus`-tyyppi puutarhan vihannesmoduulissa löytyy osoitteesta `crate::garden::vegetables::Asparagus`.
- **Yksityinen vs. julkinen**: Moduulin sisällä oleva koodi on oletuksena yksityistä suhteessa sen ylätason moduuliin. Jotta moduuli olisi julkinen, se on määritettävä `pub mod` -avainsanalla. Jotta moduulin sisällä olevat kohteet olisivat julkisia, niiden edessä on käytettävä `pub`.
- **`use`-avainsana**: `use`-avainsanalla voidaan luoda lyhyempiä polkuja laajuuden sisällä toistamisen vähentämiseksi. Jos esimerkiksi jokin laajuus voi viitata `crate::garden::vegetables::Asparagus`, voidaan käyttää `use crate::garden::vegetables::Asparagus;` ja sen jälkeen viitata tyyppiin pelkällä `Asparagus`-nimellä.

Tässä luodaan binäärikokonaisuus nimeltä `backyard`, joka havainnollistaa näitä sääntöjä. Hakemistorakenne näyttää tältä:

```text
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs
```

Ohjelmakokonaisuuden juuritiedostona toimii _src/main.rs_, ja se sisältää:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/main.rs}}
```

Rivi `pub mod garden;` kertoo kääntäjälle, että sen on sisällytettävä _src/garden.rs_-tiedostosta löytyvä koodi:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden.rs}}
```

Tässä `pub mod vegetables;` tarkoittaa, että myös _src/garden/vegetables.rs_ -tiedostossa oleva koodi sisältyy mukaan:

```rust,noplayground,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/quick-reference-example/src/garden/vegetables.rs}}
```
