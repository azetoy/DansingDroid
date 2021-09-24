use anyhow::anyhow;

#[derive(Debug, Clone)]
struct Map {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
enum Orientation {
    N,
    S,
    E,
    W,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    L,
    R,
    F,
}
//structure de robot
#[derive(Debug, Clone)]
struct Robot {
    id: i32,
    x: i32,
    y: i32,
    orientation: Orientation,
    direction: Vec<Direction>,
}
// struct MultiRobots qui permet de gere un vecteur de robot
// simplifie la gestion de robot
#[derive(Debug, Clone)]
struct MultiRobots {
    listes: Vec<Robot>,
}

impl Orientation {
    fn tourne_a_gauche(&self) -> Orientation {
        match self {
            Orientation::N => Orientation::W,
            Orientation::W => Orientation::S,
            Orientation::S => Orientation::E,
            Orientation::E => Orientation::N,
        }
    }

    fn tourne_a_droite(&self) -> Orientation {
        match self {
            Orientation::N => Orientation::E,
            Orientation::W => Orientation::N,
            Orientation::S => Orientation::W,
            Orientation::E => Orientation::S,
        }
    }
}

fn tu_va_bouger_tes_octes_oui(bibop_total: &mut MultiRobots, taille: &Map) {
    let bibop_total_n = bibop_total.clone();
    for mut indice in &mut bibop_total.listes {
        for b in indice.direction.iter() {
            match b {
                Direction::L => indice.orientation = indice.orientation.tourne_a_gauche(),
                Direction::R => indice.orientation = indice.orientation.tourne_a_droite(),
                //test de collision merci enormement a Hippolyte M pour son aide sur l'algo de collision
                //fais en collaboration avec Hippolyte merci beaucoups a lui 🐸
                Direction::F => match indice.orientation {
                    Orientation::N => {
                        for a in bibop_total_n.listes.iter() {
                            if indice.x == a.x && indice.y - 1 == a.y {
                                println!(
                                    "collision ID {:?} , en x = {:?} , y = {:?} ",
                                    indice.id, indice.x, indice.y
                                );
                                indice.y = indice.y;
                                break;
                            } else if indice.y - 1 <= 0 {
                                println!(
                                    "collision avec la map ID {:?} , en x = {:?} , y = {:?} ",
                                    indice.id, indice.x, indice.y
                                );
                                indice.y = indice.y;
                                break;
                            } else {
                                indice.y -= 1;
                                break;
                            }
                        }
                    }
                    Orientation::S => {
                        for a in bibop_total_n.listes.iter() {
                            if indice.x == a.x && indice.y + 1 == a.y {
                                println!(
                                    "collision 1ID {:?} , en x = {:?} , y = {:?} ",
                                    indice.id, indice.x, indice.y
                                );
                                indice.y = indice.y;
                                break;
                            } else if indice.y + 1 >= taille.y {
                                println!(
                                    "collision avec la map ID {:?} , en x = {:?} , y = {:?} ",
                                    indice.id, indice.x, indice.y
                                );
                                indice.y = indice.y;
                                break;
                            } else {
                                indice.y += 1;
                                break;
                            }
                        }
                    }
                    Orientation::W => {
                        for a in bibop_total_n.listes.iter() {
                            if indice.y == a.y && indice.x - 1 == a.x {
                                println!(
                                    "collision ID {:?} , en x = {:?} , y = {:?} ",
                                    indice.id, indice.x, indice.y
                                );
                                indice.x = indice.x;
                                break;
                            } else if indice.x - 1 <= 0 {
                                println!(
                                    "collision avec la map ID {:?} , en x = {:?} , y = {:?} ",
                                    indice.id, indice.x, indice.y
                                );
                                indice.x = indice.x;
                                break;
                            } else {
                                indice.x -= 1;
                                break;
                            }
                        }
                    }
                    Orientation::E => {
                        for a in bibop_total_n.listes.iter() {
                            if indice.y == a.y && indice.x + 1 == a.x {
                                println!(
                                    "collision ID {:?} , en x = {:?} , y = {:?} ",
                                    indice.id, indice.x, indice.y
                                );
                                indice.x = indice.x;
                                break;
                            } else if indice.x + 1 >= taille.x {
                                println!(
                                    "collision avec la map ID {:?} , en x = {:?} , y = {:?} ",
                                    indice.id, indice.x, indice.y
                                );
                                indice.x = indice.x;
                                break;
                            } else {
                                indice.x += 1;
                                break;
                            }
                        }
                    }
                },
            }
        }
    }
}
//test direction
impl Orientation {
    fn try_from(input: char) -> Orientation {
        match input {
            'N' | 'n' => Orientation::N,
            'S' | 's' => Orientation::S,
            'E' | 'e' => Orientation::E,
            'W' | 'w' => Orientation::W,
            _ => Orientation::N,
        }
    }
}
// test orientation
impl Direction {
    fn try_from(input: char) -> Direction {
        match input {
            'L' | 'l' => Direction::L,
            'F' | 'f' => Direction::F,
            'R' | 'r' => Direction::R,
            _ => Direction::L,
        }
    }
}
// permet la conversion d'un i32 en char car les fonction de base de rust ont pas vraiment bien marcher
fn conversion(input: i32) -> char {
    match input {
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        0 => '0',
        _ => '5',
    }
}
// recupere les info des roots dans le fichier
fn record(buf: String, robots: &mut MultiRobots) -> anyhow::Result<()> {
    let mut lines = buf.lines();
    let mut ligne = lines.next();

    let mut id = 0;
    // permet de s'arrater et pas faire de boucle infine
    let mut accumulateur = Some(2);
    while let Some(indice) = accumulateur {
        if indice < 5 {
            ligne = lines.next();
            ligne = lines.next();
            // gestion des espace et des commentaire
            match ligne {
                Some(x) => match x {
                    "" => ligne = lines.next(),
                    "//" => ligne = lines.next(),
                    "\\" => ligne = lines.next(),
                    _ => {}
                },
                None => {}
            }

            let mut delet_whitespace = ligne
                .ok_or(anyhow!("ERROR CANT CATCH THE LOLI CALL THE FBI"))?
                .split_whitespace();

            let x = delet_whitespace
                .next()
                .ok_or(anyhow!("Err cant read x"))?
                .parse::<i32>()?;

            let y = delet_whitespace
                .next()
                .ok_or(anyhow!("Err cant read y"))?
                .parse::<i32>()?;

            let orientation = delet_whitespace
                .next()
                .ok_or(anyhow!("Err cant read ori"))?
                .parse::<char>()?;

            let mut vec = Vec::new();

            let instruction = lines
                .next()
                .ok_or(anyhow!("Even the US have instru \u{F923}"))?
                .chars();

            for carac in instruction {
                vec.push(Direction::try_from(carac));
            }
            //on initialise robot avec les information du fichier
            let bibop = Robot {
                id: id,
                x: x,
                y: y,
                orientation: Orientation::try_from(orientation),
                direction: vec,
            };
            id += 1;

            println!("bibop = {:?} ", bibop);
            robots.listes.push(bibop);

            accumulateur = Some(indice + 2);
        } else {
            break;
        }
    }
    anyhow::Result::Ok(())
}

fn affichage_graphique(map: &Map, robots: &MultiRobots) {
    // ligne et colonne prennet la taille de la map
    let nb_ligne = map.x;
    let nb_colonne = map.y;
    //on fais un clone du nb de linge pour eviter un borrowing et on l'affecte a la var temp
    let temp = nb_ligne.clone();
    //on dit que iter est une var de type string
    let mut iter: String = "".to_string();
    //les espace du debut
    iter.push(' ');
    iter.push(' ');
    iter.push(' ');
    //on recup chaque valeur stocker dans temps en partant de 0 c'est un parcours et on le push en tant que chars dans iter grace a la fonction conversion
    for i in 0..(temp + 1) {
        iter.push(conversion(i));
        iter.push(' ');
        iter.push(' ');
    }
    // creation de la grille et affichage des position des robot
    for ligne in 0..(nb_ligne + 1) {
        let mut ligno: String = " . ".to_string();
        for colonne in 0..(nb_colonne) {
            ligno.push_str(" . ");

            for bibop in &robots.listes {
                if bibop.x == ligne && bibop.y == colonne {
                    ligno.pop();
                    ligno.pop();
                    ligno.pop();
                    match &bibop.orientation {
                        Orientation::N => ligno.push_str("👆 "),
                        Orientation::S => ligno.push_str("💨 "),
                        Orientation::W => ligno.push_str("🚅 "),
                        Orientation::E => ligno.push_str("👇 "),
                        _ => ligno.push_str(". "),
                    };
                }
            }
        }

        println!("{:?} {:?} ", ligne, ligno);
    }
    //uniquement pour le plaisire viseul
    iter.pop();
    println!("{:?}", iter);
}

fn main() -> anyhow::Result<()> {
    // affectatio de la struct MultiRobots a la var multithreading_robots
    let mut multithreading_robots = MultiRobots { listes: Vec::new() };
    // recuperation du buffer(file)
    let buffer = std::fs::read_to_string("instruction.txt")?;

    // parcours de ligne pour recup la taille du monde
    let mut lines = buffer.lines();
    let mut world = lines
        .next()
        .ok_or(anyhow!("wtf GIVE ME A WORLD BUDDIES"))?
        .split_whitespace();

    let taille_map_x = world
        .next()
        .ok_or(anyhow!("Meh na map_x_max"))?
        .parse::<i32>()?;

    let taille_map_y = world
        .next()
        .ok_or(anyhow!("Meh na map_y_max"))?
        .parse::<i32>()?;

    let map = Map {
        x: taille_map_x,
        y: taille_map_y,
    };
    //appel de la fonction qui va recup les cordonner orientation et direction des robots prend en parametre le buffer et la struct de robot
    record(buffer, &mut multithreading_robots)?;
    println!(
        "
Etat initial
============================
"
    );
    // affiche l'etat initail avant le deplacement
    affichage_graphique(&map, &multithreading_robots);
    //appel de la fcontion qui effectue le depalcement et le test de collision
    tu_va_bouger_tes_octes_oui(&mut multithreading_robots, &map);
    println!(
        "
Etat final
============================
"
    );
    affichage_graphique(&map, &multithreading_robots);

    println!("{:?}", multithreading_robots);

    anyhow::Result::Ok(()) // like EXIT_SUCCESS somehow.
}

#[cfg(test)]
mod test {

