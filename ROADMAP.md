---

# 📌 Task CLI — Rust Project

## 🎯 Objectif

Développer une application en ligne de commande permettant de gérer des tâches avec :

* création
* listing
* complétion
* suppression
* persistance locale

Le projet sert à **consolider les bases Rust** sur un cas concret.

---

# ⚙️ Fonctionnalités (MVP)

## ➕ Ajouter une tâche

```bash
task add "Learn Rust"
```

## 📋 Lister les tâches

```bash
task list
task list --completed
task list --pending
```

## ✅ Marquer comme complétée

```bash
task done 1
```

## ❌ Supprimer une tâche

```bash
task remove 1
```

---

# 🧱 Modèle de données

```rust
struct Task {
    id: u64,
    description: String,
    status: Status,
    created_at: DateTime,
    completed_at: Option<DateTime>,
}

enum Status {
    Pending,
    Completed,
}
```

---

# 💾 Persistance

* Format : JSON
* Fichier local (ex: `~/.task-cli/tasks.json`)
* Chargement au démarrage
* Sauvegarde après modification
* Sérialisation via Serde

---

# 🧩 Architecture

```text
src/
 ├── main.rs        # point d’entrée
 ├── cli.rs         # parsing CLI
 ├── task.rs        # logique métier
 ├── storage.rs     # persistance
 ├── error.rs       # erreurs custom
```

### Responsabilités

| Module  | Rôle                         |
| ------- | ---------------------------- |
| cli     | parsing arguments (via clap) |
| task    | logique métier               |
| storage | lecture/écriture             |
| error   | gestion d’erreurs            |

---

# ❗ Contraintes techniques

* ❌ pas de `unwrap()` hors tests
* ❌ pas de `panic!()` en production
* ✅ gestion d’erreurs avec `Result`
* ✅ code modulaire
* ✅ tests unitaires
* ✅ séparation claire des responsabilités

---

# 🗺 Roadmap

## 🧱 Phase 0 — Setup

* `cargo new task --bin`
* Vérifier que le binaire fonctionne
* `cargo run`

---

## 🧩 Phase 1 — CLI

Objectif : parser les commandes

* Intégrer clap
* Implémenter :

    * `add`
    * `list`
    * `done`
    * `remove`
* Aucune logique métier

✔ Output attendu : afficher les arguments reçus

---

## 🧠 Phase 2 — Logique métier (en mémoire)

Objectif : gérer les tâches sans fichier

* Créer :

    * `Task`
    * `Status`
    * `TaskManager`
* Implémenter :

    * add
    * list
    * done
    * remove

Contraintes :

* `Result` partout
* tests unitaires

---

## 💾 Phase 3 — Persistance

Objectif : sauvegarder les tâches

* Intégrer Serde
* Implémenter :

    * load()
    * save()
* Gérer :

    * fichier absent
    * erreurs I/O

---

## 🧪 Phase 4 — Tests

* tests unitaires (logique métier)
* tests d’intégration
* tests de sérialisation

---

## 🧰 Phase 5 — Améliorations CLI

* filtres (`--completed`, `--pending`)
* affichage lisible
* tri
* messages d’erreur propres

---

## 🚀 Phase 6 — Extensions (optionnel)

### Fonctionnelles

* tags
* priorités
* recherche
* export CSV

### Techniques

* SQLite
* architecture avancée
* TUI (interface terminal)

---

# 🎯 Objectif final

Un binaire installable :

```bash
cargo build --release
./target/release/task
```

ou :

```bash
cargo install --path .
```

---

# 🧠 Compétences travaillées

* ownership & borrowing
* `Option` / `Result`
* modularité
* sérialisation
* tests
* CLI parsing
* gestion d’erreurs

---

# ✅ Critères de réussite

* toutes les commandes fonctionnent
* données persistantes
* aucun panic
* tests passent
* code propre et modulaire

---
