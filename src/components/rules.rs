use yew::{function_component, html, Html};

#[function_component(Rules)]
pub fn rules() -> Html {
    html! {
        <section id="rules">
            <h2>{"But du jeu"}</h2>
            <p>{"Le but de "}<a href="https://www.gigamic.com/jeux-a-deux/136-gyges.html" target="_blank">{"Gygès"}</a>{" est d'amener à votre tour un pion derrière la ligne la plus proche de l'adversaire, comme mettre un but !"}</p>
            <h2>{"Les pions"}</h2>
            <p>{"Il y a 3 types de pions : les simples, les doubles et les triples. Tous les pions ont la même couleur et peuvent donc éventuellement être joués par les deux joueurs, mais on ne pourra jouer qu'un pion se trouvant sur la ligne la plus proche de soi."}</p>
            <h2>{"Déplacement"}</h2>
            <p>{"Les pions simples se déplacent d'une case, les doubles de deux cases et les triples de trois, toujours orthogonalement et jamais en diagonal. Si le mouvement s'arrête sur une case vide, le pion s'arrête simplement là. Lors de son déplacement, un pion ne peut pas chevaucher un autre, il ne peut se déplacer que sur des cases vides."}</p>
            <h2>{"Rebond ou remplacement"}</h2>
            <p>{"Si le pion arrive sur une case occupée à la fin de son mouvement, il peut alors rebondir d'autant de cases que la valeur du pion sur lequel il est arrivé. Un même pion peut bien entendu rebondir plusieurs fois de suite. Il est aussi possible de remplacer le pion d'arrivé par le pion qui s'est déplacé. Vous devrez alors le repositionner sur une case vide de votre choix n'importe ou sur le plateau."}</p>
            <p>{"Si vous avez des doutes, n'hésitez pas à "}<a href="https://www.gigamic.com/index.php?controller=attachment&id_attachment=98" target="_blank">{"consulter les règles détaillées et quelques exemples !"}</a></p>
        </section>
    }
}
