use rand::{thread_rng,Rng}; // Fonction de la dependence Rand
use std::{io, vec};

// fonction permettant de determiner si une lettre appartient oui ou non a un mots.
fn afficher_stat (pv_char:&str){
    println!("PV :{}",pv_char)
}

fn add_in_tab(lettre:char,tab:&Vec<char>)-> bool {
    let mut is_in_tab :bool=false;
    for _i in tab.iter(){
        if *_i == lettre{
            is_in_tab = true;
            break;
        }
    }
    return is_in_tab;
}


fn find_letter(letter :char,words:&str)-> (bool ,u8,Vec<u8>){

    let mut letter_count :u8 = 0;
    let mut index_tab: Vec<u8> = vec![];
    let mut index_value :u8 = 0;

    for i in words.chars() {
        if i == letter { 
            letter_count+= 1; 
            index_tab.push(index_value);
        }
        index_value += 1
    }

    if letter_count != 0 { return (true,letter_count,index_tab);// Retourne un tuple de (bool,u8,un Vec)
        }
    else { return (false,letter_count,[].to_vec());// Retourne un tuple de valeur nulle .
        }
}

fn main() {
    let mut pv :u8 = 10;
    let mut pv_char : &str = "❤️❤️❤️❤️❤️";
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

        let words_index : usize = thread_rng().gen_range(0..100);
        let words_choosed: &str = words[words_index];
        let mut tab_letter_used: Vec<char> = vec![];

        println!("Le mots contient {} lettres.",words_choosed.chars().count());
        println!("Entrez une lettre :");

        let mut ch_letter : Vec<char>=vec![];
        let mut word_form: String = ch_letter.clone().into_iter().collect();

        for _i in 0..words_choosed.chars().count(){
            ch_letter.push('_');
        }

        while (word_form != words_choosed) && (pv!=0) {

            let mut proposition :String = String::new();

            io::stdin()
                .read_line(&mut proposition)
                .expect("⚠️ vous devez impérativement entrer un caractère.");

            while proposition.trim().chars().next() == None {
                println!("⚠️ ATTENTION !! les caractères nuls ne sont pas autorisés .");
                io::stdin()
                    .read_line(&mut proposition)
                    .expect("⚠️ vous devez impérativement entrer un caractère.");
            }

            let lettre :char = proposition.trim().chars().next().unwrap();
                
            let test :(bool ,u8,Vec<u8>) = find_letter(lettre,words_choosed);

            if test.0 {
                if !add_in_tab(lettre, &tab_letter_used) {
                    tab_letter_used.push(lettre);
                } else {
                    println!("Cette lettre a déja été cité .");
                }

                for _i in test.2 {
                    let lettre :char = proposition.trim().chars().next().unwrap();
                    ch_letter[_i as usize] = lettre; 
                }
                word_form = ch_letter.clone().into_iter().collect();

                println!("La lettre '{}' appartient au mots mystere.",lettre);
                println!("Il apparait {} fois .",test.1);
                println!("Proposition = {}",word_form);
                println!(" lettres deja utilisé : {:?}",tab_letter_used);
                afficher_stat(pv_char);
                println!("Entrez une nouvelle lettre :");
            }

            else {
                if !add_in_tab(lettre, &tab_letter_used) {
                    tab_letter_used.push(lettre);
                    pv-=1;
                    pv_char = match pv_char.char_indices().next_back() {
                        Some((index,_)) => &pv_char[..index],
                        None =>"",
                    };
                } else {
                    println!("Cette lettre a déja été cité .");
                }

                println!(" La lettre {} n'appartient pas au mots mystere.",lettre);
                println!(" lettres deja utilisé : {:?}",tab_letter_used);
                afficher_stat(pv_char);
                
                if pv ==0 {
                    println!(" vous avez perdu toutes vos points de vie . Désolé");
                    println!(" le mot était :{} .",words_choosed);
                }
                else{
                    println!("Entrez une nouvelle lettre :");
                }
            }
        }
        if pv != 0{
            println!("🏆 Bravo , vous avez deviné le mots mystère 🏆.");
            println!(" le mot était belle et bien :{} .",words_choosed);
        }

        // Permettre au joueur de recommencer une nouvelle partie .
        println!(" Voulez-vous continué à jouer? y/n");
        let mut choix :String = String::new();
        io::stdin()
            .read_line(&mut choix)
            .expect(" vous devez choisir entre y et n .");
        
        if choix.trim() == "y" {
            pv = 10;
            pv_char = "❤️❤️❤️❤️❤️";
        } else {
            break;
        }
    }
}


