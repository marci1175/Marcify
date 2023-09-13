use std::{fs::File, path::PathBuf};
use egui::{RichText, Layout};
use rfd::FileDialog;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink, OutputStreamHandle};
use rodio::source::{SineWave, Source, Amplify, TakeDuration};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] 
pub struct TemplateApp {
    #[serde(skip)]
    stream: OutputStream,
    #[serde(skip)]
    stream_handle: OutputStreamHandle,
    #[serde(skip)]
    sink: Option<Sink>,
    #[serde(skip)]
    opened_file: Option<PathBuf>,
    #[serde(skip)]
    title: String,
    #[serde(skip)]
    src: Option<SineWave>
}

impl Default for TemplateApp {
    fn default() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Self {
            stream,
            stream_handle,
            sink: None,
            opened_file: None,
            title: String::from("Now playing : "),
            src: None
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        #[cfg(not(target_arch = "wasm32"))] 
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let open = ui.button("Open file").on_hover_text("Open a file");
            if open.clicked() { 
                self.opened_file = FileDialog::new()
                    .set_title("Save as")
                    .set_directory("/")
                    .add_filter("Supported audio formats", &[&"ogg", &"mp3"])
                    .pick_file();

            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            //if self.sink.is_some() {dbg!(self.sink.as_mut().unwrap().is_paused());}
            ui.label(RichText::from(RichText::from(self.title.clone()).size(25.)));
            if let Some(music) = &self.opened_file {
                //set music reader
                
                    ui.with_layout(Layout::left_to_right(egui::Align::Center), |ui| {
                        let start = ui.button("Start").on_hover_text("Start music");
                        if start.clicked() {
                           
                            let file = BufReader::new(File::open(self.opened_file.clone().unwrap()).unwrap());
                           
                            let source = Decoder::new(file).unwrap();
                            
                            //let _play = self.stream_handle.play_raw(source.convert_samples());
                            
                            //set ui
                            self.title =  "Now playing : ".to_string() + &music.file_name().unwrap().to_string_lossy().to_string();
                            
                            self.sink = Some(Sink::try_new(&self.stream_handle).unwrap());

                            self.sink.as_mut().unwrap().append(source.amplify(999.));
                                                
                        }
                        
                        if let Some(Sink) = self.sink.as_mut() {
                            
                            let pause = ui.button("Pause");
                            if pause.clicked() {
                                Sink.pause();
                                
                            }
                            let unpause = ui.button("Unpause");
                            if unpause.clicked() {
                                Sink.play();
                                
                                
                            }
                        }
                        
                    });

                }

            
            
            
        });

    }
}
