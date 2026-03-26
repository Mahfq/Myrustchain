# 🦀 Myrustchain

## Qu'est-ce qu'une Blockchain et le consensus PBFT ?

### La Blockchain
Une **Blockchain** est un registre distribué et immuable composé d'une chaîne de blocs. Chaque bloc contient un ensemble de transactions, un horodatage, son propre hash et le hash du bloc précédent, créant ainsi un lien sécurisé entre eux. Dans ce projet, la blockchain gère également un état des comptes (`accounts`) pour suivre les soldes des utilisateurs.

### L'algorithme PBFT (Practical Byzantine Fault Tolerance)
Le **PBFT** est un algorithme de consensus conçu pour permettre à un système distribué de parvenir à un accord même si certains nœuds sont défaillants ou malveillants (nœuds byzantins). Son fonctionnement repose sur trois phases principales pour valider un bloc :

1.  **Pre-Prepare** : Un nœud primaire propose un nouveau bloc aux autres nœuds du réseau.
2.  **Prepare** : Les nœuds vérifient la validité du bloc (par exemple, absence de double dépense et validité du hash précédent). S'il est valide, ils diffusent un message de préparation pour indiquer leur accord aux autres.
3.  **Commit** : Lorsqu'un nœud reçoit suffisamment de messages "Prepare" pour atteindre un quorum, il diffuse un message de validation finale ("Commit"). Une fois le quorum de messages "Commit" atteint, le bloc est définitivement ajouté à la blockchain locale du nœud.

## Fonctionnalités

* **Consensus PBFT complet** : Gestion des phases `Pre-Prepare`, `Prepare` et `Commit`.
* **Protection Anti-Double Dépense** : Validation stricte des soldes dans chaque bloc avant acceptation.
* **Architecture Asynchrone** : Utilisation de `Tokio` pour la gestion des communications réseau et des tâches concurrentes.
* **Sécurisation par Hashage** : Intégrité garantie par le calcul de hashs SHA-256 pour chaque bloc.

## Structure du Projet

```text
src/
├── consensus/
│   ├── pbft.rs       # Logique du nœud et machine à états du consensus
│   ├── message.rs    # Définition des types de messages (Prepare, Commit...)
│   └── mod.rs
├── core/
│   ├── block.rs       # Structure des blocs et calcul de hash
│   ├── transaction.rs # Logique des transactions
│   ├── chain.rs       # Gestion de la blockchain et des comptes
│   └── mod.rs
├── engine.rs         # Orchestrateur du nœud
└── main.rs           # Point d'entrée de l'application
tests/
└── blockchain_tests.rs # Tests d'intégration et unitaires

## Installation & Utilisation

### Prérequis
* [Rust & Cargo](https://rustup.rs/) (dernière version stable)

### Installation
1.  **Clonez le dépôt :**
    ```bash
    git clone [https://github.com/votre-username/myrustchain.git](https://github.com/votre-username/myrustchain.git)
    cd myrustchain
    ```
2.  **Compilez le projet :**
    ```bash
    cargo build
    ```

### Exécution des tests ✅
Pour vérifier le bon fonctionnement du consensus et de la validation :
```bash
cargo test

## Scénarios de Test Couverts

Le projet inclut une suite de tests automatisés garantissant :

*   **L'initialisation correcte du bloc Genesis** : Vérifie que la chaîne commence toujours par un état valide et stable.
*   **Le rejet des blocs corrompus** : Validation stricte empêchant l'ajout de blocs avec un mauvais hash précédent ou un index incorrect.
*   **La détection de la double dépense** : Empêche un utilisateur d'envoyer plus de fonds qu'il n'en possède au sein d'un même bloc.
*   **La validation du quorum** : Un bloc n'est validé et ajouté que si le nombre requis de votes de nœuds uniques est atteint (logique PBFT).