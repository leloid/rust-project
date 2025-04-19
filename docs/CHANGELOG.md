# Changelog
Tous les changements notables apportés à ce projet seront documentés dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.1.0/),
et ce projet adhère au [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2024-03-14

### Ajouté
- Système de simulation de robots en essaim
- Interface graphique avec Bevy
- Trois types de robots spécialisés :
  - Explorateurs pour la découverte de la carte
  - Collecteurs pour la récolte des ressources
  - Scientifiques pour l'analyse et l'optimisation
- Génération procédurale de la carte
- Système de brouillard de guerre
- Gestion des ressources :
  - Énergie
  - Minéraux
  - Science
- Station centrale pour la coordination des robots
- Contrôles interactifs :
  - Pause/Reprise
  - Ajustement de la vitesse
  - Contrôle de la caméra
- Configuration paramétrable via `config/mod.rs`
- Documentation complète du projet
- Tests unitaires

### Technique
- Architecture modulaire avec séparation des responsabilités :
  - Module `robot/` pour la logique des robots
  - Module `station/` pour la gestion de la station
  - Module `map/` pour la génération et gestion de la carte
  - Module `resources/` pour les ressources du jeu
  - Module `config/` pour la configuration
  - Module `bin/` pour les points d'entrée
- Implémentation en Rust pour des performances optimales
- Utilisation du moteur graphique Bevy 