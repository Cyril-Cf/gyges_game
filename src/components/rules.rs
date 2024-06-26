use yew::{function_component, html, Html};

#[function_component(Rules)]
pub fn rules() -> Html {
    html! {
        <>
            <div>
                <h2>{"But du jeu"}</h2>
                <p>{"Le but de "}<a href="https://www.gigamic.com/jeux-a-deux/136-gyges.html" target="_blank">{"Gygès"}</a>{" est d'amener à votre tour un pion derrière la ligne la plus proche de l'adversaire, comme mettre un but ! Chacun des participants jouera à tour de rôle."}</p>
            </div>
            <div class="container">
                <div>
                    <h2>{"Les pions"}</h2>
                    <p>{"Il y a 3 types de pions : les simples, les doubles et les triples. Tous les pions ont la même couleur et peuvent donc éventuellement être joués par les deux joueurs au cours de la partie."}</p>
                </div>
                <div>
                    <h2>{"Déplacement"}</h2>
                    <p>{"A son tour, chaque joueur doit bouger un pion de son choix, mais uniquement un pion se trouvant sur la ligne la plus proche de lui. Les pions simples se déplacent d'une case, les doubles de deux cases et les triples de trois, toujours orthogonalement et jamais en diagonal. Si le mouvement s'arrête sur une case vide, le pion s'arrête simplement là. Lors de son déplacement, un pion ne peut pas chevaucher un autre, il ne peut se déplacer que sur des cases vides."}</p>
                </div>
                <div>
                    <h2>{"Rebond"}</h2>
                    <p>{"Si le pion finit son mouvement sur une case occupée, il peut alors rebondir  et recommencer un mouvement d'autant de cases que la valeur du pion sur lequel il est arrivé. Un même pion peut bien entendu rebondir plusieurs fois de suite."}</p>
                    <p>{"Si vous avez des doutes, n'hésitez pas à "}<a href="https://www.gigamic.com/index.php?controller=attachment&id_attachment=98" target="_blank">{"consulter les règles détaillées et quelques exemples !"}</a></p>
                </div>
                <div>
                    <h2>{"Mode expert"}</h2>
                    <p>{"Il existe également un mode expert qui complémente les déplacements. A la place d'un rebond, il est possible de remplacer le pion d'arrivé par le pion qui s'est déplacé. Vous devrez alors repositionner le pion d'arrivée sur une case vide de votre choix n'importe où sur le plateau (mais pas plus loin que la première ligne de votre adversaire). Ce mode n'est pas actuellement mis en place, n'hésitez pas à revenir vers moi si cela vous intéresse !"}</p>
                </div>
            </div>
        </>
    }
}
