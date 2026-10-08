//! Swatch Libraries (#536): Window → Swatch Libraries and the Swatches panel's libraries menu open
//! the library panel, clicked through the whole window as a user would.

use egui::{Event, PointerButton, Pos2, Rect, pos2, vec2};
use serde_json::json;
use vectorcraft_engine::Session;

use crate::VectorcraftApp;

/// A short window, so the Window menu scrolls to reach its library submenus.
const SW: f32 = 1100.0;
const SH: f32 = 640.0;

/// One headless frame of the whole window with `events` → the text painted, with its rect.
fn frame(app: &mut VectorcraftApp, ctx: &egui::Context, events: Vec<Event>) -> Vec<(String, Rect)> {
    let raw = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(SW, SH))),
        events,
        time: Some(ctx.input(|i| i.time) + 0.1),
        ..Default::default()
    };
    let mut out = ctx.run_ui(raw, |ui| {
        app.logic(ui.ctx());
        app.ui(ui);
    });
    out.textures_delta.clear();
    fn walk(s: &egui::Shape, v: &mut Vec<(String, Rect)>) {
        match s {
            egui::Shape::Text(t) => v.push((t.galley.text().to_string(), Rect::from_min_size(t.pos, t.galley.size()))),
            egui::Shape::Vec(s) => s.iter().for_each(|s| walk(s, v)),
            _ => {}
        }
    }
    let mut v = vec![];
    out.shapes.iter().for_each(|c| walk(&c.shape, &mut v));
    v
}

fn at(texts: &[(String, Rect)], label: &str) -> Pos2 {
    texts
        .iter()
        .rev()
        .find(|(t, _)| t.trim_start_matches(['✓', ' ']) == label)
        .map(|(_, r)| r.center())
        .unwrap_or_else(|| panic!("no `{label}` in {texts:?}"))
}

fn hover(app: &mut VectorcraftApp, ctx: &egui::Context, p: Pos2) -> Vec<(String, Rect)> {
    frame(app, ctx, vec![Event::PointerMoved(p)]);
    frame(app, ctx, vec![]);
    frame(app, ctx, vec![])
}

fn click(app: &mut VectorcraftApp, ctx: &egui::Context, p: Pos2) -> Vec<(String, Rect)> {
    let b = |pressed| Event::PointerButton { pos: p, button: PointerButton::Primary, pressed, modifiers: Default::default() };
    frame(app, ctx, vec![Event::PointerMoved(p)]);
    frame(app, ctx, vec![b(true)]);
    frame(app, ctx, vec![b(false)]);
    frame(app, ctx, vec![]);
    frame(app, ctx, vec![])
}

fn app() -> (VectorcraftApp, egui::Context) {
    let mut app = VectorcraftApp::new(Session::new(), Default::default());
    app.run("file.new", json!({"width": 400, "height": 300})).unwrap();
    let ctx = egui::Context::default();
    for _ in 0..3 {
        frame(&mut app, &ctx, vec![]);
    }
    (app, ctx)
}

#[test]
fn window_swatch_libraries_opens_the_library_panel() {
    let (mut app, ctx) = app();
    let texts = frame(&mut app, &ctx, vec![]);
    let texts = click(&mut app, &ctx, at(&texts, "Window"));
    // Scroll the menu down to its library submenus.
    let menu = at(&texts, "Actions");
    let wheel = Event::MouseWheel {
        unit: egui::MouseWheelUnit::Point,
        delta: vec2(0.0, -2000.0),
        modifiers: Default::default(),
        phase: egui::TouchPhase::Move,
    };
    frame(&mut app, &ctx, vec![Event::PointerMoved(menu), wheel]);
    for _ in 0..10 {
        frame(&mut app, &ctx, vec![]);
    }
    let texts = frame(&mut app, &ctx, vec![]);
    let texts = hover(&mut app, &ctx, at(&texts, "Swatch Libraries"));
    let p = at(&texts, "Brights");
    let texts = hover(&mut app, &ctx, pos2(p.x, p.y));
    let texts = click(&mut app, &ctx, at(&texts, "Brights"));
    assert_eq!(app.ui.library_panel.as_ref().map(|o| o.id.as_str()), Some("brights"));
    assert!(texts.iter().any(|(t, _)| t == "Add to Swatches" || t == "Brights"), "{texts:?}");
}

#[test]
fn every_swatch_library_shows() {
    let (mut app, ctx) = app();
    for l in vectorcraft_color::libraries::SWATCH_LIBRARIES.iter().chain(vectorcraft_color::libraries::GRADIENT_LIBRARIES) {
        app.run("window.swatchLibrary", json!({"library": l.id})).unwrap();
        frame(&mut app, &ctx, vec![]);
        let texts = frame(&mut app, &ctx, vec![]);
        assert!(!app.ui.status.contains("Internal error"), "{}: {}", l.id, app.ui.status);
        assert!(texts.iter().any(|(t, _)| t == l.name), "{} shows: {texts:?}", l.id);
        assert!(ctx.memory(|m| m.area_rect(egui::Id::new(("library-panel", "swatches")))).is_some());
    }
}

/// Where the button whose tooltip is `tip` is in `area`: hovered along its bottom rows.
fn button_with_tip(app: &mut VectorcraftApp, ctx: &egui::Context, area: Rect, tip: &str) -> Pos2 {
    for dy in [22.0, 18.0, 26.0, 30.0, 14.0] {
        for i in 0..12 {
            let p = pos2(area.left() + 12.0 + 6.0 * i as f32, area.bottom() - dy);
            frame(app, ctx, vec![Event::PointerMoved(p)]);
            for _ in 0..8 {
                if frame(app, ctx, vec![]).iter().any(|(t, _)| t == tip) {
                    return p;
                }
            }
        }
    }
    panic!("no button `{tip}` in {area:?}");
}

#[test]
fn the_swatches_panel_library_button_opens_a_library() {
    let (mut app, ctx) = app();
    app.run("window.panel", json!({"panel": "swatches"})).unwrap();
    for _ in 0..3 {
        frame(&mut app, &ctx, vec![]);
    }
    let area = ctx.memory(|m| m.area_rect(egui::Id::new("icon-panel"))).expect("the Swatches flyout");
    let p = button_with_tip(&mut app, &ctx, area, "Swatch Libraries Menu");
    let texts = click(&mut app, &ctx, p);
    let texts = click(&mut app, &ctx, at(&texts, "Pastels"));
    assert_eq!(app.ui.library_panel.as_ref().map(|o| o.id.as_str()), Some("pastels"), "{}", app.ui.status);
    assert!(ctx.memory(|m| m.area_rect(egui::Id::new(("library-panel", "swatches")))).is_some());
    assert!(texts.iter().any(|(t, _)| t == "Pastels"), "{texts:?}");
}

#[test]
fn a_library_shows_without_a_document() {
    let mut app = VectorcraftApp::new(Session::new(), Default::default());
    let ctx = egui::Context::default();
    for _ in 0..3 {
        frame(&mut app, &ctx, vec![]);
    }
    app.run("window.swatchLibrary", json!({"library": "pastels"})).unwrap();
    frame(&mut app, &ctx, vec![]);
    let texts = frame(&mut app, &ctx, vec![]);
    assert!(!app.ui.status.contains("Internal error"), "{}", app.ui.status);
    assert!(texts.iter().any(|(t, _)| t == "Pastels"), "{texts:?}");
}
