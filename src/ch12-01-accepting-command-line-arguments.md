## Komentoriviargumenttien vastaanottaminen

Luodaan uusi projekti komennolla `cargo new`, kuten aina. Nimeämme projektimme
`minigrep` erottamaan sen järjestelmässäsi mahdollisesti jo olevasta `grep`-työkalusta.

```console
$ cargo new minigrep
     Created binary (application) `minigrep` project
$ cd minigrep
```

Ensimmäinen tehtävä on saada `minigrep` vastaanottamaan kaksi komentoriviargumenttia:
tiedostopolku ja etsittävä merkkijono. Haluamme pystyä ajamaan ohjelman komennolla:

```console
$ cargo run -- hakusana esimerkkitiedosto.txt
```

Tällä hetkellä `cargo new` -komennon luoma ohjelma ei osaa käsitellä argumentteja.
On olemassa valmiita kirjastoja [crates.io](https://crates.io/)-palvelussa, jotka
helpottavat komentoriviargumenttien käsittelyä, mutta koska olemme vasta oppimassa
tätä konseptia, toteutamme ominaisuuden itse.

### Argumenttien lukeminen

Jotta `minigrep` voi lukea sille annetut komentoriviargumentit, tarvitsemme
Rustin standardikirjaston `std::env::args`-funktion. Tämä funktio palauttaa
iteraattorin, joka sisältää komentoriviargumentit.

Iteraattorit tuottavat arvojoukon, ja voimme käyttää `collect`-metodia
kerätäksemme ne kokoelmaan, kuten vektoriin.

Seuraava koodi (Listing 12-1) mahdollistaa `minigrep`-ohjelman lukemaan kaikki
komentoriviargumentit ja tallentamaan ne vektoriin.

<Listing number="12-1" file-name="src/main.rs" caption="Komentoriviargumenttien kerääminen vektoriin ja tulostaminen">

```rust
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-01/src/main.rs}}
```

</Listing>

Ensimmäisenä tuomme `std::env`-moduulin käyttöön `use`-lauseella, jotta voimme käyttää sen `args`-funktiota.
Koska `std::env::args` sijaitsee kahdessa sisäkkäisessä moduulissa, on parempi tuoda koko `std::env` käyttöön
kuin pelkkä `args`. Tämä vähentää epäselvyyksiä ja mahdollistaa muidenkin `std::env`-moduulin funktioiden käytön.

> ### `args`-funktion ja virheellisen Unicoden käsittely
>
> Huomaa, että `std::env::args`-kutsu kaatuu, jos jokin argumentti sisältää virheellistä Unicodea.
> Jos ohjelmasi tarvitsee käsitellä argumentteja, jotka eivät ole kelvollista Unicodea, käytä `std::env::args_os`.
> Tämä funktio palauttaa iteraattorin, joka tuottaa `OsString`-arvoja `String`-arvojen sijaan.

`main`-funktion ensimmäisellä rivillä kutsumme `env::args`-funktiota ja käytämme `collect`-metodia muuttaaksemme
iteraattorin vektoriksi. Koska `collect` voi luoda monenlaisia kokoelmia, meidän on annettava `args`-muuttujalle
eksplisiittinen tyyppi (`Vec<String>`), jotta Rust tietää, minkä tyyppinen kokoelma halutaan.

Lopuksi tulostamme vektorin `dbg!`-makrolla. Kokeillaan ohjelman suorittamista ensin ilman argumentteja
ja sitten kahdella argumentilla:

```console
{{#include ../listings/ch12-an-io-project/listing-12-01/output.txt}}
```

```console
{{#include ../listings/ch12-an-io-project/output-only-01-with-args/output.txt}}
```

Huomaa, että ensimmäinen vektorin arvo on `"target/debug/minigrep"`, joka on ohjelmamme binäärin nimi.
Tämä vastaa C-kielessä olevaa `argv[0]`-arvoa ja antaa ohjelmille mahdollisuuden käyttää omaa nimeään
esimerkiksi käyttöohjeiden tulostamiseen. Tässä luvussa emme kuitenkaan tarvitse sitä, joten voimme jättää sen huomiotta.

### Argumenttien tallentaminen muuttujiin

Nyt ohjelma pystyy vastaanottamaan komentoriviargumentteja. Seuraavaksi tallennamme nämä argumentit muuttujiin,
jotta voimme käyttää niitä ohjelman muissa osissa. Tämä tapahtuu seuraavassa koodinpätkässä (Listing 12-2).

<Listing number="12-2" file-name="src/main.rs" caption="Muuttujien luominen hakusanan ja tiedostopolun tallentamiseen">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-02/src/main.rs}}
```

</Listing>

Koska ensimmäinen arvo (`args[0]`) on ohjelman nimi, varsinainen hakusana alkaa indeksistä `1` ja tiedostopolku
indeksistä `2`. Määritämme siis muuttujan `query` viittaamaan ensimmäiseen argumenttiin ja `file_path`-muuttujan
toiseen argumenttiin.

Tulostamme nämä arvot tilapäisesti varmistaaksemme, että ne ovat oikein. Kokeillaan ohjelman suorittamista seuraavilla
argumenteilla:

```console
{{#include ../listings/ch12-an-io-project/listing-12-02/output.txt}}
```

Hienoa, ohjelma toimii! Hakusanan ja tiedostopolun arvot tallennetaan oikeisiin muuttujiin. Myöhemmin lisäämme
virheenkäsittelyä tilanteisiin, joissa käyttäjä ei anna argumentteja. Toistaiseksi jatkamme tiedostojen lukemisen
lisäämisellä ohjelmaamme.

[ch13]: ch13-00-functional-features.html
[ch7-idiomatic-use]: ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths
