use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::Dipendente;
use crate::ui::app_ui::avvia_gui;

// use egui_extras::{Table, TableBody, TableBuilder};
pub mod models;
pub mod schema;

mod ui {
   pub mod app_ui;
}

fn main() {

   // SCHEMA DIPENDENTE + ATTIVA LA CONNESSIONE SUL DB
   // -------------------------------------------------------------//

   use self::schema::dipendente::dsl::*;
   let connessione = &mut establish_connection();

   let resul: Vec<Dipendente> = dipendente
      .select(Dipendente::as_select())
      .load::<Dipendente>(connessione)
      .unwrap();
   // -------------------------------------------------------------//

   //@STAMPA @VIDEO SU TERMINALE I DIPENDENTI
   // -------------------------------------------------------------//
   //Note : attivo il print ed eseguo un ciclo form e stampo un result
   //      sul terminale
   println!("stampo i dipendenti");
   for mydipendente in resul {
      println!("{}", mydipendente.ritorna_dipendente());
   }
   // -------------------------------------------------------------//

   //@AVVIO LA @FORM @INIZIALE E LE GUI
   avvia_gui().unwrap();
}


//FUNZIONE: pub fn establish_connection = CONNESSIONE SQLLITE
// -------------------------------------------------------------//
pub fn establish_connection() -> SqliteConnection {
   dotenv().ok();

   let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
   return SqliteConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}
//*** FINE ** pub fn establish_connection = CONNESSIONE SQLLITE
//-------------------------------------------------------------//

