use rand::{thread_rng,Rng}; // Fonction de la dependence Rand
use std::{io::{self, Write}};

fn find_letter(letter :char,words:&str)-> Vec<usize> {

    let mut index_tab: Vec<usize> =vec![];
    for (i, c) in words.chars().enumerate() {
        if c == letter { 
            index_tab.push(i);
        }    
    }
    index_tab
}

fn lire_proposition (message:&str) -> Option<char> {
    loop{

        println!("{}",message);
        let _ = io::stdout().flush();
        let mut saisie :String= String::new();

        match io::stdin().read_line(&mut saisie){
            Ok(0) => return None,
            Ok(_) => {
                if  saisie.trim().chars().count() == 1{
                    
                    if let Some(c) = saisie.trim().chars().next(){
                        if c.is_alphabetic() {
                            return  c.to_lowercase().next();
                        }
                        else {
                            println!("ATTENTION !!! Vous devez rentrer uniquement un caractère alphabétique . les chiffres ne sont pas autorisés ."); 
                        }
                    }
                }
                else {
                    println!("ATTENTION !!! Vous devez rentrer uniquement un caractère alphabétique .");
                }
            },
            Err(error)=>{
                eprintln!("erreur:{error}");
                return None ;
            },

        }
    }
}

fn main() {
    let mut pv :u8 = 10;
    let words: [&str; 100] = [
        "abracadabra", "acrobatie", "alchimiste", "anachronisme", "apothicaire", "archipel", "astrophysique", "autobiographie", "avalanche", "bibliothèque",
        "bouleversement", "boulangerie", "bureaucratie", "calligraphie", "cataclysme", "catastrophe", "chrysanthème", "cinématographe", "clairvoyance", "claustrophobie",
        "constellation", "contrebandier", "crocodile", "cryptographie", "cyclone", "dictionnaire", "dinosaure", "diplomatique", "disparition", "dysfonctionnement",
        "éclaircie", "électricité", "encyclopédie", "énigmatique", "équilibriste", "escalade", "exceptionnel", "excentrique", "expédition", "extraordinaire",
        "fantasmagorie", "fluorescent", "forteresse", "gastronomie", "gymnastique", "hallucination", "hiéroglyphe", "hippopotame", "horizontal", "hypnotiseur",
        "imagination", "incandescent", "inexplicable", "interrogation", "kaléidoscope", "kilométrage", "labyrinthe", "laboratoire", "luxueux", "magnifique",
        "mathématiques", "mélancolique", "métamorphose", "microscope", "mystificateur", "nostalgique", "obscurantisme", "orchestre", "ouragan", "palindrome",
        "papillon", "paranormal", "parallélépipède", "philosophie", "pneumatique", "psychologie", "pyramide", "quadrilatère", "quintessence", "rhinocéros",
        "rhododendron", "scaphandrier", "silhouette", "somnambule", "spectaculaire", "stratagème", "symphonie", "synchronisation", "technologie", "télescope",
        "thermomètre", "tourbillon", "transparence", "tremblement", "trompette", "typographie", "vertigineux", "volcanologue", "xylophone", "zeppelin"
    ]; // Tableau de mots.
    
    println!("😃 Salut je m'appèlle Jason, j'ai crée le jeu : Deviner un mots en rust😃");
    println!("Mon objèctif est d'apprendre le rust mais aussi vous amuser avec mon jeu .");
    println!(" Règles du jeu :\n-Pour commencer il vous sera donné le nombre de lèttres que contient le mots .\n-Vous choisissez une lettre et si la lèttre est comprise dans le mots, on vous le dira.\n-Vous saurez le nombre de fois que la lettre apparait dans le mots ainsi que son ou ses emplacements dans le mots .\n\n NB: Les mots contiènnent des accents afin d'augmenter la difficulté du jeu😁😁😁😁.\n");

    println!("A présent, commençons :");

    while pv !=0 {

        let words_index : usize = thread_rng().gen_range(0..words.len());
        let words_choosed: &str = words[words_index];
        let mut tab_letter_used: Vec<char> = vec![];

        println!("Le mots contient {} lettres.",words_choosed.chars().count());

        let mut ch_letter : Vec<char>=vec!['_';words_choosed.chars().count()];

        let mut word_form: String = ch_letter.iter().collect();

        while (word_form != words_choosed) && (pv!=0) {


            let Some(lettre) = lire_proposition("Entrez une lettre :") else {return ;};
                
            let test :Vec<usize> = find_letter(lettre ,words_choosed);

            if !test.is_empty() {
                if !tab_letter_used.contains(&lettre) {
                    tab_letter_used.push(lettre);
                } else {
                    println!("Cette lettre a déja été citée .");
                    continue;
                }

                for i in test {
                    ch_letter[ i] = lettre; 
                }
                
                word_form = ch_letter.iter().collect();

                println!("La lettre '{}' appartient au mots mystère.",lettre);
                println!("Proposition = {}",word_form);
                println!(" lettres déja utilisé : {:?}",tab_letter_used);

            }
            
            else {
                if !tab_letter_used.contains(&lettre) {
                    tab_letter_used.push(lettre);
                    pv-=1;
                    
                } else {
                    println!("Cette lettre a déja été citée .");
                    continue;
                }
                
                println!(" La lettre {} n'appartient pas au mots mystère.",lettre);
                println!(" lettres déja utilisé : {:?}",tab_letter_used);
                println!("Proposition = {}",word_form);

                
                if pv ==0 {
                    println!(" vous avez pèrdu toutes vos points de vie . Désolé");
                    println!(" le mots était :{} .",words_choosed);
                }
                
            }

            if pv != 0 && word_form != words_choosed{
                println!("PV :{}","❤️".repeat(pv as usize));
            }

        }
        if pv != 0{
            println!("🏆 Bravo , vous avez deviné le mots mystère 🏆.");
            println!(" le mots était bel et bien :{} .",words_choosed);
        }
        
        if lire_proposition("Voulez-vous continuer à jouer? y/n").unwrap_or(' ') == 'y' {
            pv = 10;
        } else {
            break;
        }
    }
}


