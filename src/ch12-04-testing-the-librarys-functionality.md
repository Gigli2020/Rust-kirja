## Kirjaston toiminnallisuuden kehittäminen testivetoisesti

Nyt kun olemme siirtäneet ohjelmalogiikan tiedostoon _src/lib.rs_ ja jättäneet
argumenttien käsittelyn sekä virheiden hallinnan _src/main.rs_-tiedostoon, on
paljon helpompaa kirjoittaa testejä koodin keskeiselle toiminnallisuudelle.
Voimme kutsua funktioita suoraan erilaisilla argumenteilla ja tarkistaa niiden
paluuarvot ilman, että tarvitsemme suorittaa binääriä komentoriviltä.

Tässä osiossa lisäämme hakutoiminnallisuuden `minigrep`-ohjelmaan
testivetoisen kehityksen (TDD) prosessin avulla:

1. Kirjoitetaan testi, joka epäonnistuu, ja ajetaan se varmistaaksemme, että
   se epäonnistuu odotetusta syystä.
2. Kirjoitetaan tai muokataan koodia juuri sen verran, että testi menee läpi.
3. Refaktoroidaan lisätty tai muutettu koodi varmistaen, että testit yhä toimivat.
4. Palataan kohtaan 1 ja toistetaan!

Vaikka tämä on vain yksi monista tavoista kirjoittaa ohjelmistoa, TDD auttaa
rakentamaan testikattavuutta ja ohjaa koodin suunnittelua. Kun testi kirjoitetaan
ennen toiminnallisuuden toteutusta, se varmistaa, että testit pysyvät ajan tasalla
ja kattavat koko kehitysprosessin.

Toteutamme hakufunktion, joka etsii hakusanaa tiedoston sisällöstä ja palauttaa
vain ne rivit, jotka sisältävät kyseisen hakusanan. Lisätään tämä toiminnallisuus
`search`-nimiseen funktioon.

### Kirjoitetaan epäonnistuva testi

Poistetaan _src/lib.rs_ ja _src/main.rs_ -tiedostoista aiemmin lisätyt `println!`-
komennot, koska emme enää tarvitse niitä. Lisäämme sen jälkeen _src/lib.rs_-tiedostoon
testimoduulin, kuten teimme [luvussa 11][ch11-anatomy].

Alla oleva testi määrittelee, miten `search`-funktion pitäisi käyttäytyä: sen
tulisi ottaa vastaan hakusana ja etsittävä teksti, ja palauttaa vain ne rivit,
joissa hakusana esiintyy. **Listing 12-15** näyttää tämän testin:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

Testi etsii merkkijonoa `"duct"`. Haettavassa tekstissä on kolme riviä, joista
vain yksi sisältää `"duct"`-sanan. Testissä varmistamme, että `search` palauttaa
vain tämän rivin.

Tällä hetkellä emme voi vielä suorittaa testiä, koska `search`-funktiota ei
ole olemassa! TDD-periaatteen mukaisesti lisäämme juuri sen verran koodia,
että testi kääntyy ja suoritetaan. Alla **Listing 12-16** näyttää, kuinka lisäämme
`search`-funktion, joka toistaiseksi vain palauttaa tyhjän vektorin:

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

`search`-funktion määrityksessä meidän täytyy määrittää elinikä `'a`, jota käytetään
`contents`-parametrille ja palautusarvolle. Tämä varmistaa, että palautettava
vektori sisältää merkkijonoviipaleita, jotka viittaavat `contents`-parametrin
viipaleisiin.

Nyt voimme ajaa testin:

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

Testi epäonnistui, kuten odotettiin. Seuraavaksi saamme sen onnistumaan!

### Kirjoitetaan koodi, joka saa testin läpäisemään

Testi epäonnistuu tällä hetkellä, koska `search` palauttaa aina tyhjän vektorin.
Korjataksemme tämän lisäämme seuraavat vaiheet:

1. Käydään läpi jokainen rivi `contents`-muuttujasta.
2. Tarkistetaan, sisältääkö rivi haettavan hakusanan.
3. Jos kyllä, lisätään se palautettavaan listaan.
4. Muussa tapauksessa ei tehdä mitään.
5. Palautetaan hakutulokset sisältävä lista.

#### Rivien läpikäynti `lines`-metodilla

Rust tarjoaa `lines`-metodin, joka helpottaa tekstin käsittelyä rivitasolla.
Seuraava koodi (Listing 12-17) esittää rivien iteroimisen:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

`lines` palauttaa iteraattorin, jonka avulla voimme käsitellä jokaista riviä erikseen.

#### Hakusanan etsiminen `contains`-metodilla

Lisätään hakutoiminnallisuus hyödyntämällä `contains`-metodia:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

Nyt funktio tarkistaa, sisältääkö rivi hakusanan.

#### Osumien tallentaminen

Seuraavaksi lisäämme mekanismin osumien tallentamiseen:

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

Nyt `search` palauttaa vain ne rivit, jotka sisältävät hakusanan. Ajetaan testi:

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

Testi onnistui!

Seuraavaksi voisimme optimoida `search`-funktion käyttämään iterointia tehokkaammin,
mutta palaamme tähän [luvussa 13][ch13-iterators], jossa käsittelemme iterointia
yksityiskohtaisemmin.

### `search`-funktion käyttäminen `run`-funktiossa

Nyt kun `search` toimii ja on testattu, kutsumme sitä `run`-funktiosta. Seuraavassa
koodissa (Listing 12-20) kutsutaan `search`-funktiota ja tulostetaan osumat:

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

Nyt voimme kokeilla ohjelmaa!

Etsitään sanaa `"frog"`, jonka pitäisi palauttaa yksi rivi:

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

Etsitään sanaa `"body"`, jonka pitäisi palauttaa useita rivejä:

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

Etsitään sanaa, joka ei esiinny tiedostossa:

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

Kaikki toimii! Olemme rakentaneet oman miniversiomme klassisesta `grep`-työkalusta
ja oppineet paljon Rustin ohjelmoinnista.

Seuraavaksi opimme, miten ympäristömuuttujia ja **standard error** -tulostusta
voidaan hyödyntää komentoriviohjelmissa.

[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch13-iterators]: ch13-02-iterators.html
