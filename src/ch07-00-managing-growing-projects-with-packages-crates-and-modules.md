# Suurten projektien hallinta pakettien, laatikoiden ja moduulien avulla

Kun ohjelmasi kasvaa, sen rakenteen järjestäminen tulee yhä tärkeämmäksi. Ryhmittelemällä toisiinsa liittyvät toiminnallisuudet ja erottamalla erilliset ominaisuudet koodista voit helpottaa koodin hallintaa. Tämä tekee selväksi, mistä tietty ominaisuus löytyy ja missä sitä voi muokata.

Tähän asti kirjoittamamme ohjelmat ovat olleet yhdessä moduulissa ja yhdessä tiedostossa. Kun projekti kasvaa, koodi kannattaa jakaa useisiin moduuleihin ja tiedostoihin. Paketti voi sisältää useita binäärisiä laatikoita (_crates_) sekä valinnaisesti yhden kirjastolaatikon. Kun paketti kasvaa, voit erottaa osia omiksi laatikoiksi, jolloin ne voivat toimia ulkoisina riippuvuuksina. Tämä luku kattaa nämä tekniikat. Suurille projekteille, joissa on useita toisiinsa liittyviä paketteja, Cargo tarjoaa _työtilat_ (_workspaces_), joita käsittelemme luvussa 14.

Käsittelemme myös yksityiskohtien kapselointia, joka mahdollistaa koodin uudelleenkäytön korkeammalla tasolla. Kun olet toteuttanut jonkin toiminnon, muu koodi voi kutsua sitä sen julkisen rajapinnan kautta ilman, että tarvitsee tietää toteutuksen yksityiskohtia. Tämä rajaa sitä tietomäärää, joka sinun täytyy pitää mielessäsi.

Aiheeseen liittyy myös näkyvyysalue (_scope_): koodin kirjoittamisen konteksti määrää, mitkä nimet ovat käytettävissä. Kun ohjelmoija tai kääntäjä käsittelee koodia, sen on tiedettävä, viittaako jokin nimi muuttujaan, funktioon, rakenteeseen, enum-tyyppiin, moduuliin, vakioon tai muuhun. Voit määritellä näkyvyysalueita ja päättää, mitkä nimet ovat käytettävissä missäkin kohdassa. Et voi antaa kahta saman nimistä elementtiä samassa näkyvyysalueessa, mutta Rust tarjoaa työkaluja nimikonfliktien ratkaisemiseen.

Rust tarjoaa useita ominaisuuksia, joilla voit hallita koodisi rakennetta, mukaan lukien sen, mitkä yksityiskohdat ovat julkisia, mitkä yksityisiä ja mitkä nimet ovat käytettävissä missäkin kontekstissa. Näitä ominaisuuksia kutsutaan usein _moduulijärjestelmäksi_, ja ne sisältävät:

- **Paketit (_Packages_)**: Cargo-ominaisuus, joka mahdollistaa laatikoiden rakentamisen, testaamisen ja jakamisen.
- **Laatikot (_Crates_)**: Moduulipuu, joka tuottaa kirjaston tai suoritettavan tiedoston.
- **Moduulit ja `use`**: Ohjaavat koodin rakennetta, näkyvyyttä ja polkujen käyttöä.
- **Polut (_Paths_)**: Määrittävät, miten nimetään elementtejä, kuten rakenteita, funktioita tai moduuleja.

Tässä luvussa käsittelemme näitä ominaisuuksia, niiden yhteistoimintaa ja sitä, miten niitä käytetään näkyvyysalueiden hallintaan. Tavoitteena on, että tämän luvun jälkeen ymmärrät Rustin moduulijärjestelmän ja pystyt hallitsemaan näkyvyysalueita sujuvasti!
