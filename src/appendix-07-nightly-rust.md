## Liite G - Miten Rustia kehitetään ja mitä on “Nightly Rust”

Tässä liitteessä käsitellään Rustin kehitysprosessia ja sen vaikutuksia Rust-kehittäjiin.

### Vakautta ilman pysähtyneisyyttä

Rust panostaa erityisen paljon koodin vakauteen. Tarkoituksena on, että Rust toimii **luotettavana perustana**, jonka varaan voi rakentaa. Jos kieli muuttuisi jatkuvasti, tämä olisi mahdotonta. Toisaalta, jos uusia ominaisuuksia ei voisi kokeilla ennen niiden julkaisua, mahdolliset ongelmat huomattaisiin vasta, kun muutoksia ei enää voida tehdä.

Ratkaisumme tähän ongelmaan on **"vakautta ilman pysähtyneisyyttä"**, ja ohjaavana periaatteena on tämä: **Rustin päivittämisen ei tulisi koskaan aiheuttaa pelkoa**. Jokaisen uuden version tulisi olla helppo ottaa käyttöön, tuoda uusia ominaisuuksia, vähentää bugeja ja nopeuttaa kääntämistä.

### Julkaisumalli – "junamalli"

Rust kehitetään **"junamallilla"**, jossa kaikki kehitystyö tehdään **`master`**-haarassa. Julkaisut tapahtuvat ohjelmistojen julkaisujunamallin mukaisesti, jota ovat käyttäneet muun muassa **Cisco IOS** ja muut suuret ohjelmistoprojektit. Rustilla on kolme **julkaisukanavaa**:

- **Nightly** (yöllinen koonti uusilla ominaisuuksilla)
- **Beta** (testausvaihe ennen vakaata julkaisua)
- **Stable** (vakaa, tuotantokäyttöön tarkoitettu versio)

Useimmat Rust-kehittäjät käyttävät ensisijaisesti **vakaa (stable)** -versiota, mutta kehittäjät, jotka haluavat kokeilla **uusimpia kokeellisia ominaisuuksia**, käyttävät **nightly**- tai **beta**-versioita.

Joka **kuudes viikko**, uusi beta-julkaisu haarautuu `master`-haarasta. Tämän jälkeen se testataan, ja **vakaa julkaisu** tehdään beta-haarasta kuuden viikon kuluttua.

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

Koska Rustin julkaisut ovat **aikataulutettuja**, jokainen uusi ominaisuus, joka ei ehdi tiettyyn julkaisuun, **tulee mukaan seuraavaan**. Tämä vähentää painetta yrittää saada keskeneräisiä ominaisuuksia mukaan viime hetkellä.

### Ylläpito ja tuen elinkaari

Rust-projekti tukee **vain uusinta vakaata versiota**. Kun uusi vakaa versio julkaistaan, vanha versio **ei enää saa virhekorjauksia**. Tämä tarkoittaa, että jokainen versio saa tukea **vain kuusi viikkoa**.

### Epävakaat ominaisuudet (Unstable Features)

Rustin kehitysmallissa kokeelliset ominaisuudet lisätään ensin **`master`-haaraan**, mutta ne pidetään **feature flag** -valinnan takana. Tämä tarkoittaa, että:

- Jos käytät **nightly**-versiota, voit ottaa kokeelliset ominaisuudet käyttöön **feature flag** -määritteellä.  
- Jos käytät **beta- tai stable**-versiota, **kokeellisia ominaisuuksia ei voi käyttää**.  

Tämä mahdollistaa uusien ominaisuuksien **testaamisen ja arvioinnin** ennen niiden lopullista vakauttamista. **Vakaa Rust** tarjoaa rock-solid-kokemuksen ilman odottamattomia muutoksia.

Tämä kirja käsittelee **vain vakaita ominaisuuksia**, sillä kehitteillä olevat ominaisuudet voivat muuttua ennen kuin ne tulevat vakaisiin julkaisuihin. **Kokeellisten ominaisuuksien dokumentaatio löytyy Rustin virallisilta sivuilta.**

### Rustup ja Nightly Rustin käyttö

`rustup`-työkalu helpottaa eri Rust-julkaisukanavien hallintaa sekä koko järjestelmässä että projektikohtaisesti. Oletuksena asennettuna on **vakaa Rust**. Voit asentaa **nightly**-version komennolla:

```console
$ rustup toolchain install nightly
```

Voit tarkistaa asennetut Rust-versiot seuraavalla komennolla:

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

Jos haluat käyttää **nightly**-versiota vain tietyssä projektissa, voit tehdä sen seuraavasti:

```console
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```

Tämä varmistaa, että **vain kyseisessä projektikansiossa** käytetään nightly-versiota, vaikka järjestelmän oletus olisi vakaa Rust.

### RFC-prosessi ja Rustin kehitysryhmät

Rustin kehitystä ohjaa **RFC-prosessi (Request For Comments)**. Jos haluat parannuksia Rustiin, voit kirjoittaa **RFC-ehdotuksen**, jonka Rust-tiimi arvioi.

Rust-tiimi koostuu useista **aihekohtaisista tiimeistä**, kuten:

- **Kielisuunnittelu** (language design)  
- **Kääntäjän toteutus** (compiler implementation)  
- **Infra ja työkalut** (infrastructure & tooling)  
- **Dokumentaatio** (documentation)  

Täydellinen lista tiimeistä löytyy [Rustin virallisilta sivuilta](https://www.rust-lang.org/governance).

Jos RFC hyväksytään, siitä avataan **GitHub-issue**, jonka kuka tahansa voi toteuttaa. Kun toteutus valmistuu, se lisätään **master-haaraan**, mutta se pidetään ensin kokeellisena ominaisuutena **feature flagin takana**.

Kun tarpeeksi **nightly-käyttäjiä on testannut ominaisuutta**, Rust-tiimi arvioi, pitäisikö se tuoda vakaaseen Rustiin. Jos kyllä, **feature flag poistetaan** ja ominaisuus lisätään vakaaseen julkaisuun seuraavassa aikataulun mukaisessa versiossa.

Tämä prosessi varmistaa, että Rust **kehittyy jatkuvasti ilman että vakaus kärsii** – eli **"stability without stagnation"**.

