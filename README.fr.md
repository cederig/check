# check

Un outil en ligne de commande simple écrit en Rust pour obtenir des informations sur un fichier ou sur tous les fichiers d'un répertoire.

## Fonctionnalités

- Obtenir la taille d'un fichier.
- Identifier le type MIME du fichier.
- Détecter l'encodage des caractères d'un fichier.
- Calculer les sommes de contrôle SHA256 et MD5.
- Traiter un seul fichier ou tous les fichiers d'un répertoire.

## Dépendances

Le projet utilise les principales caisses (crates) Rust suivantes :

- `clap` pour l'analyse des arguments de la ligne de commande.
- `anyhow` et `thiserror` pour la gestion des erreurs.
- `sha2` et `md5` pour le calcul des sommes de contrôle.
- `infer` pour la détection des types de fichiers.
- `chardet` pour la détection de l'encodage des caractères.

## Prérequis

Pour compiler ce projet, vous devez avoir la chaîne d'outils Rust (y compris `rustc` et `cargo`) installée sur votre système.

Vous pouvez l'installer depuis [rustup.rs](https://rustup.rs/).

## Installation et Compilation

Le processus de compilation est le même pour Linux, macOS et Windows.

1.  **Clonez le dépôt :**
    ```sh
    git clone <repository_url>
    cd check
    ```

2.  **Compilez le projet en mode `release` :**
    ```sh
    cargo build --release
    ```

3.  **Trouvez l'exécutable :**
    - Sur Linux et macOS, l'exécutable se trouvera à `target/release/check`.
    - Sur Windows, il se trouvera à `target\release\check.exe`.

## Utilisation

Exécutez le programme depuis la ligne de commande, en passant le chemin d'un fichier ou d'un répertoire en argument.

### Arguments

- `<PATH>` : Le chemin vers le fichier ou le répertoire à inspecter.

### Options

L'outil prend en charge les options par défaut de `clap` :

- `-h`, `--help` : Affiche les informations d'aide.
- `-V`, `--version` : Affiche les informations de version.
- `-r`, `--recursive` : Traite les répertoires de manière récursive.
- `--sha`: Affiche la somme de contrôle SHA256.
- `--md5`: Affiche la somme de contrôle MD5.

## Exemples

### 1. Vérifier un seul fichier (avec SHA256)

```sh
./target/release/check --sha ./mon_fichier.txt
```

**Exemple de sortie :**

```
--- File: ./mon_fichier.txt ---
  Size: 1.21 KB
  Type: text/plain
  Encoding: UTF-8
  SHA256: <hash_sha256>
--------------------
```

### 2. Vérifier tous les fichiers d'un répertoire (avec les deux sommes de contrôle)

```sh
./target/release/check --sha --md5 ./mon_repertoire
```

**Exemple de sortie :**

```
Processing directory: ./mon_repertoire

--- File: ./mon_repertoire/file1.jpg ---
  Size: 5.54 KB
  Type: image/jpeg
  Encoding: ASCII
  SHA256: <hash_sha256_1>
  MD5: <hash_md5_1>
--------------------

--- File: ./mon_repertoire/document.pdf ---
  Size: 88.88 KB
  Type: application/pdf
  Encoding: ASCII
  SHA256: <hash_sha256_2>
  MD5: <hash_md5_2>
--------------------

```

### 3. Vérifier tous les fichiers d'un répertoire de manière récursive

```sh
./target/release/check -r --sha --md5 ./mon_repertoire
```

**Exemple de sortie :**

```
Processing directory: ./mon_repertoire

Processing directory: ./mon_repertoire/sous_repertoire1

--- File: ./mon_repertoire/sous_repertoire1/fichier_dans_sous_repertoire.txt ---
  Size: 1.21 KB
  Type: text/plain
  Encoding: UTF-8
  SHA256: <hash_sha256_3>
  MD5: <hash_md5_3>
--------------------

Processing directory: ./mon_repertoire/sous_repertoire2

--- File: ./mon_repertoire/file1.jpg ---
  Size: 5.54 KB
  Type: image/jpeg
  Encoding: ASCII
  SHA256: <hash_sha256_1>
  MD5: <hash_md5_1>
--------------------

--- File: ./mon_repertoire/document.pdf ---
  Size: 88.88 KB
  Type: application/pdf
  Encoding: ASCII
  SHA256: <hash_sha256_2>
  MD5: <hash_md5_2>
--------------------

```