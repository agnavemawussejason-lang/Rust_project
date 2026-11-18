use rand::{thread_rng,Rng}; // Fonction de la dependence Rand
use std::io::{self, Write};

fn find_letter(letter :char,words:&str)-> Vec<u8> {

    let mut index_tab: Vec<u8> =vec![];
    let mut index_value :u8 = 0;

    for i in words.chars() {
        if i == letter { 
            index_tab.push(index_value);
        }
        index_value += 1
    }

    index_tab
}

fn lire_proposition () -> Option<char> {
    loop{

        println!("Entrez une lettre :");
        let _ = io::stdout().flush();
        let mut saisie :String= String::new();

        match io::stdin().read_line(&mut saisie){
            Ok(0) => return None,
            Ok(_) => {
                if  saisie.trim().chars().count() == 1{
                    
                    if let Some(c) = saisie.trim().chars().next(){
                        if c.is_alphabetic() {
                            return  Some(c.to_ascii_lowercase());
                        }
                        else {
                            println!("ATTENTION !!! Vous devez rentrer uniquement une lettre ."); 
                        }
                    }
                    else {
                        println!("ATTENTION !!! veuillez ne pas rentrer des caractères vides . ");
                    }
                }
                else {
                    println!("ATTENTION !!! Vous devez rentrer uniquement une lettre .");
                }
            },
            Err(error)=>{
                eprintln!("erreur:{error}");
                saisie.clear();
                return None ;
            },

        }
    }
}


fn main() {
    let mut pv :u8 = 10;
    let words: [&str; 100] = [
        "abracadabra", "acrobatie", "alchimiste", "anachronisme", "apothicaire", "archipel", "astrophysique", "autobiographie", "avalanche", "bibliotheque",
        "bouleversement", "boulangerie", "bureaucratie", "calligraphie", "cataclysme", "catastrophe", "chrysanthème", "cinematographe", "clairvoyance", "claustrophobie",
        "constellation", "contrebandier", "crocodile", "cryptographie", "cyclone", "dictionnaire", "dinosaure", "diplomatique", "disparition", "dysfonctionnement",
        "eclaircie", "electricite", "encyclopedie", "enigmatique", "equilibriste", "escalade", "exceptionnel", "excentrique", "expedition", "extraordinaire",
        "fantasmagorie", "fluorescent", "forteresse", "gastronomie", "gymnastique", "hallucination", "hieroglyphe", "hippopotame", "horizontal", "hypnotiseur",
        "imagination", "incandescent", "inexplicable", "interrogation", "kaleidoscope", "kilometrage", "labyrinthe", "laboratoire", "luxueux", "magnifique",
        "mathematiques", "melancolique", "metamorphose", "microscope", "mystificateur", "nostalgique", "obscurantisme", "orchestre", "ouragan", "palindrome",
        "papillon", "paranormal", "parallelepipede", "philosophie", "pneumatique", "psychologie", "pyramide", "quadrilatere", "quintessence", "rhinoceros",
        "rhododendron", "scaphandrier", "silhouette", "somnambule", "spectaculaire", "stratagème", "symphonie", "synchronisation", "technologie", "telescope",
        "thermometre", "tourbillon", "transparence", "tremblement", "trompette", "typographie", "vertigineux", "volcanologue", "xylophone", "zeppelin"
    ];  // Tableau de mots .
    
    println!("😃 Salut je m'appelle Jason, j'ai crée le jeu : Devine le mots en rust😃");
    println!("Mon objectif est d'apprendre le rust mais aussi vous amuser avec mon jeu .");
    println!(" Règle du jeu :\n-Pour commencer il vous sera donné le nombre de lettre que contient le mots .\n-Vous choisissez une lettre et si la lettre est comprise dans le mots, on vous le diras.\n-Vous saurez le nombre de fois que la lettre est apparait dans le mots ainsi que son ou ses emplacements dans le mots .");

    println!("A présent, commençons :");

    while pv !=0 {

        let words_index : usize = thread_rng().gen_range(0..words.len());
        let words_choosed: &str = words[words_index];
        let mut tab_letter_used: Vec<char> = vec![];

        println!("Le mots contient {} lèttres.",words_choosed.chars().count());

        let mut ch_letter : Vec<char>=vec!['_';words_choosed.chars().count()];

        let mut word_form: String = ch_letter.iter().collect();

        while (word_form != words_choosed) && (pv!=0) {


            let Some(lettre) = lire_proposition() else {return ;};
                
            let test :Vec<u8> = find_letter(lettre ,words_choosed);

            if words_choosed.contains(lettre) {
                if !tab_letter_used.contains(&lettre) {
                    tab_letter_used.push(lettre);
                } else {
                    println!("Cette lettre a déja été cité .");
                    continue;
                }

                for i in test {
                    ch_letter[ i as usize] = lettre; 
                }
                // Into_iter transforme le tableau en element iterable tandisque .iter ce contente d iterer sur le tableau.
                word_form = ch_letter.iter().collect();

                println!("La lettre '{}' appartient au mots mystere.",lettre);
                println!("Proposition = {}",word_form);
                println!(" lettres deja utilisé : {:?}",tab_letter_used);

            }
            
            else {
                if !tab_letter_used.contains(&lettre) {
                    tab_letter_used.push(lettre);
                    pv-=1;
                    
                } else {
                    println!("Cette lettre a déja été cité .");
                    continue;
                }
                
                println!(" La lettre {} n'appartient pas au mots mystere.",lettre);
                println!(" lettres deja utilisé : {:?}",tab_letter_used);
                println!("Proposition = {}",word_form);

                
                if pv ==0 {
                    println!(" vous avez pèrdu toutes vos points de vie . Désolé");
                    println!(" le mot était :{} .",words_choosed);
                }
                
            }

            if pv != 0 && word_form != words_choosed{
                println!("PV :{}","❤️".repeat(pv as usize));
            }

        }
        if pv != 0{
            println!("🏆 Bravo , vous avez deviné le mots mystère 🏆.");
            println!(" le mot était belle et bien :{} .",words_choosed);
        }

        println!("\rVoulez-vous continué à jouer? y/n");
        
        if lire_proposition().unwrap_or(' ') == 'y' {
            pv = 10;
        } else {
            break;
        }
    }
}


