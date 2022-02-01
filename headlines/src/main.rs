mod headlines;

use eframe::{
    egui::{
        vec2, CentralPanel, Color32, Hyperlink, Label, RichText, ScrollArea, Separator,
        TopBottomPanel, Ui,
    },
    epi::App,
    run_native, NativeOptions,
};
use headlines::Headlines;

pub const PADDING: f32 = 5.0;
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

impl App for Headlines {
    fn setup(
        &mut self,
        ctx: &eframe::egui::CtxRef,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        self.configure_fonts(&ctx);
    }

    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            show_header(ui);
            ScrollArea::both()
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    self.show_news_cards(ui);
                });
            show_footer(&ctx);
        });
    }

    fn name(&self) -> &str {
        "Headlines"
    }
}

fn show_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let sep = Separator::default().spacing(20.);
    ui.add(sep);
}

fn show_footer(ctx: &eframe::egui::CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Label::new(
                RichText::new("API source: newsapi.org").monospace(),
            ));
            ui.add(Hyperlink::from_label_and_url(
                RichText::new("made with egui").monospace(),
                "https://github.com/emilk/egui/",
            ));
            ui.add(Hyperlink::from_label_and_url(
                RichText::new("sebbev/headlines ❤").monospace(),
                "https://github.com/sebbev/headlines/",
            ));
            // ui.label("© 2022 by Sebastian Vallin");
            ui.add_space(10.);
        });
    });
}

fn main() {
    let app = Headlines::new();
    let mut win_options = NativeOptions::default();
    win_options.initial_window_size = Some(vec2(800., 600.));
    run_native(Box::new(app), win_options);
}
