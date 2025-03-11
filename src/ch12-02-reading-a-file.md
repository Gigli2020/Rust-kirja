## Tiedoston lukeminen

Seuraavaksi lisätään toiminnallisuus, joka lukee `file_path`-argumentilla määritellyn tiedoston. Tarvitsemme ensin testitiedoston, jolla voimme kokeilla toimintoa. Käytämme tiedostoa, jossa on pieni määrä tekstiä useilla riveillä ja joitain toistuvia sanoja. Listing 12-3 sisältää Emily Dickinsonin runon, joka sopii hyvin testitapaukseksi!

Luo projektisi juuritason hakemistoon tiedosto nimeltä _poem.txt_ ja kirjoita siihen runo “I’m Nobody! Who are you?”.

<Listing number="12-3" file-name="poem.txt" caption="Emily Dickinsonin runo toimii hyvänä testiaineistona.">

```text
{{#include ../listings/ch12-an-io-project/listing-12-03/poem.txt}}
```

</Listing>

Kun teksti on lisätty tiedostoon, muokkaa _src/main.rs_-tiedostoa ja lisää siihen koodi, joka lukee tiedoston sisällön, kuten Listing 12-4 näyttää.

<Listing number="12-4" file-name="src/main.rs" caption="Toisen argumentin määrittämän tiedoston lukeminen">

```rust,should_panic,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/src/main.rs:here}}
```

</Listing>

Ensimmäiseksi tuomme standardikirjastosta tarvittavan osan `use`-lauseella: tarvitsemme `std::fs`-moduulin tiedostojen käsittelyä varten.

`main`-funktiossa uusi `fs::read_to_string`-kutsu ottaa `file_path`-muuttujan, avaa tiedoston ja palauttaa `std::io::Result<String>`-tyyppisen arvon, joka sisältää tiedoston sisällön.

Lisäämme jälleen väliaikaisen `println!`-lauseen, joka tulostaa `contents`-muuttujan arvon tiedoston lukemisen jälkeen, jotta voimme tarkistaa, että ohjelma toimii odotetusti.

Suoritetaan ohjelma antamalla mikä tahansa merkkijono ensimmäiseksi komentoriviargumentiksi (koska emme ole vielä toteuttaneet hakutoimintoa) ja _poem.txt_-tiedosto toiseksi argumentiksi:

```console
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-04/output.txt}}
```

Hienoa! Koodi luki tiedoston ja tulosti sen sisällön. Mutta tässä on muutamia ongelmia. Tällä hetkellä `main`-funktiolla on useita vastuita: yleensä funktiot ovat selkeämpiä ja helpompia ylläpitää, jos ne hoitavat vain yhden asian kerrallaan. Toinen ongelma on, että emme käsittele virheitä niin hyvin kuin voisimme. Ohjelma on vielä pieni, joten nämä puutteet eivät ole suuria ongelmia, mutta kun ohjelma kasvaa, niiden korjaaminen siististi vaikeutuu.

On hyvä käytäntö aloittaa refaktorointi varhaisessa vaiheessa ohjelmistokehitystä, koska pienempi määrä koodia on helpompi jäsentää uudelleen. Teemme sen seuraavaksi.
