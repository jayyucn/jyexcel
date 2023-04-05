// use druid::widget::{Button, Flex, Label};
// use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};
mod excel{
    pub mod excel_reader;
}

fn main()-> Result<(), calamine::Error>{
    // let main_window = WindowDesc::new(ui_builder());
    // let data = 0_u32;
    // AppLauncher::with_window(main_window)
    //     .log_to_console()
    //     .launch(data)
    ui_builder()
}

fn ui_builder()-> Result<(), calamine::Error> {
    // The label text will be computed dynamically based on the current locale and count
    // let text =
    //     LocalizedString::new("Find").with_arg("count", |data: &u32, _env| (*data).into());
    // let label = Label::new(text).padding(5.0).center();
    // let button = Button::new("increment")
    //     .on_click(|_ctx, data, _env| *data += 1)
    //     .padding(5.0);

    // Flex::column().with_child(label).with_child(button);
    excel::excel_reader::read_excels()
}