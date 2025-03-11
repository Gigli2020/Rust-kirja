## Virheviestien kirjoittaminen standardivirtaan `stderr`

Tällä hetkellä kirjoitamme kaiken tulosteen terminaaliin käyttämällä `println!`-makroa.
Useimmissa terminaaleissa on kuitenkin kaksi erilaista tulostusvirtaa:

- **Standardituloste** (`stdout`), jota käytetään yleiseen tulostamiseen.
- **Standardivirta** (`stderr`), jota käytetään virheilmoituksiin.

Tämä erottelu mahdollistaa sen, että käyttäjät voivat ohjata ohjelman **onnistuneen** tulostuksen tiedostoon,
mutta pitää virheilmoitukset näkyvissä näytöllä.

`println!`-makro pystyy tulostamaan vain standarditulosteeseen, joten meidän on käytettävä jotain muuta
tulostaaksemme virheilmoitukset standardivirtaan.

### Tarkistetaan, mihin virheet kirjoitetaan

Ensin havainnoimme, miten `minigrep` tulostaa tietoa tällä hetkellä. Aiomme ohjata
standarditulosteen tiedostoon mutta **jättää standardivirran ohjaamatta**,
jotta näemme, tulostuuko virheilmoitus tiedostoon vai terminaaliin.

Komentoriviohjelmien tulisi lähettää virheilmoitukset **standardivirtaan**, jotta käyttäjät voivat
yhä nähdä virheet, vaikka standardituloste ohjattaisiin tiedostoon. Ohjelmamme ei vielä toimi näin,
vaan tallentaa virheilmoituksen tiedostoon!

Havainnollistetaan tätä suorittamalla ohjelma ja ohjaamalla standardituloste tiedostoon **output.txt**.
Emme anna komentoriviargumentteja, mikä tuottaa virheen:

```console
$ cargo run > output.txt
```

`>`-merkki kertoo shellille, että standardituloste tulee ohjata tiedostoon _output.txt_.
Koska emme nähneet virheilmoitusta terminaalissa, se on todennäköisesti päätynyt tiedostoon.
Tarkastetaan tiedoston sisältö:

```text
Problem parsing arguments: not enough arguments
```

Kyllä, virheilmoitus on kirjoitettu standarditulosteeseen. On hyödyllisempää,
että virheilmoitukset tulostetaan standardivirtaan, jotta ne eivät sekoitu ohjelman normaaliin tulosteeseen.
Korjaamme tämän seuraavaksi.

### Virheiden tulostaminen standardivirtaan `eprintln!`-makrolla

Koska olemme aiemmin refaktoroineet ohjelmamme, kaikki virheilmoituksia tulostava koodi
on yhdessä paikassa, `main`-funktiossa. Rustin standardikirjasto tarjoaa **`eprintln!`-makron**,
joka tulostaa **standardivirtaan**. Vaihdamme nyt `println!`-kutsut käyttämään sitä.

<Listing number="12-24" file-name="src/main.rs" caption="Virheilmoitusten tulostaminen standardivirtaan käyttäen `eprintln!`">

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

</Listing>

Kokeillaan nyt ohjelman suorittamista samalla tavalla kuin aiemmin, ilman argumentteja,
ohjaten standardituloste tiedostoon:

```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```

Nyt virheilmoitus näkyy terminaalissa ja **output.txt**-tiedosto on tyhjä, mikä on komentoriviohjelmilta odotettu käytös.

Kokeillaan ohjelmaa uudelleen **onnistuneella ajolla**, mutta ohjaamalla standardituloste tiedostoon:

```console
$ cargo run -- to poem.txt > output.txt
```

Tässä tapauksessa **mitään ei tulosteta terminaaliin**, ja tulokset löytyvät **output.txt**-tiedostosta:

```text
Are you nobody, too?
How dreary to be somebody!
```

Tämä osoittaa, että käytämme nyt standarditulostetta onnistuneelle tulokselle ja standardivirtaa virheille,
mikä on oikein.

## Yhteenveto

Tässä luvussa kertasimme monia aiemmin oppimiamme konsepteja ja opimme, miten yleisiä I/O-toimintoja
toteutetaan Rustissa. Käyttämällä komentoriviargumentteja, tiedostoja, ympäristömuuttujia ja `eprintln!`-makroa
virheiden tulostamiseen, olemme nyt valmiita kirjoittamaan monipuolisia komentoriviohjelmia.

Aiemmista luvuista opitut periaatteet – koodin organisointi, tietorakenteiden tehokas käyttö,
virheiden käsittely ja testaus – yhdistettynä tässä luvussa opittuihin ominaisuuksiin antavat
hyvän pohjan jatkaa Rustin käyttöä tehokkaasti.

Seuraavaksi tutustumme joihinkin Rustin **funktionaaliseen ohjelmointiin liittyviin ominaisuuksiin**:
sulkeisiin (*closures*) ja iteraattoreihin.
