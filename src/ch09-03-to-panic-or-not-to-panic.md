## Käyttääkö `panic!`-makroa vai ei?

Miten päättää, milloin `panic!`-makroa tulisi käyttää ja milloin tulisi palauttaa `Result`? Kun koodi käyttää `panic!`, virheestä ei voi palautua. Voit kutsua `panic!`-makroa missä tahansa virhetilanteessa, mutta tällöin päätät kutsuvan koodin puolesta, että tilanne on palauttamaton. Jos sen sijaan palautat `Result`-arvon, annat kutsuvalle koodille mahdollisuuden valita, kuinka virhe käsitellään. Kutsuva koodi voi yrittää palautua tilanteesta tai päättää, että virhe on kriittinen ja kutsua `panic!`...

Esimerkeissä, prototyyppikoodissa ja testeissä on usein sopivampaa käyttää `panic!`-makroa `Result`-arvon palauttamisen sijaan. Tarkastellaan, miksi näin on, ja käsitellään tilanteita, joissa kääntäjä ei voi tietää, että virhe on mahdoton, mutta ihminen voi. Lopuksi annamme yleisiä ohjeita siitä, milloin kirjastoissa kannattaa käyttää `panic!`-makroa.

### Esimerkit, prototyypit ja testit

Kun kirjoitat esimerkkejä selittämään jotain konseptia, virheenkäsittelykoodin lisääminen voi tehdä esimerkistä vaikeaselkoisemman. Esimerkeissä on ymmärrettävää käyttää `unwrap`-kutsuja, jotka voivat aiheuttaa paniikkitilan. Ne toimivat **paikkamerkkeinä** ja osoittavat, että virheenkäsittelyyn kannattaa kiinnittää huomiota tuotantokoodia kirjoitettaessa.

Samoin `unwrap` ja `expect` ovat käteviä prototyyppivaiheessa, kun et ole vielä päättänyt, miten virheet tulisi käsitellä. Ne jättävät selkeitä merkkejä koodiin ja auttavat tekemään siitä robustimpaa myöhemmin.

Testitilanteissa on usein tarkoituksenmukaista käyttää `panic!`-makroa, koska testin epäonnistuminen tarkoittaa, että virhe havaittiin. Siksi `unwrap` ja `expect` ovat hyödyllisiä testikoodissa.

### Tilanteet, joissa sinulla on enemmän tietoa kuin kääntäjällä

On myös tilanteita, joissa `unwrap` tai `expect` on perusteltu, koska **tiedät**, että `Result`-arvo on aina `Ok`, vaikka kääntäjä ei voi sitä päätellä. Tällöin voit silti joutua käsittelemään `Result`-arvon, koska kutsuttu funktio voi **yleisesti ottaen** epäonnistua, vaikka sinun tapauksessasi se ei voi tapahtua.

