# Texicon — A Beautiful Icon + Text Button Widget for egui

**Texicon** is a highly customizable, stateful icon + text widget for **[egui](https://github.com/emilk/egui)** — perfect for toolbars, sidebars, dashboards, or any UI where you need clean, modern-looking clickable icons with optional labels and rich interaction states (normal, hover, selected).

It supports:
- Custom SVG/PNG images with tinting
- Multi-state coloring (normal / hover / selected)
- Optional text below the icon with automatic centering and wrapping
- Clickable image and/or text areas
- Full frame or image+text-only interaction sensing
- Tooltips with customizable position and gap
- Builder pattern for clean, fluent configuration
- Runs on Linux, Mac, Windows

Ideal for desktop apps, tools, editors, and games built with `egui` + `eframe`.

## Screenshot

![Screenshot](images/Screenshot1.png)

## Preview

* [Live wasm browser example.](https://dreamy-meringue-f98d25.netlify.app/)

## Quick Start

### Option 1. Download the Texicon demo app.
* Download XXX from [Github](https://github.com/White-Rabbit-Scientific/egui-widget-texicon-demo-app) then compile and run: ```cargo run```

### Option 2. Begin from eframe_template app.
* Download eframe_template from [Github](https://github.com/emilk/eframe_template).
* Ensure ```rust-toolchain``` and ```Cargo.toml``` have the latest rust versions.
* Add ```egui-widget-texicon``` and ```egui_extras``` to `Cargo.toml`. ```egui_extras``` is required for egui to load images.
```toml
[dependencies]
egui-widget-texicon = 0.2
egui_extras = { version = "0.33", features = ["default", "all_loaders"] }
```
The egui image loaded also requires adding to ```main.rs```. The complete code block in ```main.rs``` should be as follows:
```toml
Box::new(|cc| {
    // This gives us image support:
    egui_extras::install_image_loaders(&cc.egui_ctx);
    Ok(Box::new(eframe_template::TemplateApp::new(cc)))
}),
```
Select the image to use for the Texicon. The following will embed the image into the compiled binary file.
```toml
const TEXI_IMG: egui::ImageSource<'_> = egui::include_image!("../assets/icon-1024.png");
```
Display the Texicon:
```toml
ui.add(egui_widget_texicon::Texicon::new(TEXI_IMG));
```
Texicon has a large number of customization options. Here's an example of how to use them:
```toml
egui_widget_texicon::Texicon::new(TEXI_IMG)
    .enabled(true)
    .selected(true)
    .img_size(egui::vec2(50., 50.))
    .img_scale_hov(1.5)
    .text("Button text")
    .text_size(18.)
    .bkgnd_col(egui::Color32::RED)
    .img_tint_col_hov(egui::Color32::PURPLE)
    .text_col_hov(egui::Color32::ORANGE)
    .frame_col_hov(egui::Color32::GREEN)
    .frame_size(egui::vec2(100., 150.))
    .frame_width(4.)
    .tooltip_text("I am a tooltip".to_string())
    .tooltip_gap(40.)
    .tooltip_position(egui::RectAlign::BOTTOM),
```


## License

Licensed under either of:
* Apache License, Version 2.0 (LICENSE-APACHE)
* MIT License (LICENSE-MIT)

at your option.

## Author
Made with ❤️ by Rob @ White Rabbit Scientific

Inspired by modern design tools and clean UI patterns.

Star this repo if you like it! ⭐️
