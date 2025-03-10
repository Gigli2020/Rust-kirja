## Asennus

Ensimmäinen askel on Rustin asentaminen. Lataamme Rustin käyttämällä `rustup`-työkalua, joka on komentorivityökalu Rust-versioiden ja niihin liittyvien työkalujen hallintaan. Tarvitset internet-yhteyden asennusta varten.

> Huom: Jos et halua käyttää `rustup`-työkalua, voit katsoa muita vaihtoehtoja [muut asennustavat][otherinstall] -sivulta.

Näiden ohjeiden mukaisesti asennat uusimman vakaan version Rust-kääntäjästä. Rustin vakaus takaa, että kaikki tämän kirjan esimerkit toimivat myös tulevissa Rust-versioissa. Tuloste voi hieman muuttua eri versioiden välillä, koska Rust parantaa virheilmoituksia ja varoituksia säännöllisesti. Lyhyesti sanottuna, mikä tahansa näiden ohjeiden mukaisesti asennettu Rust-versio toimii odotetusti tämän kirjan sisällön kanssa.

> ### Komentorivimerkinnät
>
> Tässä luvussa ja koko kirjassa esitämme komentoja, joita käytetään terminaalissa. Komennot, jotka sinun pitäisi kirjoittaa terminaaliin, alkavat merkillä `$`. Sinun ei tarvitse kirjoittaa `$`-merkkiä; se on vain komentorivin kehotteena. Rivien, jotka eivät ala `$`-merkillä, tulisi esittää aiemman komennon tulosteita. Lisäksi PowerShell-esimerkeissä käytetään `>`-merkkiä `$`-merkin sijasta.

### `rustup`-asennus Linuxilla tai macOS:llä

Jos käytät Linuxia tai macOS:ää, avaa terminaali ja suorita seuraava komento:

```console
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

Tämä komento lataa skriptin ja käynnistää `rustup`-työkalun asennuksen, joka asentaa uusimman vakaan Rust-version. Saatat joutua syöttämään salasanasi. Jos asennus onnistuu, näet seuraavan viestin:

```text
Rust on nyt asennettu. Mahtavaa!
```

Tarvitset myös _linkkerin_, joka yhdistää Rustin kääntämät tiedostot yhdeksi ohjelmaksi. Sinulla saattaa jo olla sellainen. Jos saat linkkerivirheitä, sinun kannattaa asentaa C-kääntäjä, joka yleensä sisältää linkkerin. C-kääntäjä on hyödyllinen myös siksi, että monet Rust-kirjastot riippuvat C-koodista ja tarvitsevat kääntäjän.

macOS:llä voit asentaa C-kääntäjän komennolla:

```console
$ xcode-select --install
```

Linux-käyttäjien tulisi asentaa GCC tai Clang jakelunsa ohjeiden mukaisesti. Esimerkiksi Ubuntussa voit asentaa `build-essential`-paketin.

### `rustup`-asennus Windowsilla

Windowsissa siirry osoitteeseen [https://www.rust-lang.org/tools/install][install] ja seuraa Rustin asennusohjeita. Asennuksen aikana sinua pyydetään asentamaan Visual Studio, joka tarjoaa linkkerin ja tarvittavat kirjastot ohjelmien kääntämiseen. Jos tarvitset lisäapua, katso [Windows MSVC -asennusohjeet][msvc].

Tämän kirjan jatko-osissa käytetään komentoja, jotka toimivat sekä **cmd.exe**- että **PowerShell**-ympäristöissä. Jos on eroja niiden välillä, mainitsemme, kumpaa käyttää.

### Vianmääritys

Voit tarkistaa, onko Rust asennettu oikein, avaamalla komentorivin ja kirjoittamalla:

```console
$ rustc --version
```

Näet versionumeron, commit-hashin ja commit-päivämäärän seuraavassa muodossa:

```text
rustc x.y.z (abcabcabc yyyy-mm-dd)
```

Jos näet tämän tiedon, Rust on asennettu onnistuneesti! Jos et näe tätä, tarkista, onko Rust polussasi (`%PATH%`-järjestelmämuuttuja).

Windows CMD:ssä:

```console
> echo %PATH%
```

PowerShellissä:

```powershell
> echo $env:Path
```

Linuxissa ja macOS:ssä:

```console
$ echo $PATH
```

Jos kaikki näyttää oikealta, mutta Rust ei silti toimi, voit saada apua Rustin [yhteisösivulta][community].

### Päivittäminen ja poistaminen

Kun Rust on asennettu `rustup`-työkalun kautta, uusimman version päivittäminen on helppoa. Suorita komentorivillä:

```console
$ rustup update
```

Jos haluat poistaa Rustin ja `rustup`-työkalun, suorita seuraava komento:

```console
$ rustup self uninstall
```

### Paikallinen dokumentaatio

Rustin asennuksen mukana tulee paikallinen kopio dokumentaatiosta, joten voit lukea sitä myös offline-tilassa. Avaa dokumentaatio selaimessa komennolla:

```console
$ rustup doc
```

Jos olet epävarma jostakin Rustin tyyppistä tai funktiosta, voit etsiä vastauksen ohjelmointirajapinnan (API) dokumentaatiosta!

### Tekstieditorit ja kehitysympäristöt

Tämä kirja ei edellytä tietyn työkalun käyttöä Rust-koodin kirjoittamiseen. Melkein mikä tahansa tekstieditori riittää! Kuitenkin monet editorit ja integraatioympäristöt (IDEt) tarjoavat Rust-tukea. Löydät ajantasaisen listan tuetuista editoreista ja IDE-ympäristöistä [Rustin työkalusivulta][tools].

[otherinstall]: https://forge.rust-lang.org/infra/other-installation-methods.html
[install]: https://www.rust-lang.org/tools/install
[msvc]: https://rust-lang.github.io/rustup/installation/windows-msvc.html
[community]: https://www.rust-lang.org/community
[tools]: https://www.rust-lang.org/tools