    #[derive(Debug, Clone)]
    enum Orientation {
        N,
        S,
        E,
        W,
    }
    #[derive(Debug, Clone, Copy)]
    enum Direction {
        L,
        R,
        F,
    }
    impl Orientation {
        fn try_from(input: char) -> Orientation {
            match input {
                'N' | 'n' => Orientation::N,
                'S' | 's' => Orientation::S,
                'E' | 'e' => Orientation::E,
                'W' | 'w' => Orientation::W,
                _ => Orientation::N,
            }
        }
    }
    // test orientation
    impl Direction {
        fn try_from(input: char) -> Direction {
            match input {
                'L' | 'l' => Direction::L,
                'F' | 'f' => Direction::F,
                'R' | 'r' => Direction::R,
                _ => Direction::L,
            }
        }
    }
    #[test]
    // test pour la lecture du fichier
    fn file_test() {
        assert!(std::fs::read_to_string("instruction.txt").is_ok());
        assert!(std::fs::read_to_string("instruction_ababa.tru").is_err());
        // assert!(File::open("cant open file").is_err());
    }

    #[test]
    fn test_orientation() {
        assert!(matches!(Orientation::try_from('N'), Orientation::N));
        assert!(matches!(Orientation::try_from('s'), Orientation::S));
        assert!(matches!(Orientation::try_from('E'), Orientation::E));
        assert!(matches!(Orientation::try_from('w'), Orientation::W));
        assert!(matches!(Orientation::try_from('g'), Orientation::N));
    }

    #[test]
    fn test_direction() {
        assert!(matches!(Direction::try_from('l'), Direction::L));
        assert!(matches!(Direction::try_from('f'), Direction::F));
        assert!(matches!(Direction::try_from('R'), Direction::R));
    }
}
