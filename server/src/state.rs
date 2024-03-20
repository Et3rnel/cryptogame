use super::player::Player;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex}; // Assurez-vous d'avoir le chemin correct vers le module `player`

pub static USER_STATES: Lazy<Arc<Mutex<HashMap<String, Player>>>> =
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));
