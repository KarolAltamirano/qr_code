use qrcode::render::svg;
use qrcode::QrCode;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: qr_code <qr_code_content> <file_name>");
        return;
    }

    let content = &args[1];
    let file_name = &args[2];

    let code = QrCode::new(content.as_bytes()).unwrap();
    let image = code
        .render()
        .min_dimensions(400, 400)
        .dark_color(svg::Color("#000000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    fs::write(format!("{file_name}.svg"), image).unwrap();
}
