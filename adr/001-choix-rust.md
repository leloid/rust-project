# ADR-001: Choix de Rust comme langage de programmation

## Statut
Accepté

## Contexte
Pour développer une simulation de robots en essaim, nous avions besoin d'un langage de programmation qui offre :
- De hautes performances pour gérer de nombreuses entités en temps réel
- Une gestion de la mémoire sûre et efficace
- Un système de typage fort pour éviter les bugs
- Une bonne capacité d'intégration avec des bibliothèques graphiques

## Décision
Nous avons choisi Rust pour les raisons suivantes :
- Sécurité mémoire garantie à la compilation
- Performances proches du C/C++
- Système de gestion des dépendances (Cargo) moderne et efficace
- Excellent support pour la programmation concurrente
- Écosystème riche en bibliothèques pour le développement de jeux et simulations

## Conséquences

### Positives
- Pas de bugs liés à la gestion de la mémoire
- Excellentes performances d'exécution
- Code plus fiable grâce au système de typage strict
- Facilité de gestion des dépendances avec Cargo
- Bonne intégration avec les outils de développement modernes

### Négatives
- Courbe d'apprentissage plus raide pour les développeurs
- Temps de compilation plus long
- Besoin de gérer explicitement les problèmes de propriété (ownership) et d'emprunt (borrowing) 