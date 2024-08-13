
# pwd-manager : Gestionnaire de Mots de Passe en Rust

Ce projet est un gestionnaire de mots de passe écrit en Rust, permettant de stocker, récupérer et lister des mots de passe de manière sécurisée. Les mots de passe sont chiffrés en utilisant AES-256-GCM.

## Installation

1. **Cloner le dépôt :**
   ```sh
   git clone https://github.com/wlemalin/pwd-manager.git
   cd pwd-manager
   ```

2. **Configurer Cargo :**
   Assurez-vous d'avoir Rust et Cargo installés. Suivez les instructions sur [rust-lang.org](https://www.rust-lang.org/learn/get-started).

3. **Config shortcut**


## Utilisation

### Commandes disponibles

- **Ajouter un mot de passe :**
  ```sh
  pwd add <nom>
  ```
  Exemple :
  ```sh
  pwd add gmail
  ```
  Le programme vous demandera d'entrer le mot de passe à ajouter.

- **Récupérer un mot de passe :**
  ```sh
  pwd get <nom>
  ```
  Exemple :
  ```sh
  pwd get gmail
  ```
  Le programme affichera le mot de passe associé au nom donné.

- **Lister tous les noms de mots de passe enregistrés :**
  ```sh
  pwd list
  ```
  Cette commande affichera tous les noms pour lesquels des mots de passe sont enregistrés.
