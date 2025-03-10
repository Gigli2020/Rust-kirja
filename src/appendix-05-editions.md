## Liite E - Rust-versiot (Editions)

Luvussa 1 näimme, että `cargo new` lisää hieman metadataa _Cargo.toml_-tiedostoon liittyen Rustin versioon (**edition**). Tämä liite selittää, mitä se tarkoittaa!

Rust-kieli ja sen kääntäjä julkaisevat uuden version **kuuden viikon välein**, mikä tarkoittaa jatkuvaa uusien ominaisuuksien virtaa. Toiset ohjelmointikielet julkaisevat suurempia muutoksia harvemmin; Rust päivittää kieltään pienemmissä erissä, mutta usein. Ajan mittaan nämä pienet muutokset kasaantuvat merkittäväksi kehitykseksi. Yksittäisten versioiden välillä voi olla vaikea huomata suuria eroja, mutta esimerkiksi Rust 1.10 ja Rust 1.31 -versioiden välillä ero voi olla huomattava.

Joka **kahden tai kolmen vuoden välein** Rust-tiimi julkaisee uuden **Rust-version (edition)**. Jokainen uusi versio kokoaa yhteen kaikki edelliset muutokset selkeäksi kokonaisuudeksi, jossa dokumentaatio ja työkalut ovat ajan tasalla. Uudet versiot julkaistaan osana normaalia kuuden viikon julkaisusykliä.

### Versioiden merkitys eri käyttäjäryhmille:

- **Aktiivisille Rustin käyttäjille** uudet versiot kokoavat pienet muutokset helposti ymmärrettäväksi paketiksi.  
- **Niille, jotka eivät vielä käytä Rustia**, uusi versio voi viestiä merkittävistä parannuksista ja kannustaa tutustumaan Rustiin uudelleen.  
- **Rustin kehittäjille** uusi versio toimii projektin yhteisenä tavoitteena ja virstanpylväänä.  

Tämän tekstin kirjoitushetkellä saatavilla on neljä Rust-versiota: **Rust 2015, Rust 2018, Rust 2021 ja Rust 2024**. Tämä kirja on kirjoitettu **Rust 2024 -version** mukaisesti.

### `edition`-avain _Cargo.toml_-tiedostossa

_Cargo.toml_-tiedostossa oleva `edition`-avain määrittää, mitä Rust-versiota kääntäjän tulisi käyttää koodillesi. Jos tätä avainta ei ole, Rust käyttää **2015-versiota** oletuksena taaksepäin yhteensopivuuden varmistamiseksi.

Jokainen projekti voi **valita haluamansa Rust-version**, eikä sen tarvitse pysyä oletusarvoisessa 2015-versiossa. Uudet versiot voivat sisältää **yhteensopimattomia muutoksia**, kuten uusien avainsanojen käyttöönoton, mikä voi vaikuttaa olemassa olevaan koodiin. **Jos et kuitenkaan valitse uudempaa versiota, koodisi kääntyy edelleen, vaikka päivität Rustin kääntäjäversiota.**

Kaikki Rustin kääntäjäversiot **tukevat aiempia versioita**, ja ne voivat yhdistää eri versioita käyttäviä **crate-kirjastoja**. Versiomuutokset vaikuttavat **vain siihen, miten kääntäjä alun perin jäsentää koodin**. Tämä tarkoittaa, että jos käytät Rust 2015 -versiota ja jokin riippuvuutesi käyttää Rust 2018 -versiota, projektisi **kääntyy ja toimii normaalisti**. Sama pätee toisinpäin: Rust 2018 -versiolla kääntyvä projekti voi käyttää Rust 2015 -versiolla käännettyjä riippuvuuksia.

### Uuden version käyttöönotto

Selvennyksen vuoksi: **suurin osa Rustin uusista ominaisuuksista toimii kaikissa versioissa**. Kehittäjät hyötyvät uusista ominaisuuksista riippumatta siitä, mitä versiota he käyttävät. **Joissain tapauksissa**, erityisesti uusien avainsanojen käyttöönoton yhteydessä, jotkut ominaisuudet voivat olla saatavilla **vain tietyissä Rust-versioissa**. Jos haluat hyödyntää näitä ominaisuuksia, sinun täytyy **vaihtaa projektisi käyttämään uudempaa Rust-versiota**.

Lisätietoa Rustin eri versioista löytyy **[_Edition Guide_](https://doc.rust-lang.org/stable/edition-guide/)** -oppaasta, joka kuvaa tarkasti eri versioiden erot ja selittää, miten voit päivittää koodisi uuteen versioon `cargo fix` -komennolla.

