# ADR-002: Utilisation de Bevy comme moteur graphique

## Statut
Accepté

## Contexte
La simulation de robots en essaim nécessite une interface graphique pour :
- Visualiser les robots et leur environnement
- Afficher les ressources et leur état
- Permettre l'interaction avec la simulation
- Gérer le rendu en temps réel

## Décision
Nous avons choisi Bevy comme moteur graphique pour les raisons suivantes :
- Natif Rust avec une excellente intégration
- Architecture ECS (Entity Component System) moderne
- Performance et optimisation pour le rendu 2D/3D
- Support actif de la communauté
- Documentation complète et exemples nombreux
- Open source et sans frais de licence

## Conséquences

### Positives
- Développement plus rapide grâce à l'ECS
- Bonnes performances de rendu
- Facilité d'extension et de modification
- Intégration naturelle avec Rust
- Hot-reloading pour le développement

### Négatives
- Moins mature que certains moteurs alternatifs
- Documentation parfois incomplète pour les cas d'usage avancés
- Nécessité d'apprendre les concepts ECS
- Taille du binaire final plus importante 