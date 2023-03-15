use eframe::egui::{self, Ui};
#[macro_use]
extern crate derive_new;

const PADDING: f32 = 5.0;
const WINDOW_SCALE: f32 = 1.4;

enum MessageData {
    Text(String),
    File { filename: String, data: Vec<u8> },
}

enum MessageFrom {
    Me,
    Remote,
}

#[derive(new)]
struct Message {
    from: MessageFrom,
    data: MessageData,
}

struct CommunicatorApp {
    messages: Vec<Message>,
    current_message: String
}

impl CommunicatorApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let mut messages_vec = Vec::new();
        for i in 0..50 {
            messages_vec.push(Message::new(
                MessageFrom::Me,
                MessageData::Text(String::from(format!("message{}", i))),
            ));
            messages_vec.push(Message::new(
                MessageFrom::Remote,
                MessageData::Text(String::from(format!("message{}", i))),
            ));
        }
        CommunicatorApp {
            messages: messages_vec,
            current_message: String::from("")
        }
    }

    fn render_messages(&self, ui: &mut Ui) {
        for message in &self.messages {
            if let MessageData::Text(text) = &message.data {
                ui.add_space(PADDING);
                ui.label(text);
            }
        }
    }

    fn render_send_message_ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let message_input = ui.text_edit_singleline(&mut self.current_message);
            if message_input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))
            || ui.button("Send").clicked() {
                self.send_message();
            }
        });
    }

    fn send_message(&mut self) {
        todo!()
    }
}

impl eframe::App for CommunicatorApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(WINDOW_SCALE);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    self.render_messages(ui);
                    self.render_send_message_ui(ui);
                });
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Box::new(CommunicatorApp::new(cc))),
    );
}
