use std::borrow::Cow;

use eframe::egui::{
    CtxRef, FontData, FontDefinitions, FontFamily, Hyperlink, Label, RichText, Separator, TextStyle,
};

use crate::{CYAN, PADDING, WHITE};

pub struct Headlines {
    articles: Vec<NewsCardData>,
}

impl Headlines {
    pub fn new() -> Headlines {
        let iter = (0..20).map(|a| NewsCardData {
            title: format!("Headline {}", a),
            description: format!("This is the description of headline {}", a),
            url: format!("https://example.com/{}", a),
        });
        Headlines {
            articles: Vec::from_iter(iter),
        }
    }

    pub fn configure_fonts(&self, ctx: &CtxRef) {
        // create font def object
        let mut font_def = FontDefinitions::default();
        font_def.font_data.insert(
            "Rubik".to_string(),
            FontData {
                font: Cow::Borrowed(include_bytes!(
                    "../assets/fonts/Inconsolata-VariableFont_wdth,wght.ttf"
                )),
                index: 0,
            },
        );
        font_def
            .family_and_size
            .insert(TextStyle::Heading, (FontFamily::Proportional, 35.));
        font_def
            .family_and_size
            .insert(TextStyle::Body, (FontFamily::Proportional, 20.));
        font_def
            .fonts_for_family
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Rubik".to_string());

        ctx.set_fonts(font_def);
    }

    pub fn show_news_cards(&mut self, ui: &mut eframe::egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            let title = format!("▶ {}", &a.title);
            ui.colored_label(WHITE, title);
            ui.add_space(PADDING);
            let desc = Label::new(RichText::new(&a.description).text_style(TextStyle::Button));
            ui.add(desc);
            ui.style_mut().visuals.hyperlink_color = CYAN;
            ui.add_space(PADDING);
            ui.add(Hyperlink::from_label_and_url("read more ⤴", &a.url));
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }
}

struct NewsCardData {
    title: String,
    description: String,
    url: String,
}
