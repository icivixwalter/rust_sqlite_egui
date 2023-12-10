use egui_extras::{Column, TableBuilder};
use crate::models::Dipendente;


//costanti
pub const TITOLO_GUI: &str = "APPLICAZIONE DI PARTENZA";

//01 CREO ED IMPLEMENTO LA STRUTTURA DELLA MYAPP
//---------------------------------------------------------------------------//
struct MyApp {
   name: String,
   age: u32,
   riga: usize,
   vettore_dipendenti: Vec<Dipendente>,
}

/// Metodi esclusivi della struttura MyApp
impl MyApp {
   fn new(lista_dipendenti: Vec<Dipendente>) -> Self {
      let mut my_app = MyApp::default();
      my_app.vettore_dipendenti = lista_dipendenti;
      return my_app;
   }
}

/// alla creazione dell'oggetto Myapp imposto i valori di default definiti in questo metodo.
impl Default for MyApp {
   fn default() -> Self {
      Self {
         name: "Arthur".to_owned(),
         age: 42,
         riga: 0,
         vettore_dipendenti: vec![], // Vec::new() default è vuoto
      }
   }
}
//---------------------------------------------------------------------------//

//02 ATTIVO LA  MYAPP
//---------------------------------------------------------------------------//
impl eframe::App for MyApp {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      //OLD
      egui::CentralPanel::default().show(ctx, |ui| {
         //ho provato la modifica -->egui::horizontal_wrapped()::default().show(ctx, |ui| {

         //titolo applicazione + label + text
         ui.heading("My egui Application");
         ui.horizontal(|ui| {
            let name_label = ui.label("Your name: ");
            ui.text_edit_singleline(&mut self.name)
               .labelled_by(name_label.id);
         });
         //@modifica da 0...120 a 0...20
         ui.add(egui::Slider::new(&mut self.age, 0..=20).text("age"));
         if ui.button("Click each year").clicked() {
            self.age += 1;
         }
         ui.label(format!("Hello '{}', age {}", self.name, self.age));

         let num_righe = self.vettore_dipendenti.len();  // numero di record recuperati
         let button_clicked = ui.button("Next").clicked();
         if button_clicked {
            self.riga = (self.riga + 1) % num_righe;
         }

         //01 creo l'oggetto table con 5 colonne
         TableBuilder::new(ui)
            .column(Column::remainder().at_least(40.0).resizable(true))
            .column(Column::remainder().at_least(80.0).resizable(true))
            .column(Column::remainder().at_least(50.0).resizable(true))
            .column(Column::remainder().at_least(50.0).resizable(true))
            .column(Column::remainder().at_least(50.0).resizable(true))

            //02 creo le 5 intestazioni di colonna
            .header(20.0, |mut header| {
               // definisci le colonne
               header.col(|app_ui| {
                  app_ui.heading("ID_DIPEN_lng");
               });
               header.col(|app_ui| {
                  app_ui.heading("DENOMINAZIONE_s");
               });

               header.col(|app_ui| {
                  app_ui.heading("COGNOME_S");
               });
               header.col(|app_ui| {
                  app_ui.heading("NOME_S");
               });
               header.col(|app_ui| {
                  app_ui.heading("INIZIALI_DIP_S");
               });
            }).body(|body| {
            //03 itero nel record ed associo i 5 valori dei campi

               body.rows(20.0, num_righe, |i, mut row| {
                  if let Some(dip) = self.vettore_dipendenti.get(i) {
                  row.col(|app_ui| {
                     app_ui.label(format!("{}", dip.get_ID_DIPEN_lng().unwrap()));
                  });
                  row.col(|app_ui| {

                     //@da@errore E NON CAPISCO PERCHE' NELLA COLONNA @DENOMINAZIONE
                     app_ui.label(dip.get_DENOMINAZIONE_s().unwrap_or(&"null".to_owned()));
                  });
                  row.col(|app_ui| {
                     app_ui.label(dip.get_COGNOME_S().unwrap_or(&"null".to_owned()));
                  });
                  row.col(|app_ui| {
                     let _ = app_ui.label(dip.get_NOME_S().unwrap_or(&"null".to_owned()));
                  });
                  row.col(|app_ui| {
                     let _ = app_ui.label(dip.get_INIZIALI_DIP_S().unwrap_or(&"null".to_owned()));
                  });
               }
            });
         });
      });
   }
}
//---------------------------------------------------------------------------//

//old ---> egui::TopBottomPanel::top("Tabella").show(ctx, |app_ui| {
//@modifica = al posto di egui::TopBottomPanel::top è stato impostato
//l'impostazione right a destra per affiiancarlo ai valori
//egui::SidePanel::right = imposta il panella della tabella a sx del progetto
// egui::SidePanel::right("Tabella").show(ctx, |app_ui| {
//
// });
//imposto la tabella
//------------------------------------------------------------------------------------//


//03 AVVIO LA  MYAPP
//---------------------------------------------------------------------------//

pub fn avvia_gui(lista_dipendenti: Vec<Dipendente>) -> Result<(), eframe::Error> {
   let options = eframe::NativeOptions {
      //grandezza iniziale form @modifica = da 320.0 a 240.0 in x320 y 390
      initial_window_size: Some(egui::vec2(800.0, 600.0)),
      ..Default::default()
   };
   eframe::run_native(
      TITOLO_GUI,
      options,
      Box::new(|_| {
         Box::new(MyApp::new(lista_dipendenti))
      }),
   )
}
//---------------------------------------------------------------------------//
