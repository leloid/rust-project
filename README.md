# ProjetEssaim - Simulation de Robots en Essaim

![Banner](https://raw.githubusercontent.com/username/ProjetEssaim/main/docs/banner.png)

## Démonstration

[Vidéo de démonstration](docs/video.mov)

*Note: Une copie de la vidéo de démonstration est disponible dans le dossier `docs/`.*

## Description

ProjetEssaim est une simulation de robots en essaim développée en Rust. Le projet modélise un système où différents types de robots (explorateurs, collecteurs, scientifiques) travaillent ensemble pour explorer une carte, collecter des ressources et les ramener à une station centrale.

La simulation utilise le moteur graphique Bevy pour offrir une visualisation interactive et en temps réel du comportement des robots.

## Fonctionnalités

- 🤖 **Robots spécialisés** : Explorateurs, Collecteurs et Scientifiques, chacun avec des comportements et des objectifs différents
- 🗺️ **Génération procédurale** de cartes avec différents types de ressources et d'obstacles
- 🔍 **Exploration collaborative** avec partage d'informations entre robots
- 📊 **Interface graphique** avec contrôles interactifs et visualisations claires
- ⚙️ **Simulation paramétrable** avec brouillard de guerre, vitesse ajustable et contrôles de pause/reprise

## Contrôles de la simulation

- **Pause/Reprise** : Bouton dans le coin inférieur droit
- **Vitesse** : Boutons + et - dans le coin inférieur gauche, ou touches du clavier + et -
- **Caméra** : Clic gauche + déplacement pour se déplacer, molette de la souris pour zoomer/dézoomer

## Installation

### Prérequis

- Rust et Cargo (version récente)
- Dépendances de Bevy ([voir la documentation officielle](https://bevyengine.org/learn/book/getting-started/setup/))

### Compilation et exécution

1. Cloner le dépôt
   ```bash
   git clone https://github.com/username/ProjetEssaim.git
   cd ProjetEssaim
   ```

2. Lancer la simulation graphique
   ```bash
   cargo run --bin gui
   ```

3. Ou lancer la version console
   ```bash
   cargo run --bin main
   ```

## Configuration

Vous pouvez modifier les paramètres de la simulation dans `src/config/mod.rs`:

- Taille de la carte
- Graine aléatoire
- Activation/désactivation du brouillard de guerre

## Architecture

- **robot/** : Définition des robots et de leurs comportements
- **station/** : Logique de la station centrale
- **map/** : Génération et gestion de la carte
- **resources/** : Ressources pour l'interface graphique et la simulation
- **bin/** : Points d'entrée de l'application
- **config/** : Configuration globale

## Licence

Ce projet est sous licence [MIT](LICENSE).



TODO :  

Rajoute la creation de robot 
    - Si 5 energy crée un robot explorer 
    - Si 5 mineral créer un robot collectot 
    - Si 5 Scientique créer un robot scientist 

Rajouter les collisions 

Ameliorer le graphisme

Faires les fichiers tests 

Ameliorer la structure du projet, faire des sous fichier decomposer le code 

Retierer les logos/emoticones 