Esimerkiksi seuraavassa tapauksessa `expect`-metodin käyttö on perusteltua:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-08-unwrap-that-cant-fail/src/main.rs:here}}
```

Tässä `IpAddr`-instanssi luodaan **kovakoodatusta merkkijonosta**. Tiedämme, että `"127.0.0.1"` on **aina** kelvollinen IP-osoite. Kääntäjä ei kuitenkaan ymmärrä tätä, joten meidän on silti käsiteltävä `Result`. `expect`-metodin käyttö on tässä perusteltua, mutta on hyvä dokumentoida syy, miksi `expect` ei voi epäonnistua.

Jos merkkijono olisi saatu käyttäjän syötteenä, meidän olisi ehdottomasti käytettävä `Result`-tyypin virheenkäsittelyä, koska virhe voisi tapahtua.

### Virheenkäsittelyn suuntaviivat

Panic-tilan käyttö on suositeltavaa silloin, kun koodi voi päätyä **huonoon tilaan** (*bad state*). Tällainen tilanne syntyy, kun:

- Jokin **oletus, sopimus tai sääntö** on rikkoutunut (esim. virheellisiä tai ristiriitaisia arvoja on syötetty).
- Tilanne on **ennalta arvaamaton**, eikä kyse ole vain esimerkiksi käyttäjän virheellisestä syötteestä.
- Koodin suorittaminen **ei voi jatkua turvallisesti** tämän jälkeen.
- Tätä tilannetta **ei voi kuvata selkeästi** Rustin tyyppejä käyttäen.

Jos joku kutsuu funktiotasi väärillä arvoilla, on yleensä parempi **palauttaa virhe** `Result`-arvona, jotta kutsuva koodi voi päättää, mitä tehdä. Jos kuitenkin jatkaminen olisi **epäturvallista tai haitallista**, paras vaihtoehto voi olla `panic!`, jolloin ohjelmoija huomaa ongelman kehitysvaiheessa.

Samoin `panic!` voi olla hyvä valinta, jos kutsumasi ulkoinen koodi palauttaa **kelvottoman tilan**, jota et voi itse korjata.

### `panic!` vs. `Result`

Jos virhe on **ennakoitavissa**, on yleensä parempi **palauttaa `Result`** kuin käyttää `panic!`-makroa. Esimerkiksi:

- Jos parseri saa **virheellistä dataa**, `Result` on parempi.
- Jos verkkopyyntö **ylittää nopeusrajan**, `Result` on parempi.

Tällaisissa tapauksissa `Result` kertoo kutsuvalle koodille, että virhe on **odotettavissa** ja se voi päättää, miten se käsittelee sen.

Jos taas operaatio voi vaarantaa ohjelman käyttäjän, kannattaa käyttää `panic!`-makroa. Esimerkiksi Rustin standardikirjasto **pysäyttää ohjelman**, jos yrität käyttää taulukon rajojen ulkopuolista indeksiä. Tämä on tietoturvasyistä, sillä muistin ulkopuolelle meneminen voi aiheuttaa haavoittuvuuksia.

### Räätälöityjen tyyppien käyttäminen validoinnissa

Rustin tyyppijärjestelmää voi käyttää virheiden estämiseen jo käännösaikana. Esimerkiksi:

- `Option<T>` kertoo, että arvo voi puuttua.
- `u32` varmistaa, että arvo ei voi olla negatiivinen.

Tämä vähentää tarvetta virheiden tarkistamiseen ajonaikana.

Esimerkkinä katsotaan **luvunarvauspeliä** luvusta 2. Oletetaan, että ohjelma pyytää käyttäjää arvaamaan luvun **1 ja 100 välillä**. Voimme lisätä tarkistuksen, joka varmistaa, että annettu arvo on sallitulla alueella:

```rust,ignore
{{#rustdoc_include ../listings/ch09-error-handling/no-listing-09-guess-out-of-range/src/main.rs:here}}
```

Tämä toimii, mutta tarkistus on toistettava jokaisessa paikassa, jossa arvoa käytetään. Parempi tapa on luoda **oma tyyppi**, joka **takuulla** sisältää vain luvut 1 ja 100 välillä:

```rust
{{#rustdoc_include ../listings/ch09-error-handling/listing-09-13/src/lib.rs}}
```

Tässä `Guess`-rakenne varmistaa, että arvo on **aina** validi. `new`-metodi **estää** virheellisten arvojen luonnin. Näin muut funktiot voivat turvallisesti käyttää `Guess`-tyyppiä ilman ylimääräisiä tarkistuksia.

---

## Yhteenveto

Rustin virheenkäsittely auttaa tekemään koodista **luotettavampaa**:

- `panic!` **pysäyttää ohjelman**, kun jatkaminen ei ole turvallista.
- `Result` antaa mahdollisuuden **palautua virheestä**.
- Rustin **tyyppijärjestelmä** voi auttaa ehkäisemään virheitä jo käännösaikana.

Seuraavaksi tutustumme **geneerisiin tyyppeihin**, joita käytetään laajalti Rustin standardikirjastossa.
