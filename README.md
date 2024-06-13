
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

3. **Configurer les raccourcis :**
   Les raccourcis sont disponibles dans le dépôt. Suivez ces étapes pour les configurer :

   - **Créer un répertoire `bin` dans votre répertoire personnel :**
     ```sh
     mkdir -p ~/bin
     ```

   - **Copier les scripts de raccourcis dans le répertoire `bin` :**
     ```sh
     cp pwd-add pwd-get pwd-list ~/bin/
     ```

   - **Rendre les scripts exécutables :**
     ```sh
     chmod +x ~/bin/pwd-add
     chmod +x ~/bin/pwd-get
     chmod +x ~/bin/pwd-list
     ```

   - **Ajouter `~/bin` à votre variable d'environnement PATH :**
     Ouvrez le fichier `.bashrc` dans votre répertoire personnel avec un éditeur de texte, par exemple :
     ```sh
     nano ~/.bashrc
     ```
     Ajoutez la ligne suivante à la fin du fichier :
     ```sh
     export PATH="$HOME/bin:$PATH"
     ```
     Sauvegardez et fermez le fichier. Puis, rechargez le fichier `.bashrc` pour appliquer les modifications :
     ```sh
     source ~/.bashrc
     ```

## Utilisation

### Commandes disponibles

- **Ajouter un mot de passe :**
  ```sh
  pwd-add <nom>
  ```
  Exemple :
  ```sh
  pwd-add gmail
  ```
  Le programme vous demandera d'entrer le mot de passe à ajouter.

- **Récupérer un mot de passe :**
  ```sh
  pwd-get <nom>
  ```
  Exemple :
  ```sh
  pwd-get gmail
  ```
  Le programme affichera le mot de passe associé au nom donné.

- **Lister tous les noms de mots de passe enregistrés :**
  ```sh
  pwd-list
  ```
  Cette commande affichera tous les noms pour lesquels des mots de passe sont enregistrés.
