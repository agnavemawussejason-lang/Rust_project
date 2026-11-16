use rand::{thread_rng,Rng}; // Fonction de la dependence Rand
use std::io;

fn find_letter(letter :char,words:&str)-> (u8,Vec<u8>){

    let mut letter_count :u8 = 0;
    let mut index_tab: Vec<u8> =[].to_vec();
    let mut index_value :u8 = 0;

    for i in words.chars() {
        if i == letter { 
            letter_count+= 1; 
            index_tab.push(index_value);
        }
        index_value += 1
    }

    if letter_count != 0 { 
        (letter_count,index_tab)// Retourne un tuple de (bool,u8,un Vec)
    }
    else { 
        (letter_count,[].to_vec())// Retourne un tuple de valeur nulle .
    }
}

fn lire_proposition () -> String {
    let mut value :String =String::new();
     io::stdin()
        .read_line( &mut value)
        .expect("⚠️ vous devez impérativement entrer un caractère.");

    value
}


fn main() {
    let mut pv :u8 = 10;
    let words: [&str; 100] = [
        "arbre", "avion", "balai", "banane", "bateau", "bijou", "blanc", "boite", "bouche", "bouteille",
        "brique", "bureau", "cahier", "camion", "canape", "carotte", "casque", "chaise", "champ", "chapeau",
        "chaton", "chemin", "chemise", "cheval", "chien", "citron", "clavier", "cloche", "cochon", "coffre",
        "colline", "compte", "crayon", "cuisine", "danse", "dauphin", "dentiste", "dessin", "doigt", "drapeau",
        "ecole", "ecran", "enfant", "epaule", "etoile", "fauteuil", "fenetre", "feuille", "fleur", "fleuve",
        "foret", "fourmi", "fraise", "fromage", "fruit", "garage", "garcon", "gateau", "girafe", "glace",
        "gomme", "graine", "guitare", "herbe", "heure", "hiver", "horloge", "image", "insecte", "jardin",
        "jaune", "joueur", "journal", "jupe", "lampe", "lapin", "lettre", "livre", "lumiere", "lunettes",
        "maison", "manteau", "marche", "marteau", "matin", "medecin", "melon", "miroir", "montagne", "montre",
        "moteur", "moulin", "mouton", "musique", "nuage", "oiseau", "ombre", "orange", "ordinateur", "oreille"
    ];  // Tableau de mots .
    
    println!("😃 Salut je m'appelle Jason, j'ai crée le jeu : Devine le mots en rust😃");
    println!("Mon objectif est d'apprendre le rust mais aussi vous amuser avec mon jeu .");
    println!(" Règle du jeu :\n-Pour commencer il vous sera donné le nombre de lettre que contient le mots .\n-Vous choisissez une lettre et si la lettre est comprise dans le mots, on vous le diras.\n-Vous saurez le nombre de fois que la lettre est apparait dans le mots ainsi que son ou ses emplacements dans le mots .");

    println!("A présent, commençons :");

    while pv !=0 {

        let words_index : usize = thread_rng().gen_range(0..words.len());
        let words_choosed: &str = words[words_index];
        let mut tab_letter_used: Vec<char> = [].to_vec();

        println!("Le mots contient {} lettres.",words_choosed.chars().count());
        println!("Entrez une lettre :");

        let mut ch_letter : Vec<char>=[].to_vec();

        for _i in 0..words_choosed.chars().count(){
            ch_letter.push('_');
        }

        let mut word_form: String = String::from("");

        while (word_form != words_choosed) && (pv!=0) {

            let mut proposition :String ;
            proposition =lire_proposition();

            while proposition.trim().chars().next().is_none() {
                println!("⚠️ ATTENTION !! les caractères nuls ne sont pas autorisés .");
                proposition.clear();
                proposition =lire_proposition()
            }

            let lettre :char = proposition.trim().chars().next().unwrap().to_ascii_lowercase();
                
            let test :(u8,Vec<u8>) = find_letter(lettre,words_choosed);

            if words_choosed.contains(lettre) {
                if !tab_letter_used.contains(&lettre) {
                    tab_letter_used.push(lettre);
                } else {
                    println!("Cette lettre a déja été cité .");
                    continue;
                }

                for i in test.1 {
                    ch_letter[ i as usize] = lettre; 
                }
                // Into_iter transforme le tableau en element iterable tandisque .iter ce contente d iterer sur le tableau.
                word_form = ch_letter.iter().collect();

                println!("La lettre '{}' appartient au mots mystere.",lettre);
                println!("Il apparait {} fois .",test.0);
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
                    println!(" vous avez perdu toutes vos points de vie . Désolé");
                    println!(" le mot était :{} .",words_choosed);
                }
                
            }

            if pv != 0 && word_form != words_choosed{
                println!("PV :{}","❤️".repeat(pv as usize));
                println!("Entrez une nouvelle lettre :");
            }

        }
        if pv != 0{
            println!("🏆 Bravo , vous avez deviné le mots mystère 🏆.");
            println!(" le mot était belle et bien :{} .",words_choosed);
        }

        // Permettre au joueur de recommencer une nouvelle partie .
        println!(" Voulez-vous continué à jouer? y/n");
        let choix :String ;
        choix = lire_proposition();
        
        if choix.trim() == "y" {
            pv = 10;
        } else {
            break;
        }
    }
}


