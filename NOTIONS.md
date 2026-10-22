# Notions autorisées — Advent of Code (ESGI, Rust)

Sources : supports des chapitres 1 à 3 (cours, exercices et corrections), listings 01 à 09,
support « Présentation Projet ». Mis à jour au fil des énigmes.

---

## 1. Le cadre

| | Contrôle continu | Examen |
|---|---|---|
| Programme | chapitres 1 à 3 (+ « boîte à outils » de l'énoncé) | tout le module |
| Bibliothèque standard | **toute autorisée** (confirmé par le prof) | **toute autorisée** |
| Sujet | exercice de code en paliers | variante d'une énigme AoC 2023/2024/2025, jours 1 à 5 |

- Entrée sur **stdin**, sortie : **un seul nombre suivi d'un retour à la ligne** (`println!`), rien d'autre.
- Édition 2024, bibliothèque standard uniquement (ni `regex`, ni `itertools`).
- Collage désactivé : tout se tape. 5 envois notés. Ne compile pas = 0 sur l'exercice.
- Une variante change : les motifs cherchés, les cas limites, le calcul final.
  Le cœur (lecture de l'entrée, boucle principale) ne change pas.

### Squelette fourni
```rust
use std::io::Read;

fn main() {
    let mut input = String::new();
    if std::io::stdin().read_to_string(&mut input).is_ok() {
        print!("{input}");
    }
}
```

### Lancer avec un fichier (PowerShell)
```powershell
Get-Content inputs\input_2023_01.txt | cargo run --bin 2023_01
```
(`<` ne marche pas dans PowerShell. Alternative : `cmd /c "cargo run --bin 2023_01 < inputs\input_2023_01.txt"`)

- Toujours lancer `cargo run` depuis la **racine** du projet (sinon `NotFound` sur le fichier).
- Les programmes sont rangés par année (`src/bin/2024/...`) : Cargo ne les trouve pas seul.
  Pour chaque nouveau jour, ajouter dans `Cargo.toml` :
  ```toml
  [[bin]]
  name = "2024_03"
  path = "src/bin/2024/2024_03.rs"
  ```
  Sinon : `no targets specified in the manifest` ou `no bin target named ...`.

### Lire un fichier directement (entraînement seulement)
```rust
let input = std::fs::read_to_string("inputs/input_2023_02.txt").unwrap();
```
- Chemin relatif au dossier où l'on lance `cargo run` (la racine du projet).
- Erreur `NotFound` (code 2) = mauvais nom de fichier.
- **Jamais aux épreuves** : la plateforme donne l'entrée sur stdin. Garder la résolution
  dans une fonction `fn resoudre(input: String) -> u32` pour changer seulement `main`.
- Garder l'exemple de l'énoncé dans `inputs/example_AAAA_JJ.txt` (résultat connu).

---

## 2. Vu dans les chapitres 1 à 3

### Syntaxe et contrôle
- `fn nom(x: u32) -> u32`, `return x;` ou retour par **expression** (dernière ligne sans `;`)
- `let`, `let mut` (obligatoire pour modifier une variable), `+=`, `-=`
- Une variable doit avoir une valeur avant d'être lue -> `let mut x: u32 = 0;`
- `if / else if / else` (c'est aussi une expression : elle peut renvoyer une valeur)
- `while cond`, `for i in a..b`, `for i in a..=b`, `for _ in 0..n`
- `match` : valeurs, plages `1..=9`, `10 | 11`, gardes `n if n <= 20`, joker `_` ;
  bras testés dans l'ordre, doit être exhaustif
- `match (a % 3, a % 5) { (0, 0) => ..., (0, _) => ..., _ => ... }` (tuple)
- `enum` simple (+ variante avec donnée `Nombre(u32)`) et `match` dessus

### Types et conversions
- Types en minuscules : `u8`, `u32`, `u64`, `usize`, `bool`, `char`
- Suffixes `0u64`, séparateur `100_000`
- `x as u8` (peut tronquer), `u32::from(petit)` (sûr), `2u64.pow(30)`
- Débordement : panique en debug, silencieux en release -> choisir un type assez large

### Chaînes
- `&str` (morceau de chaîne, emprunté) en paramètre ; `String` (possédée) ; `"abc".to_string()`
- `s.len()` = nombre d'**octets** ; `s.chars().count()` = nombre de caractères
- `for c in s.chars()` : un caractère à la fois
- `for (i, c) in s.char_indices()` : position **en octets** + caractère, à chaque tour
  (le `(i, c)` découpe un tuple). `i` tombe toujours au début d'un caractère -> `&s[i..]` est sûr.
- Tranches : `&s[0..7]`, `&s[..fin]` (du début à fin), `&s[i..]` (de i jusqu'à la fin)
- Une tranche au milieu d'un caractère UTF-8 (`é`, `🦀`) panique ; `s.is_char_boundary(i)`
- **Type** : `&s[i..]` est un `&str`, pas un `char`. Les méthodes de `char`
  (`is_ascii_digit`...) ne marchent pas dessus.
- On ne peut pas écrire `s[i]` pour avoir le i-ème caractère.

### Divers
- `println!`, `print!`, `{}`, `{x}`, `{:?}`, `{:2}`, `println!()`
- `assert_eq!(a, b)` : tester une fonction seule (panique si différent)
- `Box::new(x)`, `*x`, tableau `[0u8; N]`, `vec![0u8; N]`

### Vu seulement dans une variante de correction (ch. 3, ex. 2)
- `.map(|(i, c)| ...)`, `.take_while(|&x| ...)`, `.last()`, `.unwrap_or(0)`, `c.len_utf8()`

---

## 3. Boîte à outils du support « Présentation Projet »
- `input.lines()`, `ligne.split_whitespace()`, `ligne.split(',')`
- `c.is_ascii_digit()` (méthode de `char`)
- `reste.starts_with("motif")` -> `true` / `false` (méthode de `&str`, avec un **s**)
- `c.to_digit(10)` -> renvoie une `Option` (`.unwrap()` pour avoir le nombre)
- `"42".parse::<i64>()` -> renvoie un `Result` (`.unwrap()` pour avoir le nombre)

## 3 bis. Découper une chaîne (vu au jour 2023-02)

| Outil | Donne | S'utilise avec |
|---|---|---|
| `s.split(';')` | **plusieurs** morceaux | `for morceau in s.split(';') { ... }` |
| `s.split_whitespace()` | plusieurs morceaux, espaces en trop ignorés | `for` ou `collect` |
| `s.split_once(':')` | **exactement deux** morceaux (`Option`) | `let (gauche, droite) = s.split_once(':').unwrap();` |
| `.collect()` | range les morceaux dans un `Vec` | `let v: Vec<&str> = s.split(' ').collect();` puis `v[0]`, `v[1]` |
| `.nth(1)` | le morceau d'indice 1 (`Option`) | `s.split_whitespace().nth(1).unwrap()` |
| `.trim()` | enlève les espaces au début et à la fin | après un `split(',')` |
| `.parse::<u32>()` | texte -> nombre (`Result`) | `.parse::<u32>().unwrap()` |

- Les indices d'un `Vec` commencent à **0** ; `v[1]` panique s'il n'y a qu'un morceau.
- On **ne peut pas** écrire `let (a, b) = s.split(...)` : `split` ne donne pas une paire.
- On **ne peut pas** faire `for ... in s.split_once(...)` : c'est une paire, pas une liste.
- Découper selon la **structure** (séparateurs), jamais selon une longueur supposée
  (« les identifiants vont jusqu'à 100 ») : une variante peut changer les tailles.
- Méthode : lister les niveaux de séparateurs du plus gros au plus fin
  (`:` -> `;` -> `,` -> espace), un niveau = un découpage = une boucle (ou un `let`).
- Déboguer un découpage : `println!("{:?}", morceau);` montre les espaces cachés.

## 3 ter. Listes (`Vec`) et emprunts (vu en 2024-01 et 2024-02)

| Outil | Effet |
|---|---|
| `let mut v: Vec<u32> = Vec::new();` | liste vide (`mut` pour pouvoir la remplir) |
| `v.push(n);` | ajoute `n` à la fin (le nombre va **entre les parenthèses**) |
| `v.len()` | taille, de type `usize` |
| `v[i]` | élément à la position `i` (de `0` à `len() - 1`) |
| `v.sort();` | trie la liste elle-même (croissant) |
| `v.swap(i, j);` | échange deux cases |
| `let mut c = v.clone();` | copie indépendante (l'original n'est pas touché) |
| `c.remove(i);` | retire l'élément `i`, les suivants se décalent |
| `a.abs_diff(b)` | écart positif entre deux `u32`, quel que soit l'ordre |

**Passer une liste à une fonction :**
- `fn f(v: Vec<u32>)` + `f(liste)` : la fonction **prend** la liste, elle est détruite à la fin
  de `f` -> `liste` inutilisable après (`borrow of moved value`).
- `fn f(v: &Vec<u32>)` + `f(&liste)` : **prêt en lecture** (ex. `is_croissant`).
- `fn f(v: &mut Vec<u32>)` + `f(&mut liste)` : **prêt modifiable** (ex. une fonction de tri).

**Soustractions en `u32` / `usize` :**
- `petit - grand` **panique** (pas de négatif) -> `if a > b { a - b } else { b - a }` ou `abs_diff`.
- `v.len() - 1` sur une liste vide panique -> `if v.len() < 2 { ... }` si une variante peut en donner.
- `a > b && a - b <= 3` est sûr : le `&&` ne calcule la droite que si la gauche est vraie.

**Tri à bulles** (pour comprendre ; aux épreuves, utiliser `sort()`) :
```
echange <- vrai
TANT QUE echange :
    echange <- faux                       (au début de CHAQUE passage)
    POUR i de 0 à len-2 :
        SI v[i] > v[i+1] : échanger, echange <- vrai
```

## 4. Toute la bibliothèque standard est autorisée (confirmé par le prof)
Seules les **crates extérieures** sont interdites (`regex`, `itertools`...).
Donc utilisables aux deux épreuves, même si le cours ne les a pas encore détaillés :
- `c.to_digit(10).unwrap()`, `"42".parse::<u32>().unwrap()`
- `Option`, `Result`, `Some`, `None`, `if let`, `is_none`, `unwrap`
- `Vec`, `push`, `sort`, `HashMap`, `HashSet`, `contains`, `iter`...

Attention : `unwrap()` **panique** si la valeur est absente (ex. `parse` sur une ligne vide).
Une variante peut mettre exprès ce genre de cas limite -> vérifier avant (`is_ascii_digit`, ligne vide...).

## 5. Astuces utiles (sans Option, toujours valables)
- Valeur d'un chiffre : `c as u32 - '0' as u32` (les codes de `'0'` à `'9'` se suivent)
- Nombre construit à la main : `n = n * 10 + chiffre`
- « Déjà trouvé ? » : un `bool`, remis à `false` à chaque ligne
- « Rien trouvé » : une **valeur impossible** (0 si les valeurs vont de 1 à 9)

---

## 6. Méthodes (ce qui a marché)

1. **Lire l'énoncé, noter les cas limites**, faire passer l'exemple avant la vraie entrée.
2. **Séparer** la lecture de l'entrée et la résolution (une fonction qui renvoie le résultat).
3. **Découper** : une petite fonction pour « qu'est-ce qui commence ici ? »,
   testée seule avec `assert_eq!`, puis la boucle principale autour.
4. **Calculer une fois, puis décider** : mettre la valeur trouvée dans une variable,
   puis choisir quoi en faire (premier / dernier), au lieu de recopier la recherche
   dans chaque branche.
5. **Avancer d'une position à la fois** avec `&reste[i..].starts_with(...)` :
   gère tout seul les motifs qui se chevauchent.
6. **Dérouler à la main** sur une ligne d'exemple quand on est perdu :
   noter la valeur de chaque variable à chaque tour.
7. **Partie 2 : repartir de la partie 1**, ne pas tout réécrire.
8. Quand le code devient illisible : reculer, réécrire le plan en français, puis recoder.
9. **Chercher le contre-exemple** : pour « tout est valide », un seul booléen à `true`
   au début de la ligne, qui passe à `false` dès qu'un élément casse la règle (et n'y revient jamais).
10. **Maximum courant** : variable à 0 au début de la ligne ; `if x > max { max = x; }` à chaque élément.
11. **Placer le calcul au bon niveau de boucle** : ce qui concerne une ligne entière
    (ajouter au total, produit) se fait **après** les boucles internes, une fois tout lu.
12. **Partie 2 : relire ce qui ne sert plus.** Ne pas recopier aveuglément la partie 1
    (limites, booléens, identifiants inutiles).
13. **Reformuler l'énoncé en français** sur un exemple quand on ne le comprend pas.
14. **Choisir la structure de données avant de coder** : deux colonnes à trier séparément
    = deux `Vec`, pas un `Vec` de paires (2024-01).
15. **Fonction « tout est valide » = `return false` dès la première erreur**, `true` à la fin
    (plus besoin de variable booléenne ; une liste d'un seul élément est alors valide).
16. **Partie 2 « on a droit à une erreur » : essayer toutes les possibilités.**
    Pour chaque position, fabriquer une copie sans cet élément, la tester avec les fonctions
    **inchangées** de la partie 1, `break` dès qu'une copie passe (sinon compté plusieurs fois).
17. **Ne pas modifier une fonction qui marche** pour la partie 2 : ajouter la logique autour.

## 7. Pièges rencontrés
- Variables déclarées hors de la boucle des lignes -> pas remises à zéro.
- `!found;` ne modifie rien -> il faut `found = true;`.
- `if / else` pour premier / dernier : sur une ligne à un seul chiffre, le dernier n'est jamais rempli.
- Un booléen jamais passé à `true` -> une branche ne s'exécute jamais.
- Oublier `if value != 0` -> « rien trouvé » écrase une vraie valeur.
- Boucle imbriquée par erreur (reparcourir toute la ligne à chaque position).
- `total = ...` au lieu de `total += ...` ; `println!` dans la boucle au lieu d'après.
- Appeler une variable `char` ou `str` (noms de types) -> confusion.
- Passer par une `String` puis `parse` alors qu'un calcul `a * 10 + b` suffit.
- `blue_ok == true;` (comparaison) au lieu de `blue_ok = true;` (affectation).
- Inverser `v[0]` et `v[1]` -> `parse` sur `"blue"` -> panique.
- Un `if` placé dans la boucle interne au lieu de la fin de ligne -> ajouté plusieurs fois.
- Copier-coller de blocs : les blocs vert/bleu modifiaient encore `max_red`.
- Comparer la limite (`red_count`) au lieu de la valeur lue (`nb_cube`).
- Logique « prouver que tout va bien » (`red_ok = true` dès qu'un tirage est correct)
  au lieu de « chercher un dépassement ».
- Oublier qu'une couleur peut être **absente** d'une partie (max reste à 0).

---

## 8. Journal des énigmes

| Énigme | P1 | P2 | Notions clés | Piège |
|---|---|---|---|---|
| 2023-01 Trebuchet | ✅ 142 | ✅ 281 | `lines`, `char_indices`, `&s[i..]`, `starts_with`, fonction à valeur sentinelle, booléen `found` | chevauchements `twone`, ligne à un seul chiffre |
| 2023-02 Cube Conundrum | ✅ 8 | ✅ 2286 | `split_once`, `split`, `split_whitespace`, `collect` en `Vec`, `nth`, `parse`, booléen contre-exemple, maximum courant | ordre `v[0]`/`v[1]`, niveau de boucle du calcul, couleur absente |
| 2023-03 Gear Ratios | | | | |
| 2023-04 Scratchcards | | | | |
| 2023-05 Seed/Fertilizer | | | | |
| 2024-01 Historian Hysteria | | | | |
| 2024-02 Red-Nosed Reports | | | | |
| 2024-03 Mull It Over | | | | |
| 2024-04 Ceres Search | | | | |
| 2024-05 Print Queue | | | | |
| 2025-01 Secret Entrance | | | | |
| 2025-02 Gift Shop | | | | |
| 2025-03 Lobby | | | | |
| 2025-04 Printing Department | | | | |
| 2025-05 Cafeteria | | | | |

### À faire sur 2023-01
- [x] `print!` -> `println!`
- [x] Supprimer le `if / else` final : le dernier reçoit aussi le premier chiffre
- [ ] Refaire une variante type examen de zéro, sans regarder le code

### À faire sur 2023-02
- [ ] Partie 2 : supprimer `game_index` (inutile), fusionner les `if` imbriqués
- [ ] Déclarer les `max_` dans la boucle des lignes (plus de remise à zéro à la main)
- [ ] Réfléchir : si une variante dit « une couleur absente compte pour 1 », que changer ?
- [ ] Refaire une variante type examen de zéro, sans regarder le code

---

## 9. Notions à revoir (repérées pendant l'entraînement)

| Notion | Pourquoi la revoir | Où |
|---|---|---|
| `=` (affecter) vs `==` (comparer), `!x;` ne modifie rien | erreur faite 2 fois (2023-01, 2023-02) | ch. 2, bindings |
| Une variable doit être initialisée et `mut` pour changer | plusieurs erreurs de compilation | ch. 2, bindings |
| Types : `char` vs `&str` vs `String` vs `u32` | `is_ascii_digit` sur un `&str`, `s[i]` | ch. 3 |
| Plusieurs morceaux (`split`) vs une paire (`split_once`) | `for` sur une paire, `let (a, b)` sur `split` | section 3 bis |
| Portée des variables : déclarer **dans** la boucle des lignes | variables non remises à zéro (2 jours de suite) | méthode 11 |
| Où placer un calcul par rapport aux boucles imbriquées | ajout dans la mauvaise boucle (2023-01 et 02) | méthode 11 |
| Logique booléenne « au moins un » / « aucun » | `*_ok = true` au lieu du contre-exemple | méthode 9 |
| `Option` / `Result` et `unwrap` : quand ça panique | `parse` sur un morceau vide ou du texte | ch. 5 (à venir) |
| Lire l'énoncé en entier, partie 2 comprise | partie 2 de 2023-02 mal comprise au début | méthode 13 |

