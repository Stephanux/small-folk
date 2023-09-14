
mod appdyn;
mod view;

use std::{
    collections::{hash_map::Entry, HashMap},
    sync::Arc,
};

/* Module principal contenant la fonction main de lancement de l'application */
use serde_json::{self, Value};


fn  main() {
    println!("small-folk is starting...");
    appdyn::appdyn::main();
}
