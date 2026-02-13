use eframe::egui;
use todo_inator::TodoList;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust Todo.txt Editor",
        native_options,
        Box::new(|_cc| Ok(Box::new(TodoInator::default()))),
    )
}

struct TodoInator {
    list: TodoList,
    new_task_input: String,
}

impl Default for TodoInator {
    fn default() -> Self {
        Self {
            // Try to load the file on startup, or start empty
            list: TodoList::load_file("todo.txt").unwrap_or_default(),
            new_task_input: String::new(),
        }
    }
}

impl eframe::App for TodoInator {
    // This function runs every single frame (60fps)
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("My Todo List");

            // --- SECTION: Input ---
            ui.horizontal(|ui| {
                let res = ui.text_edit_singleline(&mut self.new_task_input);
                if ui.button("Add Task").clicked() || (res.lost_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter))) {
                    if let Ok(_) = self.list.add_item(&self.new_task_input) {
                        self.new_task_input.clear();
                    }
                }
            });

            ui.separator();

            // --- SECTION: List Scroll Area ---
            egui::ScrollArea::vertical().show(ui, |ui| {
                for item in &self.list.items {
                    ui.label(item.to_string());
                }
            });
        });
    }
}
