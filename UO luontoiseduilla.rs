use std::io;

fn main() {
    println!("Ulosottolaskuri");

    // Kysytään käyttäjältä kuukausittainen nettopalkka
    println!("Syötä kuukausittainen nettopalkka (euroa):");
    let mut net_income_input = String::new();
    io::stdin()
        .read_line(&mut net_income_input)
        .expect("Virhe syötteen lukemisessa");
    let net_income: f64 = net_income_input
        .trim()
        .parse()
        .expect("Syötä kelvollinen numero");

    // Kysytään luontoisedut, jotka lisätään nettopalkkaan
    println!("Syötä luontoisedut (euroa):");
    let mut benefits_input = String::new();
    io::stdin()
        .read_line(&mut benefits_input)
        .expect("Virhe syötteen lukemisessa");
    let benefits: f64 = benefits_input
        .trim()
        .parse()
        .expect("Syötä kelvollinen numero");

    // Kysytään kotitalouden jäsenten lukumäärä perussuojan laskentaa varten
    println!("Syötä kotitalouden jäsenten lukumäärä:");
    let mut persons_input = String::new();
    io::stdin()
        .read_line(&mut persons_input)
        .expect("Virhe syötteen lukemisessa");
    let persons: u32 = persons_input
        .trim()
        .parse()
        .expect("Syötä kelvollinen kokonaisluku");

    // Lasketaan perussuojamäärä
    let basic_protection = calculate_basic_protection(persons);
    println!("Perussuojamäärä: {:.2} euroa", basic_protection);

    // Yhdistetään nettopalkka ja luontoisedut
    let total_net = net_income + benefits;
    println!("Kokonaisnetto (nettopalkka + luontoisedut): {:.2} euroa", total_net);

    // Lasketaan ulosmittaukseen menevä osa: kokonaisnetosta vähennetään perussuojamäärä,
    // mikäli kokonaisnetto ylittää suojan.
    let garnishable_total = if total_net > basic_protection {
        total_net - basic_protection
    } else {
        0.0
    };

    // Esimerkinomaisesti käytetään 20 % ulosmittausprosenttina.
    let garnishment_rate = 0.20;
    let garnishment_amount = garnishable_total * garnishment_rate;

    println!("Ulosmitattava määrä: {:.2} euroa", garnishment_amount);
}

/// Laskee perussuojamäärän kotitalouden jäsenten lukumäärän perusteella.
/// Esimerkkisäännöt:
/// - Yhden hengen kotitaloudessa suoja-arvo on 1000 euroa
/// - Kahden hengen kotitaloudessa 1300 euroa
/// - Kolmen hengen kotitaloudessa 1550 euroa
/// - Neljän tai useamman hengen kotitaloudessa lisätään 150 euroa jokaisesta lisähenkilöstä
fn calculate_basic_protection(persons: u32) -> f64 {
    match persons {
        1 => 1000.0,
        2 => 1300.0,
        3 => 1550.0,
        n if n >= 4 => 1550.0 + 150.0 * ((n - 3) as f64),
        _ => 1000.0,
    }
}
