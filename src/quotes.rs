use rand::seq::SliceRandom;

/// Cada quote es una tupla: (texto, quién lo dice)
/// `&str` = referencia a texto que vive en el binario (no se copia, no se aloca)
const QUOTES: &[(&str, &str)] = &[
    ("I used to be an adventurer like you, then I took an arrow in the knee.", "Whiterun Guard"),
    ("FUS RO DAH!", "Dragonborn"),
    ("Do you get to the Cloud District very often? Oh, what am I saying, of course you don't.", "Nazeem"),
    ("Let me guess... someone stole your sweetroll.", "Guard"),
    ("I am Alduin, firstborn of Akatosh! I am the world-eater!", "Alduin"),
    ("What is better - to be born good, or to overcome your evil nature through great effort?", "Paarthurnax"),
    ("My ancestors are smiling at me, Imperials. Can you say the same?", "Stormcloak Soldier"),
    ("I mostly deal with petty thievery and drunken brawls. Been too long since we've had a good bandit raid.", "Guard"),
    ("Disrespect the law, and you disrespect me.", "Guard"),
    ("No lollygagging.", "Guard"),
    ("I am sworn to carry your burdens.", "Lydia"),
    ("You have committed crimes against Skyrim and her people. What say you in your defense?", "Guard"),
    ("Skyrim belongs to the Nords!", "Stormcloak"),
    ("I fight for the men I've held in my arms, dying on foreign soil.", "Ulfric Stormcloak"),
    ("By the order of the Jarl, stop right there!", "Guard"),
    ("Hey, you. You're finally awake.", "Ralof"),
    ("I need to ask you to stop. That... shouting... is making people nervous.", "Guard"),
    ("War doesn't determine who's right, only who's left.", "Ulfric Stormcloak"),
    ("The next person that calls me chicken is getting the axe!", "Guard"),
    ("Dovahkiin, Dovahkiin, naal ok zin los vahriin!", "Bard"),
    ("You never should have come here!", "Bandit"),
    ("I'll have you know that there's no PUSSIEEEEE!", "Bandit"),
    ("Another wanderer, here to lick my father's boots. Good job.", "Balgruuf's Kid"),
    ("Hail Sithis!", "Dark Brotherhood"),
];

/// Retorna un quote aleatorio formateado
pub fn random_quote() -> String {
    let mut rng = rand::thread_rng();
    let (quote, author) = QUOTES.choose(&mut rng).unwrap();
    format!("\"{}\" --{}", quote, author)
}
