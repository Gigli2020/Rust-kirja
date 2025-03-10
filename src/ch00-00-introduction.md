# Johdanto

> Huom: Tämä kirjan versio on sama kuin [The Rust Programming Language][nsprust], 
> joka on saatavilla painettuna ja e-kirjana [No Starch Pressiltä][nsp].

[nsprust]: https://nostarch.com/rust-programming-language-2nd-edition
[nsp]: https://nostarch.com/

Tervetuloa _The Rust Programming Language_ -kirjaan, joka toimii johdantona Rustiin.
Rust-ohjelmointikieli auttaa kirjoittamaan nopeampaa ja luotettavampaa ohjelmistoa.
Ohjelmointikielissä korkean tason käytettävyys ja matalan tason hallinta ovat usein ristiriidassa keskenään; 
Rust haastaa tämän asetelman. Yhdistämällä tehokkaan teknisen suorituskyvyn ja erinomaisen 
kehittäjäkokemuksen Rust antaa sinulle mahdollisuuden hallita matalan tason yksityiskohtia (kuten muistin käyttöä) 
ilman perinteisesti siihen liittyvää vaivaa.

## Kenelle Rust on tarkoitettu?

Rust sopii monenlaisille kehittäjille ja käyttäjille. Tarkastellaan joitakin keskeisiä ryhmiä.

### Kehitystiimit

Rust osoittautuu tuottavaksi työkaluksi suurille kehitystiimeille, joiden jäsenillä on vaihteleva 
kokemus järjestelmätason ohjelmoinnista. Matalan tason koodi on altis monille hienovaraisille 
bugeille, jotka useimmissa muissa kielissä havaitaan vain laajamittaisella testaamisella ja 
kokeneiden kehittäjien huolellisella koodikatselmoinnilla. Rustin kääntäjä toimii portinvartijana, 
joka kieltäytyy kääntämästä koodia, jossa on tällaisia vaikeasti havaittavia virheitä, mukaan lukien rinnakkaisuusvirheet. 
Työskentelemällä yhdessä kääntäjän kanssa tiimi voi keskittyä ohjelman logiikkaan virheiden metsästämisen sijaan.

Rust tuo myös modernit kehitystyökalut järjestelmätason ohjelmointiin:

- Cargo, sisäänrakennettu riippuvuuksien hallinta- ja rakennustyökalu, tekee riippuvuuksien hallinnasta helppoa ja yhdenmukaista koko Rust-ekosysteemissä.
- Rustfmt-muotoilutyökalu varmistaa yhtenäisen koodityylin kehittäjien kesken.
- Rust-analyzer tarjoaa IDE-integraation, joka mahdollistaa koodintäydennyksen ja reaaliaikaiset virheilmoitukset.

Näiden työkalujen ja muun Rust-ekosysteemin avulla kehittäjät voivat olla tuottavia kirjoittaessaan järjestelmätason koodia.

### Opiskelijat

Rust on hyvä valinta opiskelijoille ja kaikille, jotka haluavat oppia järjestelmätason ohjelmoinnista. 
Rustin avulla monet ovat oppineet aiheista, kuten käyttöjärjestelmien kehittämisestä. Yhteisö on hyvin 
tervetullut ja innokas vastaamaan opiskelijoiden kysymyksiin. Rust-tiimit haluavat tehdä järjestelmätason 
ohjelmoinnin helpommin lähestyttäväksi yhä useammille ihmisille.

### Yritykset

Sadat yritykset, sekä suuret että pienet, käyttävät Rustia tuotannossa monenlaisiin tehtäviin, kuten:

- komentorivityökalut
- verkkopalvelut
- DevOps-työkalut
- sulautetut järjestelmät
- äänen ja videon analysointi ja transkoodaus
- kryptovaluutat
- bioinformatiikka
- hakukoneet
- IoT-sovellukset
- koneoppiminen
- merkittävät osat Firefox-selaimesta

### Avoimen lähdekoodin kehittäjät

Rust on myös niille, jotka haluavat rakentaa Rust-ohjelmointikieltä, sen yhteisöä, kehittäjätyökaluja ja kirjastoja. Olet tervetullut osallistumaan Rustin kehitykseen!

### Nopeutta ja vakautta arvostavat kehittäjät

Rust on tarkoitettu niille, jotka arvostavat sekä nopeutta että vakautta ohjelmointikielessä. Nopeudella tarkoitetaan sekä Rust-koodin suoritustehoa että ohjelmoinnin tehokkuutta. Rustin kääntäjä varmistaa vakauden uusien ominaisuuksien ja refaktoroinnin myötä. Tämä eroaa perinteisistä kielistä, joissa kehittäjät usein pelkäävät muokata haurasta perintökoodia.

Rust pyrkii yhdistämään turvallisuuden _ja_ tuottavuuden, nopeuden _ja_ helppokäyttöisyyden. Kokeile Rustia ja katso, sopiiko sen lähestymistapa sinulle.

## Kenelle tämä kirja on tarkoitettu?

Kirja olettaa, että olet kirjoittanut koodia jollakin ohjelmointikielellä aiemmin, mutta ei edellytä tietoa mistään tietystä kielestä. Kirja ei keskity perusohjelmointikäsitteisiin, joten jos olet täysin uusi ohjelmoinnissa, voi olla parempi aloittaa kirja, joka toimii yleisenä ohjelmointijohdantona.

## Miten tätä kirjaa kannattaa lukea?

Kirjan luvut on järjestetty siten, että ne rakentuvat aiempien lukujen käsitteiden päälle. Kirja sisältää kahdenlaisia lukuja: käsitelukuja ja projektipohjaisia lukuja. Käsiteluvuissa opit Rustin eri ominaisuuksista, kun taas projektipohjaisissa luvuissa rakennamme yhdessä pieniä ohjelmia.

Yksityiskohtaisempi sisältö ja etenemissuositukset löytyvät varsinaisesta kirjan rungosta.

## Lähdekoodi

Tämän kirjan lähdetiedostot löytyvät [GitHubista][book].

[book]: https://github.com/rust-lang/book/tree/main/src
