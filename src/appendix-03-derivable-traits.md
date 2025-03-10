## Liite C: Johdettavat rajapinnat

Eri kohdissa kirjaa olemme käsitelleet `derive`-attribuuttia, jonka voi lisätä struct- tai enum-määrittelyyn. `derive`-attribuutti luo koodia, joka toteuttaa rajapinnan (trait) omalla oletusimplementaatiollaan sille tyypille, johon `derive` on lisätty.

Tässä liitteessä annamme viitteen kaikista standardikirjaston rajapinnoista, joita voi käyttää `derive`-attribuutin kanssa. Kukin osio käsittelee:

- Mitä operaattoreita ja metodeja tämän rajapinnan periyttäminen mahdollistaa
- Mitä `derive`-attribuutin tarjoama toteutus tekee
- Mitä kyseisen rajapinnan toteuttaminen tarkoittaa tyypille
- Milloin rajapinnan voi tai ei voi toteuttaa
- Esimerkkejä toiminnoista, jotka vaativat kyseisen rajapinnan

Jos haluat eri käyttäytymisen kuin `derive` tarjoaa, katso [standardikirjaston dokumentaatio](../std/index.html)<!-- ignore --> kunkin rajapinnan manuaalista toteuttamista varten.

Tässä luetellut rajapinnat ovat ainoat standardikirjastossa määritellyt rajapinnat, jotka voidaan toteuttaa `derive`-attribuutilla. Muilla standardikirjaston rajapinnoilla ei ole järkevää oletuskäyttäytymistä, joten niiden toteutus jää ohjelmoijan vastuulle.

Esimerkiksi `Display`-rajapintaa ei voi periyttää, koska se käsittelee loppukäyttäjälle tarkoitettua muotoilua. Ohjelmoijan täytyy harkita, miten tyyppi tulisi esittää loppukäyttäjälle. Mitkä osat tyypistä ovat olennaisia? Mikä tietojen esitystapa on käyttäjälle selkein? Koska Rust-kääntäjä ei voi tehdä näitä päätöksiä puolestasi, se ei tarjoa oletuskäyttäytymistä `Display`-rajapinnalle.

Tässä liitteessä luetellut `derive`-attribuutilla toteutettavat rajapinnat eivät kata kaikkea: myös kolmannen osapuolen kirjastot voivat määritellä omia rajapintojaan, jotka tukevat `derive`-attribuuttia. `derive`-toteutukset hyödyntävät proseduraalisia makroja, joista kerrotaan lisää luvussa [“Makrot”][macros]<!-- ignore -->.

### `Debug` ohjelmoijan tulostusta varten

`Debug`-rajapinta mahdollistaa virheenkorjaustulostuksen formaattijonoissa, kun lisäät `:?` `{}`-paikantimen sisään.

`Debug`-rajapinta mahdollistaa tyyppien tulostuksen virheenkorjausta varten, jolloin sekä ohjelmoija että muut kehittäjät voivat tarkastella tyyppien tilaa ohjelman suorituksen aikana.

Esimerkiksi `Debug`-rajapinta on vaatimuksena käytettäessä `assert_eq!`-makroa. Tämä makro tulostaa argumenttina annettujen arvojen tiedot, jos vertailu epäonnistuu, jolloin ohjelmoija voi nähdä, miksi arvot eivät olleet yhtäläisiä.

### `PartialEq` ja `Eq` tasa-arvovertailuihin

`PartialEq`-rajapinta mahdollistaa tyyppien vertailun `==`- ja `!=`-operaattoreilla.

Periytettäessä `PartialEq` toteuttaa `eq`-metodin. Jos `PartialEq` periytetään struct-tyyppiin, kaksi instanssia ovat yhtäläisiä vain, jos **kaikki** kentät ovat yhtäläisiä. Jos mikä tahansa kenttä poikkeaa, instanssit eivät ole yhtä suuria. Jos `PartialEq` periytetään enum-tyyppiin, jokainen variantti on itsensä kanssa yhtäläinen, mutta ei muiden varianttien kanssa.

`Eq`-rajapinta ei sisällä metodeja. Sen ainoa tarkoitus on ilmaista, että kaikki kyseisen tyypin arvot ovat aina yhtäläisiä itsensä kanssa. `Eq` voidaan toteuttaa vain, jos tyyppi toteuttaa myös `PartialEq`-rajapinnan, mutta kaikki `PartialEq`:n toteuttavat tyypit eivät välttämättä voi toteuttaa `Eq`:tä. Esimerkiksi liukuluvut (`NaN`-arvot) eivät aina täytä `Eq`-rajoitetta, koska `NaN != NaN`.

_(Loput rajapinnat ja esimerkit noudattavat samaa kaavaa ja säilyttävät alkuperäisen Markdown-muotoilun ja koodilohkot)_

[macros]: ch20-05-macros.html#macros
