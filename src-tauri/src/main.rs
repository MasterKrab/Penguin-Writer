use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use syntect::highlighting::ThemeSet;

#[tauri::command]
fn markdown_to_html(app: tauri::AppHandle, value: &str) -> String {
    let mut options = Options::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.superscript = true;
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.extension.tagfilter = true;
    options.extension.shortcodes = true;
    options.parse.smart = true;
    options.render.hardbreaks = true;
    options.render.unsafe_ = true;

    let path_resolver = app.path_resolver();

    let path = path_resolver
        .resolve_resource("themes")
        .expect("theme resources not resolved");

    let themes = ThemeSet::load_from_folder(path).expect("themes not loaded");

    let adapter = SyntectAdapterBuilder::new()
        .theme_set(themes)
        .theme("Dracula")
        .build();

    let mut plugins = Plugins::default();

    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    return markdown_to_html_with_plugins(value, &options, &plugins);
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![markdown_to_html])
        .run(context)
        .expect("error while running tauri application");
}
