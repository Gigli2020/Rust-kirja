## Suorituskyvyn vertailu: Silmukat vs. iteraattorit

Kun päätetään, käytetäänkö silmukoita vai iteraattoreita, on tärkeää tietää,
kumpi toteutus on nopeampi: `search`-funktion versio, joka käyttää `for`-silmukkaa,
vai versio, joka käyttää iteraattoreita.

Teimme vertailun lataamalla **Sherlock Holmesin seikkailut** -kirjan kokonaisuudessaan
`String`-muuttujaan ja etsimällä tekstistä sanaa _the_. Seuraavat tulokset saatiin
ajamalla testi `search`-funktion versiolle, joka käyttää `for`-silmukkaa, ja versiolle,
joka käyttää iteraattoreita:

```text
test bench_search_for  ... bench:  19,620,300 ns/iter (+/- 915,700)
test bench_search_iter ... bench:  19,234,900 ns/iter (+/- 657,200)
```

Molemmilla toteutuksilla on **käytännössä sama suorituskyky**! Emme syvenny
tässä testikoodiin, sillä tarkoitus ei ole todistaa, että nämä kaksi versiota
ovat täysin identtisiä, vaan antaa yleinen käsitys siitä, miten suorituskyky
vertailtavissa.

Jos haluat perusteellisemman suorituskykytestin, voit kokeilla eri tekstien,
eri kokoisten `contents`-syötteiden ja eripituisten hakusanojen vaikutusta.
Tärkeintä on huomata, että vaikka iteraattorit ovat **korkean tason abstraktioita**,
Rust kääntää ne **suunnilleen samaan koodiin** kuin jos olisit kirjoittanut alemman tason
silmukkakoodin itse.

Tämä tekee iteraattoreista yhden Rustin **nollakustannusabstraktioista** (*zero-cost abstractions*),
eli abstraktioiden käyttäminen ei lisää suorituskykykustannuksia ajonaikana. Tämä on
samanlainen periaate kuin C++-kielen luoja **Bjarne Stroustrup** kuvaili vuonna 2012:

> Yleisesti ottaen C++-toteutukset noudattavat **nollakustannusperiaatetta**:
> mitä et käytä, siitä et maksa. Lisäksi: mitä käytät, et voisi kirjoittaa
> käsin tehokkaammin.

### Käytännön esimerkki: Äänidekooderin optimointi

Seuraava koodi on otettu äänidekooderista. Algoritmi käyttää **lineaarista ennustamista**,
joka arvioi tulevia arvoja aiempien näytteiden perusteella. Seuraava esimerkki käyttää
iteraattoriketjua suorittamaan laskutoimituksia kolmella muuttujalla:

- `buffer`: Viipale (slice), joka sisältää aiemmat arvot.
- `coefficients`: 12:n kertoimen taulukko.
- `qlp_shift`: Luku, jolla tulos siirretään oikealle.

Tämän esimerkin muuttujia ei ole alustettu, mutta koodi havainnollistaa,
miten Rust muuntaa korkean tason ajatukset tehokkaaksi alemman tason koodiksi.

```rust,ignore
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

Tässä koodissa `prediction`-arvo lasketaan seuraavasti:

1. Käydään läpi kaikki `coefficients`-taulukon 12 arvoa.
2. Käytetään `zip`-metodia yhdistämään jokainen kerroin `buffer`-taulukon 12 edelliseen arvoon.
3. Jokainen kerroin kerrotaan yhdistetyllä `buffer`-arvolla.
4. Tulokset summataan yhteen.
5. Lopuksi tulosta siirretään `qlp_shift`-bittimäärällä oikealle.

Äänidekoodereiden kaltaisissa sovelluksissa suorituskyky on **erittäin tärkeää**.
Tässä koodissa luodaan iteraattori, käytetään kahta sovitinta (`zip` ja `map`),
ja kulutetaan tulos `sum`-metodilla. Mitä Rust kääntää tästä?

Nykyisten optimointien ansiosta **käännetty assembly-koodi vastaa käsin optimoitua koodia**.
Rust pystyy **"rullaamaan"** (*unrolling*) silmukan, koska se tietää, että iteraatioita
on aina tasan 12. Tämä poistaa silmukan hallintakustannukset ja generoi toistuvan koodin
suoraan käännöksessä.

Lisäksi **kaikki kertoimet tallennetaan suoraan rekistereihin**, mikä tekee
niihin pääsystä erittäin nopeaa. Rust myös **poistaa turhat taulukon rajatarkistukset**,
koska se voi päätellä turvallisuuden kääntöaikana. Kaikki nämä optimoinnit tekevät
lopullisesta konekoodista erittäin tehokasta.

## Yhteenveto

Sulkeiset ja iteraattorit ovat Rustin ominaisuuksia, jotka saavat vaikutteita
funktionaalisista ohjelmointikielistä. Ne auttavat ilmaisemaan **korkean tason ajatuksia**
ilman suorituskykykustannuksia. Rustin **iteraattorit ja sulkeiset eivät hidasta koodia**,
koska kääntäjä optimoi ne alemman tason koodiksi tehokkaasti.

Nyt kun olemme optimoineet I/O-projektimme ja tehneet siitä selkeämmän,
siirrytään seuraavaksi **Cargo-työkalun lisäominaisuuksiin**, joiden avulla
voimme jakaa projektimme maailmalle!
