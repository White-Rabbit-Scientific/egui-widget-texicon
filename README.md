# Texicon — A Beautiful Icon + Text Button Widget for egui

**Texicon** is a highly customizable, stateful icon + text widget for **[egui](https://github.com/emilk/egui)** — perfect for toolbars, sidebars, dashboards, or any UI where you need clean, modern-looking clickable icons with optional labels and rich interaction states (normal, hover, selected).

It supports:
- Custom SVG/PNG images with tinting
- Multi-state coloring (normal / hover / selected)
- Optional text below the icon with automatic centering and wrapping
- Clickable full frame or image+text-only interaction sensing
- Tooltips with customizable position and gap
- Builder pattern for clean, fluent configuration
- Runs on Linux, Mac, Windows, WebAssembly (wasm) and Raspberry Pi
- Images are compiled into the binary executable

Ideal for desktop apps, tools, editors, and games built with [egui](https://github.com/emilk/egui/) + [eframe](https://github.com/emilk/eframe_template).

## Live wasm demo app

The wasm [demo app](https://dreamy-meringue-f98d25.netlify.app/) runs in your web browser.

![Screenshot](images/Screenshot1.png)
![Screenshot](images/Screenshot2.png)

## Theming

The demo app uses the [Catppuccin](https://catppuccin.com/) ```egui-themes``` widget with extended color palette, available on White Rabbit's [Github repo](https://github.com/White-Rabbit-Scientific/egui-themes).

## Quick Start

### Option 1. Download the Texicon demo app.
* [Download from Github](https://github.com/White-Rabbit-Scientific/egui-widget-texicon-demo-app) then compile and run: ```cargo run```

### Option 2. Begin from eframe_template app.
* [Download the eframe_template](https://github.com/emilk/eframe_template) from Github.
* Ensure ```rust-toolchain``` and ```Cargo.toml``` specify the latest rust version.
* Add ```egui-widget-texicon``` and ```egui_extras``` to `Cargo.toml`. ```egui_extras``` is required for egui to load images.
```toml
[dependencies]
egui-widget-texicon = 0.2
egui_extras = { version = "0.33", features = ["default", "all_loaders"] }
```
```main.rs``` needs a small modification to include the image loader. Modify the ```Box::new``` code block:
```toml
Box::new(|cc| {
    // This gives us image support:
    egui_extras::install_image_loaders(&cc.egui_ctx);
    Ok(Box::new(eframe_template::TemplateApp::new(cc)))
}),
```
In ```app.rs``` select an image to use for the Texicon. The following uses an existing ```eframe_template``` image:
```toml
const TEXI_IMG: egui::ImageSource<'_> = egui::include_image!("../assets/icon-1024.png");
```
The Texicon widget is added into your app as follows. TEXI_IMG is passed to the widget:
```toml
ui.add(egui_widget_texicon::Texicon::new(TEXI_IMG));
```
Texicon widgets can be customized as shown in the example below:
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
