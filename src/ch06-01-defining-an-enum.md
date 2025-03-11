## Luettelotyypin (Enum) määrittely

Rakenteet mahdollistavat toisiinsa liittyvien tietojen ryhmittelyn, kuten `Rectangle`-rakenne, jolla on `width` ja `height`. Luettelotyypit (_enums_) sen sijaan tarjoavat tavan määritellä arvo, joka voi olla yksi useista vaihtoehdoista. Esimerkiksi `Rectangle` voisi olla yksi mahdollinen muoto muiden, kuten `Circle` ja `Triangle`, joukossa. Rustin _enum_-ominaisuuden avulla voimme ilmaista nämä vaihtoehdot yksiselitteisesti.

### Enumien käyttö IP-osoitteiden käsittelyssä

Oletetaan, että ohjelmamme käsittelee IP-osoitteita. Nykyisin käytössä on kaksi pääasiallista IP-osoitetyyppiä: IPv4 ja IPv6. Koska nämä ovat ainoat mahdolliset vaihtoehdot, voimme luetella ne Rustin _enum_-määrittelyllä:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:def}}
```

Nyt `IpAddrKind` on mukautettu tietotyyppi, jota voimme käyttää ohjelmassamme. 

### Enum-arvojen luominen ja käyttö

Voimme luoda kunkin variantin ilmentymiä näin:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:instance}}
```

Enumien variantit ovat nimettyjä `IpAddrKind`-tyypin sisällä ja niitä käytetään kaksoispisteoperaattorilla `::`.

Voimme myös määritellä funktion, joka hyväksyy `IpAddrKind`-arvon:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn}}
```

Tätä funktiota voidaan kutsua molemmilla enum-arvoilla:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-01-defining-enums/src/main.rs:fn_call}}
```

### Enum vs. rakenne

Voimme yhdistää IP-osoitteen tietoa rakenteeseen, kuten seuraavassa esimerkissä:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-01/src/main.rs:here}}
```

Tässä `IpAddr`-rakenne sisältää `kind`-kentän, joka kertoo osoitteen tyypin (`IpAddrKind`), ja `address`-kentän, joka tallentaa itse osoitteen merkkijonona.

Kuitenkin enumien avulla voimme tehdä tämän tiiviimmin ja ilman erillistä rakennetta:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-02-enum-with-data/src/main.rs:here}}
```

Tässä `V4` ja `V6` -variantit sisältävät osoitetiedon suoraan ilman erillistä rakennetta.

### Eri tietotyypit enumissa

Enumien etuna on myös se, että eri variantit voivat sisältää eri määrän ja tyyppisiä tietoja:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-03-variants-with-different-data/src/main.rs:here}}
```

Tässä `V4` sisältää neljä `u8`-arvoa, kun taas `V6` sisältää merkkijonon.

### Standardikirjaston `IpAddr`-määrittely

Koska IP-osoitteiden käsittely on yleistä, Rustin standardikirjasto tarjoaa jo valmiin `IpAddr`-määrittelyn:

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Tämä osoittaa, että enum-varianteissa voi olla myös muita rakenteita.

### Toinen esimerkki: `Message`-enum

Enumien avulla voidaan myös mallintaa viestejä eri tiedoilla:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-02/src/main.rs:here}}
```

Tässä `Message`-enumilla on neljä varianttia:

- `Quit`, jolla ei ole mitään dataa.
- `Move`, joka sisältää kaksi `i32`-kenttää.
- `Write`, joka sisältää `String`-arvon.
- `ChangeColor`, joka sisältää kolme `i32`-arvoa.

Tämä vastaisi seuraavaa rakennekokoelmaa:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-04-structs-similar-to-message-enum/src/main.rs:here}}
```

Mutta erillisten rakenteiden sijaan enum mahdollistaa yhden tietotyypin käytön kaikille varianteille.

### Metodien määrittely enumeille

Kuten rakenteille, voimme myös määritellä metodeja enum-arvoille:

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-05-methods-on-enums/src/main.rs:here}}
```

Tässä `call`-metodi voidaan kutsua `Message`-ilmentymälle.

Seuraavaksi tutustumme `Option`-enum-arvoon, joka mahdollistaa arvojen olemassaolon tai puuttumisen hallinnan ilman `null`-arvoja.
