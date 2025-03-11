## Moduulien erottaminen eri tiedostoihin

Tähän asti kaikki esimerkit tässä luvussa ovat määrittäneet useita moduuleja yhdessä tiedostossa. Kun moduulit kasvavat suuriksi, saatat haluta siirtää niiden määritelmät erillisiin tiedostoihin, jotta koodia olisi helpompi selata.

Esimerkiksi voimme aloittaa luvun 7-17 koodista, jossa oli useita ravintolaan liittyviä moduuleja. Siirrämme nämä moduulit erillisiin tiedostoihin sen sijaan, että määrittelisimme ne ohjelmakokonaisuuden juuritiedostossa. Tässä tapauksessa juuritiedosto on _src/lib.rs_, mutta sama menetelmä toimii myös binäärikokonaisuuksissa, joiden juuritiedosto on _src/main.rs_.

Ensimmäiseksi siirrämme `front_of_house`-moduulin omaan tiedostoonsa. Poista sen koodilohkon sisältö ja jätä vain `mod front_of_house;` -määrittely, jolloin _src/lib.rs_ näyttää seuraavalta:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/lib.rs}}
```

Huomaa, että tämä ei käänny ennen kuin luomme tiedoston _src/front_of_house.rs_:

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-21-and-22/src/front_of_house.rs}}
```

Rust tietää etsiä tätä tiedostoa, koska se löysi `mod front_of_house;`-määrittelyn ohjelmakokonaisuuden juurimoduulissa.

Huomioi, että `mod`-määrittely tarvitaan vain **yhden kerran** moduulipuussa. Kun kääntäjä tietää, että tiedosto kuuluu projektiin ja missä moduulipuussa se sijaitsee, muiden tiedostojen tulisi viitata siihen samalla tavoin kuin muihin moduuleihin – kuten käsiteltiin luvussa [Polut moduulipuussa viittaamiseen](ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html). `mod` ei siis toimi kuten `#include` joissakin muissa ohjelmointikielissä.

Seuraavaksi siirrämme `hosting`-moduulin omaan tiedostoonsa. Koska `hosting` on `front_of_house`-moduulin lapsimoduuli eikä juurimoduuli, prosessi on hieman erilainen. Luomme `hosting`-tiedoston _src/front_of_house_-hakemistoon.

Aloitamme muuttamalla _src/front_of_house.rs_-tiedostoa niin, että se sisältää vain `hosting`-moduulin määrittelyn:

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house.rs}}
```

Seuraavaksi luomme uuden _src/front_of_house/hosting.rs_-tiedoston ja siirrämme `hosting`-moduulin koodin sinne:

```rust,ignore
{{#rustdoc_include ../listings/ch07-managing-growing-projects/no-listing-02-extracting-hosting/src/front_of_house/hosting.rs}}
```

Jos `hosting.rs` sijaitsisi suoraan _src_-hakemistossa, kääntäjä olettaisi sen olevan ohjelmakokonaisuuden juurimoduulin `hosting`-moduuli eikä `front_of_house`-moduulin lapsimoduuli. Tämä tarkoittaa, että kääntäjän säännöt tiedostojen sijoittelusta vastaavat moduulipuuta.

> ### Vaihtoehtoiset tiedostopolut
>
> Rust tukee myös vanhempaa tiedostorakennetta. Esimerkiksi `front_of_house`-moduuli voi sijaita seuraavissa paikoissa:
>
> - _src/front_of_house.rs_ (nykyinen suositeltu tyyli)
> - _src/front_of_house/mod.rs_ (vanhempi tapa, jota yhä tuetaan)
>
> Vastaavasti `hosting`-moduuli voi sijaita:
>
> - _src/front_of_house/hosting.rs_ (nykyinen suositeltu tyyli)
> - _src/front_of_house/hosting/mod.rs_ (vanhempi tapa, jota yhä tuetaan)
>
> Jos käytät molempia tapoja samalle moduulille, saat käännösvirheen. Sekä vanhan että uuden tyylin käyttäminen eri moduuleissa samassa projektissa on sallittua, mutta voi tehdä projektin rakenteen vaikeasti ymmärrettäväksi.
>
> Vanhan tyylin suurin haitta on, että projektissa voi olla useita _mod.rs_-tiedostoja, mikä voi tehdä niiden käsittelystä vaikeaa, kun useita tiedostoja on auki samanaikaisesti.

Nyt jokainen moduuli on siirretty omaan tiedostoonsa, mutta moduulipuu pysyy muuttumattomana. `eat_at_restaurant`-funktion kutsut toimivat edelleen ilman muutoksia, vaikka määrittelyt ovat eri tiedostoissa. Tämä menetelmä mahdollistaa moduulien siirtämisen uusiin tiedostoihin niiden kasvaessa ilman, että koodia tarvitsee muuttaa merkittävästi.

Huomaa, että _src/lib.rs_:ssä oleva `pub use crate::front_of_house::hosting`-määrittely ei ole muuttunut, eikä `use` vaikuta siihen, mitkä tiedostot käännetään osana ohjelmakokonaisuutta. `mod`-avainsana määrittää moduulit, ja Rust etsii moduulin nimeä vastaavasta tiedostosta sen koodin.

## Yhteenveto

Rust mahdollistaa paketin jakamisen useisiin ohjelmakokonaisuuksiin (*crates*) ja ohjelmakokonaisuuden jakamisen moduuleihin, jotta eri moduuleihin määriteltyihin kohteisiin voidaan viitata toisista moduuleista. Tämä voidaan tehdä käyttämällä absoluuttisia tai suhteellisia polkuja. `use`-avainsanalla polku voidaan tuoda laajuuteen, jolloin sitä voidaan käyttää lyhyemmällä muodolla. Moduulin koodi on oletuksena yksityistä, mutta sen määrittelyt voidaan tehdä julkisiksi lisäämällä `pub`-avainsana.

Seuraavassa luvussa tutustumme Rustin standardikirjaston kokoelmatietorakenteisiin, joita voit käyttää järjestelmällisessä koodissasi.

