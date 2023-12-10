use egui_extras::{Column, TableBuilder};

//costanti
pub const TITOLO_GUI: &str = "APPLICAZIONE DI PARTENZA";

//01 CREO ED IMPLEMENTO LA STRUTTURA DELLA MYAPP
//---------------------------------------------------------------------------//
struct MyApp {
    name: String,
    age: u32,
    riga: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            riga: 0,
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

            let num_righe = 24;  //provo : eui::SidePanel
            let button_clicked = ui.button("Next").clicked();
            if button_clicked {
                self.riga= (self.riga+ 1) % num_righe;
            }
            TableBuilder::new(ui)
               .column(Column::remainder().resizable(true))
               .column(Column::remainder().resizable(true))
               .scroll_to_row(self.riga, None)
               //@modifica = da 20.0 a 10.0??
               .header(20.0, |mut header| {
                   header.col(|app_ui| {
                       app_ui.heading("Id");
                   });
                   header.col(|app_ui| {
                       app_ui.heading("Azioni");
                   });
               }).body(|mut body| {
                for i in 0..num_righe {
                    body.row(20.0, |mut row| {
                        row.col(|app_ui| {
                            app_ui.label(format!("{i}"));
                        });
                        row.col(|app_ui| {

                            let _ = app_ui.button("Modifica");

                        });
                    });
                }
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

pub fn avvia_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        //grandezza iniziale form @modifica = da 320.0 a 240.0 in x320 y 390
        initial_window_size: Some(egui::vec2(400.0, 590.0)),
        ..Default::default()
    };
    eframe::run_native(
        TITOLO_GUI,
        options,
        Box::new(|_| {
            Box::<MyApp>::default()
        }),
    )
}
//---------------------------------------------------------------------------//
