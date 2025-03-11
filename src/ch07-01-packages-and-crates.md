## Paketit ja laatikot

Ensimmäisenä käsittelemme moduulijärjestelmän perusosia: paketteja ja laatikoita (_crates_).

_Laatikko_ (_crate_) on pienin yksikkö, jota Rustin kääntäjä käsittelee kerrallaan. Vaikka käyttäisit `rustc`-komentoa ilman Cargoa ja kääntäisit yhden lähdekooditiedoston (kuten luvussa 1 teimme), kääntäjä pitää sitä yksittäisenä laatikkona. Laatikoihin voi kuulua moduuleita, ja moduulit voivat olla määriteltyinä muissa tiedostoissa, jotka käännetään osana laatikkoa.

Laatikot voivat olla kahdenlaisia: binäärilaatikoita ja kirjastolaatikoita.  
- **Binäärilaatikot** kääntyvät suoritettavaksi ohjelmaksi, kuten komentorivityökalu tai palvelin. Jokaisessa binäärilaatikossa täytyy olla `main`-funktio, joka määrittelee ohjelman suorituksen aloituskohdan.  
- **Kirjastolaatikot** eivät sisällä `main`-funktiota eivätkä käänny suoritettavaksi tiedostoksi. Sen sijaan ne tarjoavat toiminnallisuutta, jota muut projektit voivat käyttää. Esimerkiksi `rand`-kirjastolaatikko luvussa 2 tarjosi satunnaislukujen generoimiseen liittyviä toimintoja.

_Laatikon juuri_ on lähdekooditiedosto, josta Rustin kääntäjä aloittaa ja josta muodostuu laatikon päämoduuli.

**Paketti** on yksi tai useampi laatikko, joka tarjoaa tietyn toiminnallisuuden. Paketti sisältää _Cargo.toml_-tiedoston, joka määrittelee, miten laatikot kootaan ja rakennetaan. Cargo itse on paketti, joka sisältää sekä binäärilaatikon (komentorivityökalun) että kirjastolaatikon, jota muut ohjelmat voivat käyttää.

Paketti voi sisältää useita binäärilaatikoita, mutta enintään yhden kirjastolaatikon. Jokaisen paketin täytyy sisältää vähintään yksi laatikko, joko binääri- tai kirjastolaatikko.

### Paketin luominen

Kun suoritat komennon `cargo new my-project`, tapahtuu seuraavaa:

```console
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src
$ ls my-project/src
main.rs
```

Tässä `cargo new` luo paketin, jossa on _Cargo.toml_-tiedosto ja _src/main.rs_. Koska paketti ei sisällä _src/lib.rs_-tiedostoa, se on binäärilaatikko. Jos kansiossa olisi myös _src/lib.rs_, paketti sisältäisi sekä binäärilaatikon että kirjastolaatikon.

Voit myös lisätä useita binäärilaatikoita luomalla tiedostoja _src/bin/_-hakemistoon. Jokainen tiedosto _src/bin/-hakemistossa muodostaa oman binäärilaatikkonsa.

Seuraavaksi tarkastelemme moduuleita ja sitä, kuinka ne vaikuttavat näkyvyysalueisiin ja tietoturvaan.
