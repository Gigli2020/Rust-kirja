## Polut moduulipuun kohteisiin viittaamiseen

Jotta Rust tietäisi, mistä moduulipuun kohde löytyy, käytämme polkua samalla tavalla kuin navigoidessa tiedostojärjestelmässä. Funktiota kutsuttaessa meidän on tiedettävä sen polku.

Polku voi olla kahdenlainen:

- **Absoluuttinen polku** on täydellinen polku, joka alkaa ohjelmakokonaisuuden (*crate*) juuresta. Ulkoisista ohjelmakokonaisuuksista tulevan koodin absoluuttinen polku alkaa ohjelmakokonaisuuden nimellä, ja nykyisestä ohjelmakokonaisuudesta tulevan koodin polku alkaa sanalla `crate`.
- **Suhteellinen polku** alkaa nykyisestä moduulista ja käyttää `self`, `super` tai moduulissa määriteltyä tunnistetta.

Sekä absoluuttisia että suhteellisia polkuja seuraa yksi tai useampi tunniste, jotka erotetaan kaksoispisteillä (`::`).

Palaamme luvun 7-1 esimerkkiin ja sanomme, että haluamme kutsua `add_to_waitlist`-funktiota. Tämä tarkoittaa käytännössä sen polun selvittämistä. Alla oleva koodi sisältää osan aiemmasta esimerkistä, josta on poistettu joitakin moduuleja ja funktioita.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

Ensimmäisellä kerralla `add_to_waitlist`-funktiota kutsutaan absoluuttisella polulla. Koska funktio on määritetty samassa ohjelmakokonaisuudessa kuin `eat_at_restaurant`, voimme käyttää `crate`-avainta absoluuttisen polun alussa. Sen jälkeen lisäämme kaikki tarvittavat moduulit, kunnes saavutamme `add_to_waitlist`-funktion. Tämä on verrattavissa tiedostojärjestelmän polkuun, jossa käyttäisimme `/front_of_house/hosting/add_to_waitlist` kutsuaksemme ohjelmaa. `crate` toimii vastaavasti kuin `/` tiedostojärjestelmän juurena.

Toisella kerralla `add_to_waitlist` kutsutaan suhteellisella polulla. Polku alkaa `front_of_house`-moduulista, joka on määritetty samalle tasolle `eat_at_restaurant`-funktion kanssa moduulipuussa. Tämä on verrattavissa tiedostojärjestelmän polkuun `front_of_house/hosting/add_to_waitlist`. Koska polku alkaa moduulin nimellä, se on suhteellinen polku.

Päätös käyttääkö suhteellista vai absoluuttista polkua riippuu projektista ja siitä, siirtyykö koodin määrittely erikseen vai yhdessä sen käyttävän koodin kanssa. Yleisesti suosimme absoluuttisia polkuja, koska niiden avulla koodin määrittelyt ja niiden käyttöpaikat voivat siirtyä toisistaan riippumatta.

### Polkujen julkistaminen `pub`-avainsanalla

Rustin moduulijärjestelmässä kaikki kohteet (funktiot, metodit, rakenteet, luettelot, moduulit ja vakiot) ovat oletuksena yksityisiä suhteessa niiden ylätason moduuliin. Jos haluat tehdä kohteesta julkisen, voit määrittää sen `pub`-avainsanalla.

Tarkastellaan virheilmoitusta, joka kertoo, että `hosting`-moduuli on yksityinen. Haluamme, että `eat_at_restaurant`-funktio voi käyttää `add_to_waitlist`-funktiota, joten merkitsemme `hosting`-moduulin julkiseksi:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

Tämä ei kuitenkaan vielä riitä. Vaikka `hosting`-moduuli on nyt julkinen, sen sisältö on yhä yksityinen. `pub`-avainsana moduulin edessä sallii ylätason moduulien viitata siihen, mutta ei käyttää sen sisäistä koodia. Tehdäksemme `add_to_waitlist`-funktion julkiseksi, lisäämme `pub`-avainsanan myös sen määrittelyyn:

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

Nyt koodi toimii, koska `crate`-moduuli voi käyttää `front_of_house`-moduulia, `front_of_house` voi käyttää `hosting`-moduulia, ja `hosting`-moduuli sisältää julkisen `add_to_waitlist`-funktion.

### Suhteellisten polkujen aloittaminen `super`-avainsanalla

Voimme luoda suhteellisia polkuja, jotka alkavat ylätason moduulista käyttämällä `super`-avainsanaa. Tämä vastaa tiedostojärjestelmässä `..`-syntaksia. `super` auttaa viittaamaan kohteeseen, jonka tiedämme olevan ylemmässä moduulissa, mikä helpottaa moduulipuun uudelleenjärjestelyä.

Seuraavassa esimerkissä kokki korjaa virheellisen tilauksen ja tuo sen itse asiakkaalle. `fix_incorrect_order`-funktio on `back_of_house`-moduulissa ja kutsuu `deliver_order`-funktiota ylätason moduulista `super`-polun avulla:

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

### Rakenteiden ja luetteloiden julkistaminen

Voimme käyttää `pub`-avainsanaa rakenteiden ja luetteloiden määrittämiseen julkisiksi, mutta niillä on hieman erilaiset säännöt. Kun asetamme `pub`-avainsanan rakenteen eteen, itse rakenne on julkinen, mutta sen kentät ovat yhä yksityisiä. Jokainen kenttä on tehtävä julkiseksi erikseen.

Alla `back_of_house::Breakfast`-rakenne määritellään julkiseksi. `toast`-kenttä on julkinen, mutta `seasonal_fruit` on yksityinen. Tämä jäljittelee ravintolan käytäntöä, jossa asiakas voi valita leivän tyypin, mutta kokki päättää kausiluonteisen hedelmän:

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

Koska `toast`-kenttä on julkinen, voimme lukea ja muokata sitä `eat_at_restaurant`-funktiossa. `seasonal_fruit`-kenttä ei kuitenkaan ole käytettävissä, koska se on yksityinen.

Toisaalta, jos määritämme luettelon (`enum`) julkiseksi, kaikki sen variantit ovat automaattisesti julkisia:

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

Rustissa luettelon varianttien oletusarvoinen näkyvyys on julkinen, koska muuten niistä ei olisi hyötyä. Rakenteiden kentät sen sijaan ovat oletuksena yksityisiä, koska niitä käytetään usein sisäiseen tietorakenteeseen.

