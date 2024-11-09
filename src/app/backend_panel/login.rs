use eframe::NativeOptions;
use egui::CentralPanel;

use egui_form::garde::field_path;
use egui_form::{Form, FormField};
use garde::Validate;

#[derive(Validate, Debug)]
pub struct Login {
    #[garde(length(min = 8, max = 32))]
    pub user_name: String,
    #[garde(email)]
    pub email: String,
    #[garde(dive)]
    pub nested: Nested,
    #[garde(dive)]
    pub vec: Vec<Nested>,
}

#[derive(Validate, Debug, Default)]
pub struct Nested {
    #[garde(range(min = 1, max = 10))]
    pub test: u64,
}

impl Default for Login {
    fn default() -> Self {
        // respect all minimums
        Self {
            user_name: "an_8_character_Username!".to_string(),
            email: "password123!".to_string(),
            nested: Nested { test: 0 },
            vec: vec![Nested { test: 0 }],
        }
    }
}

impl Login {
    /// Login UI
    pub fn login_ui(&mut self, ui: &mut egui::Ui) {
        let mut form = Form::new().add_report(egui_form::garde::GardeReport::new(self.validate()));

        FormField::new(&mut form, "user_name")
            .label("User Name")
            .ui(ui, egui::TextEdit::singleline(&mut self.user_name));
        FormField::new(&mut form, "email")
            .label("Email")
            .ui(ui, egui::TextEdit::singleline(&mut self.email));
        FormField::new(&mut form, field_path!("nested", "test"))
            .label("Nested Test")
            .ui(ui, egui::Slider::new(&mut self.nested.test, 0..=11));
        FormField::new(&mut form, field_path!("vec", 0, "test"))
            .label("Vec Test")
            .ui(
                ui,
                egui::DragValue::new(&mut self.vec[0].test).range(0..=11),
            );

        if let Some(Ok(())) = form.handle_submit(&ui.button("Submit"), ui) {
            println!("Form submitted: {self:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui_form::garde::GardeReport;
    use egui_form::{EguiValidationReport, IntoFieldPath};

    #[test]
    fn test() {
        let test = Login {
            user_name: "testfiwuehfwoi".to_string(),
            email: "garbage".to_string(),
            nested: Nested { test: 0 },
            vec: vec![Nested { test: 0 }],
        };

        let report = GardeReport::new(test.validate());

        assert!(report
            .get_field_error("user_name".into_field_path())
            .is_some());
        assert!(report.get_field_error(field_path!("email")).is_some());
        assert!(report
            .get_field_error(field_path!("nested", "test"))
            .is_some());
        assert!(report
            .get_field_error(field_path!("vec", 0, "test"))
            .is_some());

        assert_eq!(report.error_count(), 4);
    }
}
