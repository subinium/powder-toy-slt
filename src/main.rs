use std::time::{Duration, Instant};
use std::{fs, path::Path};

use slt::*;

mod elements;
mod simulation;

use elements::{elements_in_category, Behavior, Element, CATEGORY_NAMES, NUM_CATEGORIES};
use simulation::Simulation;

const BG: Color = Color::Rgb(12, 12, 18);
const SIDEBAR_W: u32 = 30;
const SAVE_DIR: &str = "save";
const SAVE_SLOT_MAX: usize = 5;
const HISTORY_LIMIT: usize = 64;

#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Normal,
    Pressure,
    Heat,
    Electric,
    Airflow,
}

impl ViewMode {
    fn next(self) -> Self {
        match self {
            ViewMode::Normal => ViewMode::Pressure,
            ViewMode::Pressure => ViewMode::Heat,
            ViewMode::Heat => ViewMode::Electric,
            ViewMode::Electric => ViewMode::Airflow,
            ViewMode::Airflow => ViewMode::Normal,
        }
    }

    fn label(self) -> &'static str {
        match self {
            ViewMode::Normal => "Normal",
            ViewMode::Pressure => "Pressure",
            ViewMode::Heat => "Heat",
            ViewMode::Electric => "Electric",
            ViewMode::Airflow => "Airflow",
        }
    }
}

struct SidebarState<'a> {
    cat_idx: usize,
    elem_idx: usize,
    displayed_elems: &'a [(usize, Element)],
    selected: Element,
    hovered_elem: Option<Element>,
    save_slot: usize,
    search_mode: bool,
    search_query: &'a str,
    grid_overlay: bool,
    brush_size: usize,
    brush_shape: BrushShape,
    eraser: bool,
    sim: &'a Simulation,
    count: usize,
    fps: u32,
    tps: u32,
    view_mode: ViewMode,
    zoom_level: usize,
    viewport_x: usize,
    viewport_y: usize,
    elem_counts: &'a [(Element, usize)],
    electric_activity: usize,
    history: &'a [f64],
    pressure_stats: (f32, f32),
    heat_stats: (f32, f32),
    velocity_stats: (f32, f32),
    status: &'a str,
    stamp_mode: StampMode,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum BrushShape {
    Circle,
    Square,
    Line,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum StampMode {
    None,
    Selecting,
    Pasting,
}

impl StampMode {
    fn label(self) -> Option<&'static str> {
        match self {
            StampMode::None => None,
            StampMode::Selecting => Some("COPY"),
            StampMode::Pasting => Some("PASTE"),
        }
    }
}

impl BrushShape {
    fn next(self) -> Self {
        match self {
            BrushShape::Circle => BrushShape::Square,
            BrushShape::Square => BrushShape::Line,
            BrushShape::Line => BrushShape::Circle,
        }
    }

    fn label(self) -> &'static str {
        match self {
            BrushShape::Circle => "Circle",
            BrushShape::Square => "Square",
            BrushShape::Line => "Line",
        }
    }
}

fn main() -> std::io::Result<()> {
    let (tw, th) = crossterm::terminal::size().unwrap_or((120, 40));
    let sidebar = SIDEBAR_W as usize;
    let canvas_w = (tw as usize).saturating_sub(sidebar);
    let canvas_h = (th as usize).saturating_sub(2);
    let sim_w = canvas_w;
    let sim_h = canvas_h * 2;

    let mut sim = Simulation::new(sim_w, sim_h);
    setup_demo(&mut sim);

    let mut cat_idx: usize = 0;
    let mut elem_idx: usize = 0;
    let mut brush_size: usize = 3;
    let mut brush_shape = BrushShape::Circle;
    let mut eraser = false;
    let mut history: Vec<f64> = Vec::with_capacity(64);
    let mut fps_time = Instant::now();
    let mut fps_frames: u32 = 0;
    let mut fps: u32 = 0;
    let mut tps_time = Instant::now();
    let mut tps_last_tick = sim.tick;
    let mut tps: u32 = 0;
    let mut view_mode = ViewMode::Normal;
    let mut undo_stack: Vec<Simulation> = Vec::new();
    let mut redo_stack: Vec<Simulation> = Vec::new();
    let mut status = String::from("Ready");
    let mut save_slot: usize = 1;
    let mut grid_overlay = false;
    let mut search_mode = false;
    let mut search_query = String::new();
    let mut zoom_level: usize = 1;
    let mut viewport_x: usize = 0;
    let mut viewport_y: usize = 0;
    let mut stamp_mode = StampMode::None;
    let mut stamp_start: Option<(usize, usize)> = None;
    let mut stamp_data: Vec<Option<Element>> = Vec::new();
    let mut stamp_w: usize = 0;
    let mut stamp_h: usize = 0;

    slt::run_with(
        RunConfig {
            mouse: true,
            tick_rate: Duration::from_millis(33),
            theme: Theme::dark(),
            max_fps: Some(30),
        },
        move |ui: &mut Context| {
            fps_frames += 1;
            if fps_time.elapsed().as_millis() >= 1000 {
                fps = fps_frames;
                fps_frames = 0;
                fps_time = Instant::now();
            }

            if ui.key('q') {
                ui.quit();
            }
            if search_mode {
                if ui.key_code(KeyCode::Esc) {
                    search_mode = false;
                    search_query.clear();
                    status = String::from("Search cancelled");
                }
            } else if ui.key_code(KeyCode::Esc) {
                if stamp_mode != StampMode::None {
                    stamp_mode = StampMode::None;
                    stamp_start = None;
                    status = String::from("Stamp mode cancelled");
                } else {
                    ui.quit();
                }
            }

            if !search_mode && ui.key('/') {
                search_mode = true;
                search_query.clear();
                elem_idx = 0;
                status = String::from("Search mode");
            }

            if search_mode {
                append_search_input(ui, &mut search_query);
            }

            if !search_mode && ui.key(' ') {
                sim.paused = !sim.paused;
                status = if sim.paused {
                    String::from("Paused")
                } else {
                    String::from("Running")
                };
            }
            if !search_mode && ui.key('r') {
                remember_state(&sim, &mut undo_stack, &mut redo_stack);
                sim.clear();
                history.clear();
                status = String::from("Cleared")
            }
            if !search_mode && ui.key('e') {
                eraser = !eraser;
                status = if eraser {
                    String::from("Eraser mode")
                } else {
                    String::from("Draw mode")
                };
            }
            if !search_mode && ui.key('v') {
                view_mode = view_mode.next();
                status = format!("View: {}", view_mode.label());
            }
            if !search_mode && ui.key('g') {
                sim.gravity_mode = sim.gravity_mode.next();
                status = format!("Gravity: {}", sim.gravity_mode.label());
            }
            if !search_mode && ui.key('f') {
                sim.sim_speed = match sim.sim_speed {
                    1 => 2,
                    2 => 4,
                    _ => 1,
                };
                status = format!("Speed: {}x", sim.sim_speed);
            }
            if !search_mode && ui.key('b') {
                brush_shape = brush_shape.next();
                status = format!("Brush shape: {}", brush_shape.label());
            }
            if !search_mode && ui.key('n') && sim.paused {
                remember_state(&sim, &mut undo_stack, &mut redo_stack);
                sim.step_once();
                status = String::from("Step")
            }
            if !search_mode && ui.key('z') {
                if let Some(prev) = undo_stack.pop() {
                    redo_stack.push(sim.clone());
                    sim = prev;
                    status = String::from("Undo")
                }
            }
            if !search_mode && ui.key('y') {
                if let Some(next) = redo_stack.pop() {
                    undo_stack.push(sim.clone());
                    sim = next;
                    status = String::from("Redo")
                }
            }
            if !search_mode && ui.key('S') {
                save_slot = (save_slot % SAVE_SLOT_MAX) + 1;
                status = format!("Save slot: {save_slot}");
            }
            if !search_mode && ui.key('s') {
                let save_path = save_path_for_slot(save_slot);
                if let Some(parent) = Path::new(&save_path).parent() {
                    if fs::create_dir_all(parent).is_ok()
                        && fs::write(&save_path, sim.serialize()).is_ok()
                    {
                        status = format!("Saved: {save_path}");
                    } else {
                        status = String::from("Save failed");
                    }
                }
            }
            if !search_mode && ui.key('l') {
                let save_path = save_path_for_slot(save_slot);
                match fs::read_to_string(&save_path) {
                    Ok(content) => {
                        if let Some(loaded) = Simulation::deserialize(&content) {
                            remember_state(&sim, &mut undo_stack, &mut redo_stack);
                            sim = loaded;
                            status = format!("Loaded: {save_path}");
                        } else {
                            status = String::from("Load parse failed");
                        }
                    }
                    Err(_) => {
                        status = String::from("No save file");
                    }
                }
            }
            if !search_mode && ui.key('o') {
                grid_overlay = !grid_overlay;
                status = if grid_overlay {
                    String::from("Grid overlay on")
                } else {
                    String::from("Grid overlay off")
                };
            }
            if !search_mode && ui.key('p') {
                let screenshot_path = format!("{SAVE_DIR}/screenshot_{}.txt", sim.tick);
                if fs::create_dir_all(SAVE_DIR).is_ok()
                    && write_screenshot(&sim, &screenshot_path).is_ok()
                {
                    status = format!("Screenshot: {screenshot_path}");
                } else {
                    status = String::from("Screenshot failed");
                }
            }
            if !search_mode && ui.key(']') {
                brush_size = (brush_size + 1).min(12);
            }
            if !search_mode && ui.key('[') {
                brush_size = brush_size.saturating_sub(1).max(1);
            }

            if !search_mode && ui.key('c') {
                stamp_mode = StampMode::Selecting;
                stamp_start = None;
                status = String::from("Stamp copy: click first corner");
            }
            if !search_mode && ui.key('x') {
                if stamp_w > 0 && stamp_h > 0 && !stamp_data.is_empty() {
                    stamp_mode = StampMode::Pasting;
                    status = String::from("Stamp paste: click canvas");
                } else {
                    status = String::from("No stamp copied");
                }
            }

            if !search_mode && (ui.key('+') || ui.key('=')) && zoom_level < 4 {
                zoom_level += 1;
                clamp_viewport(&sim, zoom_level, &mut viewport_x, &mut viewport_y);
                status = format!("Zoom: {}x", zoom_level);
            }
            if !search_mode && (ui.key('-') || ui.key('_')) && zoom_level > 1 {
                zoom_level -= 1;
                clamp_viewport(&sim, zoom_level, &mut viewport_x, &mut viewport_y);
                status = format!("Zoom: {}x", zoom_level);
            }

            if !search_mode && zoom_level > 1 {
                let pan_step = zoom_level;
                if ui.key_code(KeyCode::Home) {
                    viewport_x = viewport_x.saturating_sub(pan_step);
                }
                if ui.key_code(KeyCode::End) {
                    viewport_x = viewport_x.saturating_add(pan_step);
                }
                if ui.key_code(KeyCode::PageUp) {
                    viewport_y = viewport_y.saturating_sub(pan_step);
                }
                if ui.key_code(KeyCode::PageDown) {
                    viewport_y = viewport_y.saturating_add(pan_step);
                }
                clamp_viewport(&sim, zoom_level, &mut viewport_x, &mut viewport_y);
            }

            if !search_mode && ui.key_code(KeyCode::Left) {
                cat_idx = if cat_idx == 0 {
                    NUM_CATEGORIES - 1
                } else {
                    cat_idx - 1
                };
                elem_idx = 0;
            }
            if !search_mode && ui.key_code(KeyCode::Right) {
                cat_idx = (cat_idx + 1) % NUM_CATEGORIES;
                elem_idx = 0;
            }

            let mut displayed_elems =
                collect_displayed_elements(cat_idx, search_mode, &search_query);

            if search_mode {
                if !displayed_elems.is_empty() {
                    if ui.key_code(KeyCode::Up) {
                        elem_idx = if elem_idx == 0 {
                            displayed_elems.len() - 1
                        } else {
                            elem_idx - 1
                        };
                    }
                    if ui.key_code(KeyCode::Down) {
                        elem_idx = (elem_idx + 1) % displayed_elems.len();
                    }
                }
                if ui.key_code(KeyCode::Enter) {
                    if let Some((new_cat_idx, selected_elem)) =
                        displayed_elems.get(elem_idx).copied()
                    {
                        cat_idx = new_cat_idx;
                        if let Some(new_elem_idx) = elements_in_category(cat_idx)
                            .iter()
                            .position(|elem| *elem == selected_elem)
                        {
                            elem_idx = new_elem_idx;
                        }
                        status = format!("Selected: {}", selected_elem.name());
                    }
                    search_mode = false;
                    search_query.clear();
                }
            } else {
                let cat_elems = elements_in_category(cat_idx);

                if ui.key_code(KeyCode::Up) {
                    elem_idx = if elem_idx == 0 {
                        cat_elems.len() - 1
                    } else {
                        elem_idx - 1
                    };
                }
                if ui.key_code(KeyCode::Down) {
                    elem_idx = (elem_idx + 1) % cat_elems.len();
                }

                for (i, c) in ['1', '2', '3', '4', '5', '6', '7', '8', '9']
                    .iter()
                    .enumerate()
                {
                    if ui.key(*c) && i < cat_elems.len() {
                        elem_idx = i;
                    }
                }

                if ui.key_code(KeyCode::Tab) {
                    elem_idx += 1;
                    if elem_idx >= cat_elems.len() {
                        elem_idx = 0;
                        cat_idx = (cat_idx + 1) % NUM_CATEGORIES;
                    }
                }
            }

            displayed_elems = collect_displayed_elements(cat_idx, search_mode, &search_query);
            if displayed_elems.is_empty() {
                elem_idx = 0;
            } else {
                elem_idx = elem_idx.min(displayed_elems.len().saturating_sub(1));
            }

            let cat_elems = elements_in_category(cat_idx);
            let current_elem_idx = elem_idx.min(cat_elems.len().saturating_sub(1));
            let selected = displayed_elems
                .get(elem_idx)
                .map_or(cat_elems[current_elem_idx], |(_, elem)| *elem);

            let mut hovered_elem: Option<Element> = None;
            let mouse_pos = ui.mouse_pos();
            clamp_viewport(&sim, zoom_level, &mut viewport_x, &mut viewport_y);
            if let Some((mx, my)) = mouse_pos {
                if (mx as usize) >= sim.width {
                    let sidebar_row = my as usize;
                    let header_rows = 6;
                    if sidebar_row >= header_rows {
                        let hovered_idx = sidebar_row - header_rows;
                        if let Some((_, elem)) = displayed_elems.get(hovered_idx) {
                            hovered_elem = Some(*elem);
                        }
                    }
                }
            }

            if let Some((mx, my)) = ui.mouse_down() {
                let screen_x = mx as usize;
                let gx = viewport_x + (mx as usize) / zoom_level;
                let gy = viewport_y + ((my as usize) * 2) / zoom_level;
                let sidebar_x = sim.width;
                if screen_x >= sidebar_x {
                    let sidebar_row = my as usize;
                    let header_rows = 6;
                    if sidebar_row >= header_rows {
                        let clicked_elem = sidebar_row - header_rows;
                        if let Some((clicked_cat, clicked_element)) =
                            displayed_elems.get(clicked_elem).copied()
                        {
                            cat_idx = clicked_cat;
                            if let Some(new_elem_idx) = elements_in_category(cat_idx)
                                .iter()
                                .position(|elem| *elem == clicked_element)
                            {
                                elem_idx = new_elem_idx;
                            }
                            if search_mode {
                                search_mode = false;
                                search_query.clear();
                            }
                            status = format!("Selected: {}", clicked_element.name());
                        }
                    }
                } else if gy < sim.height {
                    match stamp_mode {
                        StampMode::Selecting => {
                            if let Some((sx, sy)) = stamp_start {
                                let min_x = sx.min(gx);
                                let max_x = sx.max(gx).min(sim.width.saturating_sub(1));
                                let min_y = sy.min(gy);
                                let max_y = sy.max(gy).min(sim.height.saturating_sub(1));
                                stamp_w = max_x - min_x + 1;
                                stamp_h = max_y - min_y + 1;
                                stamp_data.clear();
                                stamp_data.reserve(stamp_w * stamp_h);
                                for yy in min_y..=max_y {
                                    for xx in min_x..=max_x {
                                        stamp_data
                                            .push(sim.get(xx, yy).as_ref().map(|p| p.element));
                                    }
                                }
                                stamp_mode = StampMode::None;
                                stamp_start = None;
                                status = format!("Copied stamp {}x{}", stamp_w, stamp_h);
                            } else {
                                stamp_start = Some((gx, gy));
                                status = String::from("Stamp copy: click second corner");
                            }
                        }
                        StampMode::Pasting => {
                            remember_state(&sim, &mut undo_stack, &mut redo_stack);
                            for sy in 0..stamp_h {
                                for sx in 0..stamp_w {
                                    let data_idx = sy * stamp_w + sx;
                                    if let Some(elem) = stamp_data.get(data_idx).and_then(|e| *e) {
                                        let px = gx + sx;
                                        let py = gy + sy;
                                        if px < sim.width && py < sim.height {
                                            sim.place_brush(px, py, elem, 1);
                                        }
                                    }
                                }
                            }
                            status = format!("Pasted stamp {}x{}", stamp_w, stamp_h);
                        }
                        StampMode::None => {
                            remember_state(&sim, &mut undo_stack, &mut redo_stack);
                            if eraser {
                                apply_brush(
                                    &mut sim,
                                    gx,
                                    gy,
                                    selected,
                                    brush_size,
                                    brush_shape,
                                    true,
                                );
                            } else {
                                apply_brush(
                                    &mut sim,
                                    gx,
                                    gy,
                                    selected,
                                    brush_size,
                                    brush_shape,
                                    false,
                                );
                            }
                            status = format!("Brush {}", selected.name());
                        }
                    }
                }
            }

            let brush_preview = mouse_pos.and_then(|(mx, my)| {
                let gx = viewport_x + (mx as usize) / zoom_level;
                let gy = viewport_y + ((my as usize) * 2) / zoom_level;
                if gx < sim.width && gy < sim.height {
                    Some((gx, gy, brush_size, brush_shape))
                } else {
                    None
                }
            });

            sim.update();

            let tps_elapsed = tps_time.elapsed();
            if tps_elapsed.as_secs_f64() >= 1.0 {
                let tick_delta = sim.tick.saturating_sub(tps_last_tick);
                tps = (tick_delta as f64 / tps_elapsed.as_secs_f64()).round() as u32;
                tps_last_tick = sim.tick;
                tps_time = Instant::now();
            }

            let count = sim.particle_count();
            if sim.tick.is_multiple_of(5) {
                history.push(count as f64);
                if history.len() > 50 {
                    history.remove(0);
                }
            }

            let elem_counts = sim.element_counts();
            let electric_activity = sim.electric_activity();
            let pressure_stats = sim.pressure_stats();
            let heat_stats = sim.heat_stats();
            let velocity_stats = sim.velocity_stats();

            ui.col(|ui| {
                ui.container().grow(1).row(|ui| {
                    ui.container().grow(1).bg(BG).col(|ui| {
                        render_canvas(
                            ui,
                            &sim,
                            view_mode,
                            grid_overlay,
                            brush_preview,
                            zoom_level,
                            viewport_x,
                            viewport_y,
                        );
                    });

                    ui.container()
                        .w(SIDEBAR_W)
                        .border(Border::Rounded)
                        .border_left(true)
                        .px(1)
                        .col(|ui| {
                            render_sidebar(
                                ui,
                                &SidebarState {
                                    cat_idx,
                                    elem_idx,
                                    displayed_elems: &displayed_elems,
                                    selected,
                                    hovered_elem,
                                    save_slot,
                                    search_mode,
                                    search_query: &search_query,
                                    grid_overlay,
                                    brush_size,
                                    brush_shape,
                                    eraser,
                                    sim: &sim,
                                    count,
                                    fps,
                                    tps,
                                    view_mode,
                                    zoom_level,
                                    viewport_x,
                                    viewport_y,
                                    elem_counts: &elem_counts,
                                    electric_activity,
                                    history: &history,
                                    pressure_stats,
                                    heat_stats,
                                    velocity_stats,
                                    status: &status,
                                    stamp_mode,
                                },
                            );
                        });
                });

                ui.container().bg(Color::Rgb(20, 20, 30)).row(|ui| {
                    ui.help(&[
                        ("q", "quit"),
                        ("Space", "pause"),
                        ("r", "clear"),
                        ("n", "step"),
                        ("e", "eraser"),
                        ("b", "shape"),
                        ("v", "view"),
                        ("g", "gravity"),
                        ("f", "speed"),
                        ("s/l", "save/load"),
                        ("S", "slot"),
                        ("/", "search"),
                        ("c", "copy"),
                        ("x", "paste"),
                        ("o", "grid"),
                        ("p", "shot"),
                        ("z/y", "undo/redo"),
                        ("\u{2190}\u{2192}", "category"),
                        ("\u{2191}\u{2193}", "element"),
                        ("+/-", "zoom"),
                        ("Home/End", "pan X"),
                        ("PgUp/PgDn", "pan Y"),
                        ("[/]", "brush"),
                        ("Click", "draw"),
                    ]);
                });
            });
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn render_canvas(
    ui: &mut Context,
    sim: &Simulation,
    view_mode: ViewMode,
    grid_overlay: bool,
    brush_preview: Option<(usize, usize, usize, BrushShape)>,
    zoom_level: usize,
    viewport_x: usize,
    viewport_y: usize,
) {
    let display_rows = sim.height.div_ceil(2);
    for dy in 0..display_rows {
        let top_y = dy * 2;
        let bot_y = dy * 2 + 1;

        ui.row(|ui| {
            let mut run_start: usize = 0;
            let mut run_fg = paint_color(
                sim,
                viewport_x,
                viewport_y + top_y / zoom_level,
                view_mode,
                grid_overlay,
                brush_preview,
            );
            let mut run_bg = paint_color(
                sim,
                viewport_x,
                viewport_y + bot_y / zoom_level,
                view_mode,
                grid_overlay,
                brush_preview,
            );

            for x in 1..sim.width {
                let sim_x = viewport_x + x / zoom_level;
                let fg = paint_color(
                    sim,
                    sim_x,
                    viewport_y + top_y / zoom_level,
                    view_mode,
                    grid_overlay,
                    brush_preview,
                );
                let bg = paint_color(
                    sim,
                    sim_x,
                    viewport_y + bot_y / zoom_level,
                    view_mode,
                    grid_overlay,
                    brush_preview,
                );

                if fg != run_fg || bg != run_bg {
                    let s: String = "\u{2580}".repeat(x - run_start);
                    ui.styled(&s, Style::new().fg(run_fg).bg(run_bg));
                    run_start = x;
                    run_fg = fg;
                    run_bg = bg;
                }
            }

            let s: String = "\u{2580}".repeat(sim.width - run_start);
            ui.styled(&s, Style::new().fg(run_fg).bg(run_bg));
        });
    }
}

fn clamp_viewport(
    sim: &Simulation,
    zoom_level: usize,
    viewport_x: &mut usize,
    viewport_y: &mut usize,
) {
    let zoom = zoom_level.max(1);
    let visible_w = sim.width.div_ceil(zoom);
    let visible_h = sim.height.div_ceil(zoom);
    let max_x = sim.width.saturating_sub(visible_w);
    let max_y = sim.height.saturating_sub(visible_h);
    *viewport_x = (*viewport_x).min(max_x);
    *viewport_y = (*viewport_y).min(max_y);
}

fn paint_color(
    sim: &Simulation,
    x: usize,
    y: usize,
    view_mode: ViewMode,
    grid_overlay: bool,
    brush_preview: Option<(usize, usize, usize, BrushShape)>,
) -> Color {
    let mut color = pixel_color(sim, x, y, view_mode);
    if grid_overlay && (x.is_multiple_of(10) || y.is_multiple_of(10)) {
        color = mix_color(color, Color::Rgb(0, 0, 0), 0.12);
    }
    if let Some((cx, cy, radius, shape)) = brush_preview {
        if on_brush_outline(x, y, cx, cy, radius, shape) {
            color = mix_color(color, Color::Rgb(220, 220, 220), 0.35);
        }
    }
    color
}

fn pixel_color(sim: &Simulation, x: usize, y: usize, view_mode: ViewMode) -> Color {
    if y >= sim.height {
        return BG;
    }

    if view_mode == ViewMode::Pressure {
        let p = sim.pressure_at(x, y);
        return if p > 0.0 {
            let v = (p * 255.0).clamp(0.0, 255.0) as u8;
            Color::Rgb(v, 60, 40)
        } else {
            let v = (-p * 255.0).clamp(0.0, 255.0) as u8;
            Color::Rgb(40, 90, v)
        };
    }

    if view_mode == ViewMode::Heat {
        let h = sim.heat_at(x, y).clamp(0.0, 1.0);
        let r = (h * 255.0) as u8;
        let g = ((1.0 - (h - 0.5).abs() * 2.0) * 180.0).max(0.0) as u8;
        let b = ((1.0 - h) * 255.0) as u8;
        return Color::Rgb(r, g, b);
    }

    if view_mode == ViewMode::Airflow {
        let (vx, vy) = sim.velocity_at(x, y);
        let speed = (vx * vx + vy * vy).sqrt().clamp(0.0, 1.5) / 1.5;
        let r = (vx.max(0.0) * 255.0).clamp(0.0, 255.0) as u8;
        let b = ((-vx).max(0.0) * 255.0).clamp(0.0, 255.0) as u8;
        let g = (speed * 220.0).clamp(0.0, 220.0) as u8;
        return Color::Rgb(r.max(20), g, b.max(20));
    }

    match sim.get(x, y) {
        Some(p) => match p.element {
            Element::Fire => {
                let r = (rand::random::<u8>() % 60) as i16;
                Color::Rgb(
                    (255 - r / 2).max(150) as u8,
                    (160i16 - r).max(30) as u8,
                    (40 + r / 3).min(80) as u8,
                )
            }
            Element::Lava => {
                let r = (rand::random::<u8>() % 40) as i16;
                Color::Rgb(
                    (255 - r / 3).max(200) as u8,
                    (60 + r - 20).clamp(30, 100) as u8,
                    10,
                )
            }
            Element::Plasma => {
                let r = (rand::random::<u8>() % 50) as i16;
                Color::Rgb(
                    (200 + r / 3).min(255) as u8,
                    (120i16 - r).max(50) as u8,
                    255,
                )
            }
            Element::Cloner => match &p.extra {
                Some(stored) => {
                    let c = stored.color();
                    if let Color::Rgb(cr, cg, cb) = c {
                        Color::Rgb(
                            (cr as u16 + 30).min(255) as u8,
                            (cg as u16 + 30).min(255) as u8,
                            (cb as u16 + 30).min(255) as u8,
                        )
                    } else {
                        c
                    }
                }
                None => Element::Cloner.color(),
            },
            Element::Spark if view_mode == ViewMode::Electric => Color::Rgb(255, 255, 220),
            Element::Wire if view_mode == ViewMode::Electric => {
                let h = sim.heat_at(x, y);
                if h > 0.3 {
                    Color::Rgb(255, 200, 120)
                } else {
                    Color::Rgb(180, 130, 40)
                }
            }
            _ => p.element.color(),
        },
        None => BG,
    }
}

#[allow(clippy::too_many_lines)]
fn render_sidebar(ui: &mut Context, s: &SidebarState) {
    ui.text("POWDER TOY").bold().fg(Color::Cyan);
    ui.text("SLT Edition").dim();
    ui.separator();

    if s.search_mode {
        ui.row(|ui| {
            ui.text("/").dim();
            ui.text(format!(" {}", s.search_query))
                .bold()
                .fg(Color::Yellow);
        });
    } else {
        let cat_name = CATEGORY_NAMES[s.cat_idx];
        ui.row(|ui| {
            ui.text("\u{25C0} ").dim();
            ui.text(cat_name).bold().fg(Color::Yellow);
            ui.spacer();
            ui.text(" \u{25B6}").dim();
        });
    }
    ui.separator();

    if s.displayed_elems.is_empty() {
        ui.text("No matches").dim();
    }

    for (i, (_, elem)) in s.displayed_elems.iter().enumerate() {
        let key = if !s.search_mode && i < 9 {
            format!("{}", i + 1)
        } else {
            " ".to_string()
        };

        if i == s.elem_idx && !s.displayed_elems.is_empty() {
            let label = format!("\u{25B8}{} {:<8}", key, elem.name());
            ui.styled(
                &label,
                Style::new().fg(Color::Black).bg(elem.color()).bold(),
            );
        } else {
            let label = format!(" {} {:<8}", key, elem.name());
            ui.row(|ui| {
                ui.text(&label).fg(elem.color());
                ui.spacer();
                ui.styled("\u{2588}\u{2588}", Style::new().fg(elem.color()));
            });
        }
    }

    let info_elem = s.hovered_elem.unwrap_or(s.selected);
    ui.row(|ui| {
        ui.text(format!("D:{:.2}", info_elem.density())).dim();
        ui.text(" | ").dim();
        ui.text(behavior_label(info_elem.behavior())).dim();
        ui.text(" | ").dim();
        ui.text(if info_elem.flammable() {
            "Flam:Y"
        } else {
            "Flam:N"
        })
        .dim();
    });

    ui.spacer();
    ui.separator();

    ui.row(|ui| {
        ui.styled("\u{2588}\u{2588}", Style::new().fg(s.selected.color()));
        ui.text(format!(" {}", s.selected.name())).bold();
    });

    let bs = s.brush_size;
    let dots: String = (0..bs).map(|_| '\u{25CF}').collect::<String>()
        + &(0..(10 - bs).min(10))
            .map(|_| '\u{25CB}')
            .collect::<String>();
    ui.row(|ui| {
        ui.text("Brush ").dim();
        ui.text(&dots).fg(Color::Cyan);
        ui.text(format!(" {bs}")).bold();
    });
    ui.row(|ui| {
        ui.text("Shape").dim();
        ui.spacer();
        ui.text(s.brush_shape.label()).bold().fg(Color::Yellow);
    });

    if s.eraser {
        ui.text("\u{25A0} ERASER").bold().fg(Color::Red);
    }
    if let Some(mode) = s.stamp_mode.label() {
        ui.text(format!("\u{25A0} {mode}"))
            .bold()
            .fg(Color::Magenta);
    }
    if s.sim.paused {
        ui.text("\u{25A0} PAUSED").bold().fg(Color::Yellow);
    }

    ui.separator();

    ui.row(|ui| {
        ui.text("FPS").dim();
        ui.spacer();
        ui.text(format!("{}", s.fps)).bold().fg(Color::Green);
    });
    ui.row(|ui| {
        ui.text("TPS").dim();
        ui.spacer();
        ui.text(format!("{}", s.tps)).bold().fg(Color::Cyan);
    });
    ui.row(|ui| {
        ui.text("View").dim();
        ui.spacer();
        ui.text(s.view_mode.label()).bold().fg(Color::Yellow);
    });
    ui.row(|ui| {
        ui.text("Zoom").dim();
        ui.spacer();
        ui.text(format!("{}x", s.zoom_level))
            .bold()
            .fg(Color::Yellow);
    });
    ui.row(|ui| {
        ui.text("Pan").dim();
        ui.spacer();
        ui.text(format!("{},{}", s.viewport_x, s.viewport_y))
            .bold()
            .fg(Color::Cyan);
    });
    ui.row(|ui| {
        ui.text("Gravity").dim();
        ui.spacer();
        ui.text(s.sim.gravity_mode.label()).bold().fg(Color::Yellow);
    });
    ui.row(|ui| {
        ui.text("Speed").dim();
        ui.spacer();
        ui.text(format!("{}x", s.sim.sim_speed))
            .bold()
            .fg(Color::Cyan);
    });
    ui.row(|ui| {
        ui.text("Slot").dim();
        ui.spacer();
        ui.text(format!("{}", s.save_slot)).bold().fg(Color::Yellow);
    });
    ui.row(|ui| {
        ui.text("Grid").dim();
        ui.spacer();
        ui.text(if s.grid_overlay { "On" } else { "Off" })
            .bold()
            .fg(Color::Yellow);
    });
    ui.row(|ui| {
        ui.text("Particles").dim();
        ui.spacer();
        ui.text(format!("{}", s.count)).bold().fg(Color::Cyan);
    });
    ui.row(|ui| {
        ui.text("Tick").dim();
        ui.spacer();
        ui.text(format!("{}", s.sim.tick)).bold();
    });
    ui.row(|ui| {
        ui.text("Press").dim();
        ui.spacer();
        ui.text(format!(
            "{:.2}/{:.2}",
            s.pressure_stats.0, s.pressure_stats.1
        ))
        .bold();
    });
    ui.row(|ui| {
        ui.text("Heat").dim();
        ui.spacer();
        ui.text(format!("{:.2}/{:.2}", s.heat_stats.0, s.heat_stats.1))
            .bold();
    });
    ui.row(|ui| {
        ui.text("Electric").dim();
        ui.spacer();
        ui.text(format!("{}", s.electric_activity)).bold();
    });
    ui.row(|ui| {
        ui.text("Flow").dim();
        ui.spacer();
        ui.text(format!(
            "{:.2}/{:.2}",
            s.velocity_stats.0, s.velocity_stats.1
        ))
        .bold();
    });

    if !s.elem_counts.is_empty() {
        ui.separator();
        ui.text("Breakdown").dim();
        for (elem, cnt) in s.elem_counts.iter().take(6) {
            ui.row(|ui| {
                ui.styled("\u{2588}\u{2588}", Style::new().fg(elem.color()));
                ui.text(format!(" {:<7}", elem.name())).dim();
                ui.spacer();
                ui.text(format!("{cnt}")).bold();
            });
        }
    }

    if s.history.len() > 2 {
        ui.separator();
        ui.text("History").dim();
        ui.sparkline(s.history, 20);
    }

    ui.separator();
    ui.text("Status").dim();
    ui.text(s.status).fg(Color::Green);
}

fn append_search_input(ui: &mut Context, search_query: &mut String) {
    if ui.key_code(KeyCode::Backspace) {
        search_query.pop();
    }
    for ch in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 _-.".chars() {
        if ui.key(ch) {
            search_query.push(ch);
        }
    }
}

fn save_path_for_slot(slot: usize) -> String {
    format!("{SAVE_DIR}/slot{slot}.ptslt")
}

fn collect_displayed_elements(
    cat_idx: usize,
    search_mode: bool,
    search_query: &str,
) -> Vec<(usize, Element)> {
    if !search_mode {
        return elements_in_category(cat_idx)
            .iter()
            .map(|elem| (cat_idx, *elem))
            .collect();
    }

    let query = search_query.to_ascii_lowercase();
    let mut out = Vec::new();
    for category in 0..NUM_CATEGORIES {
        for elem in elements_in_category(category) {
            let name = elem.name().to_ascii_lowercase();
            if query.is_empty() || name.contains(&query) {
                out.push((category, *elem));
            }
        }
    }
    out
}

fn mix_color(base: Color, blend: Color, amount: f32) -> Color {
    match (base, blend) {
        (Color::Rgb(br, bg, bb), Color::Rgb(rr, rg, rb)) => {
            let t = amount.clamp(0.0, 1.0);
            let mix = |a: u8, b: u8| -> u8 {
                let af = f32::from(a);
                let bf = f32::from(b);
                (af + (bf - af) * t).round().clamp(0.0, 255.0) as u8
            };
            Color::Rgb(mix(br, rr), mix(bg, rg), mix(bb, rb))
        }
        _ => base,
    }
}

fn on_brush_outline(
    x: usize,
    y: usize,
    cx: usize,
    cy: usize,
    radius: usize,
    shape: BrushShape,
) -> bool {
    let xi = x as i32;
    let yi = y as i32;
    let cxi = cx as i32;
    let cyi = cy as i32;
    let ri = radius as i32;
    let dx = xi - cxi;
    let dy = yi - cyi;

    match shape {
        BrushShape::Circle => {
            let d2 = dx * dx + dy * dy;
            let outer = ri * ri;
            let inner = (ri.saturating_sub(1)) * (ri.saturating_sub(1));
            d2 <= outer && d2 >= inner
        }
        BrushShape::Square => {
            dx.abs() <= ri && dy.abs() <= ri && (dx.abs() == ri || dy.abs() == ri)
        }
        BrushShape::Line => {
            let half_thickness = (ri / 3).max(0);
            dx.abs() <= ri
                && dy.abs() <= half_thickness
                && (dx.abs() == ri || dy.abs() == half_thickness)
        }
    }
}

fn write_screenshot(sim: &Simulation, path: &str) -> std::io::Result<()> {
    let mut out = String::new();
    out.push_str(&format!(
        "tick={} size={}x{}\n",
        sim.tick, sim.width, sim.height
    ));
    for y in 0..sim.height {
        for x in 0..sim.width {
            let token = sim
                .get(x, y)
                .as_ref()
                .map_or("..".to_string(), |p| element_token(p.element));
            out.push_str(&token);
            if x + 1 < sim.width {
                out.push(' ');
            }
        }
        out.push('\n');
    }
    fs::write(path, out)
}

fn element_token(element: Element) -> String {
    let mut token = String::new();
    for ch in element.name().chars() {
        if ch.is_ascii_alphabetic() {
            token.push(ch.to_ascii_uppercase());
            if token.len() == 2 {
                break;
            }
        }
    }
    if token.is_empty() {
        String::from("??")
    } else if token.len() == 1 {
        format!("{token}_")
    } else {
        token
    }
}

fn behavior_label(behavior: Behavior) -> &'static str {
    match behavior {
        Behavior::Powder => "Powder",
        Behavior::Liquid => "Liquid",
        Behavior::Gas => "Gas",
        Behavior::Solid => "Solid",
    }
}

fn remember_state(
    sim: &Simulation,
    undo_stack: &mut Vec<Simulation>,
    redo_stack: &mut Vec<Simulation>,
) {
    undo_stack.push(sim.clone());
    if undo_stack.len() > HISTORY_LIMIT {
        undo_stack.remove(0);
    }
    redo_stack.clear();
}

fn apply_brush(
    sim: &mut Simulation,
    cx: usize,
    cy: usize,
    element: Element,
    radius: usize,
    shape: BrushShape,
    erase: bool,
) {
    let ri = radius as i32;

    let mut paint = |x: i32, y: i32| {
        if !sim.in_bounds(x, y) {
            return;
        }
        if erase {
            sim.erase(x as usize, y as usize);
        } else {
            sim.place_brush(x as usize, y as usize, element, 1);
        }
    };

    match shape {
        BrushShape::Circle => {
            if erase {
                sim.erase_brush(cx, cy, radius);
            } else {
                sim.place_brush(cx, cy, element, radius);
            }
        }
        BrushShape::Square => {
            for dy in -ri..=ri {
                for dx in -ri..=ri {
                    paint(cx as i32 + dx, cy as i32 + dy);
                }
            }
        }
        BrushShape::Line => {
            let half_thickness = (ri / 3).max(0);
            for dx in -ri..=ri {
                for dy in -half_thickness..=half_thickness {
                    paint(cx as i32 + dx, cy as i32 + dy);
                }
            }
        }
    }
}

fn setup_demo(sim: &mut Simulation) {
    let w = sim.width;
    let h = sim.height;
    let mid = w / 2;
    let floor = h - 1;

    // ═══ GROUND + BOUNDARY WALLS ═══
    for x in 0..w {
        sim.place_brush(x, floor, Element::Wall, 1);
        sim.place_brush(x, floor - 1, Element::Stone, 1);
    }
    for y in 0..h {
        sim.place_brush(0, y, Element::Wall, 1);
        sim.place_brush(w - 1, y, Element::Wall, 1);
    }

    // ═══ ZONE A (left): WATERFALL + SAND HOURGLASS ═══
    // Elevated stone basin with water
    let basin_y = h / 5;
    for x in 4..28 {
        sim.place_brush(x, basin_y + 8, Element::Stone, 1);
    }
    for y in basin_y..(basin_y + 8) {
        sim.place_brush(4, y, Element::Stone, 1);
        sim.place_brush(27, y, Element::Stone, 1);
    }
    // Water in basin
    for x in 5..27 {
        for y in (basin_y + 1)..(basin_y + 7) {
            sim.place_brush(x, y, Element::Water, 1);
        }
    }
    // Drain hole in basin → waterfall
    sim.erase(15, basin_y + 8);
    sim.erase(16, basin_y + 8);

    // Sand hourglass above basin
    for x in 10..22 {
        for y in 1..4 {
            sim.place_brush(x, y, Element::Sand, 1);
        }
    }

    // Catch pool at bottom-left
    for x in 2..35 {
        sim.place_brush(x, floor - 10, Element::Glass, 1);
    }
    for y in (floor - 10)..floor {
        sim.place_brush(2, y, Element::Glass, 1);
        sim.place_brush(34, y, Element::Glass, 1);
    }
    // Oil layer floating on future water pool
    for x in 3..34 {
        sim.place_brush(x, floor - 9, Element::Oil, 1);
    }

    // ═══ ZONE B (center-left): VOLCANO ═══
    let vx = mid - 15;
    let vy = floor - 2;
    // Mountain shape
    for layer in 0i32..12 {
        let half = 12 - layer;
        let y = vy - layer as usize;
        for dx in -half..=half {
            let x = vx as i32 + dx;
            if x >= 0 && (x as usize) < w {
                sim.place_brush(x as usize, y, Element::Stone, 1);
            }
        }
    }
    // Lava chamber inside
    for layer in 1i32..8 {
        let half = (8 - layer).min(3);
        let y = vy - layer as usize;
        for dx in -half..=half {
            let x = vx as i32 + dx;
            if x >= 0 && (x as usize) < w {
                sim.place_brush(x as usize, y, Element::Lava, 1);
            }
        }
    }
    // Vent at top
    sim.erase(vx, vy - 12);
    sim.erase(vx, vy - 11);

    // ═══ ZONE C (center): ELECTRICAL CIRCUIT ═══
    let cx = mid;
    let cy = h / 3;
    // Battery
    sim.place_brush(cx - 12, cy, Element::Battery, 1);
    // Wire path: horizontal → down → horizontal → up (loop shape)
    for x in (cx - 11)..=(cx + 5) {
        sim.place_brush(x, cy, Element::Wire, 1);
    }
    for y in cy..(cy + 8) {
        sim.place_brush(cx + 5, y, Element::Wire, 1);
    }
    for x in (cx - 5)..=(cx + 5) {
        sim.place_brush(x, cy + 8, Element::Wire, 1);
    }
    for y in (cy + 2)..(cy + 8) {
        sim.place_brush(cx - 5, y, Element::Wire, 1);
    }
    // Fuse line extending from circuit → triggers chain reaction
    for x in (cx + 6)..(cx + 14) {
        sim.place_brush(x, cy + 4, Element::Fuse, 1);
    }
    // Gunpowder cache at end of fuse
    for x in (cx + 14)..(cx + 18) {
        for y in (cy + 2)..(cy + 7) {
            sim.place_brush(x, y, Element::Gunpowder, 1);
        }
    }

    // ═══ ZONE D (center-right): GAS CHAMBER ═══
    let gx = mid + 20;
    let gy = h / 4;
    // Sealed glass chamber
    for x in gx..(gx + 20).min(w - 2) {
        sim.place_brush(x, gy, Element::Glass, 1);
        sim.place_brush(x, gy + 14, Element::Glass, 1);
    }
    for y in gy..(gy + 15) {
        sim.place_brush(gx, y, Element::Glass, 1);
        sim.place_brush((gx + 19).min(w - 2), y, Element::Glass, 1);
    }
    // Hydrogen top half
    for x in (gx + 1)..(gx + 19).min(w - 2) {
        for y in (gy + 1)..(gy + 7) {
            sim.place_brush(x, y, Element::Hydrogen, 1);
        }
    }
    // Oxygen bottom half
    for x in (gx + 1)..(gx + 19).min(w - 2) {
        for y in (gy + 8)..(gy + 14) {
            sim.place_brush(x, y, Element::Oxygen, 1);
        }
    }
    // Spark igniter on the side (timed via wire)
    sim.place_brush(gx + 1, gy + 7, Element::Fire, 1);

    // ═══ ZONE E (right): ICE + CRYO LAB ═══
    let lx = w - 30;
    let ly = floor - 20;
    for x in lx..(lx + 25).min(w - 1) {
        sim.place_brush(x, ly, Element::Wall, 1);
        sim.place_brush(x, ly + 18, Element::Wall, 1);
    }
    for y in ly..(ly + 19) {
        sim.place_brush(lx, y, Element::Wall, 1);
        sim.place_brush((lx + 24).min(w - 2), y, Element::Wall, 1);
    }
    // Water pool
    for x in (lx + 1)..(lx + 24).min(w - 2) {
        for y in (ly + 10)..(ly + 18) {
            sim.place_brush(x, y, Element::Water, 1);
        }
    }
    // Cryo source at top
    for x in (lx + 8)..(lx + 16).min(w - 2) {
        sim.place_brush(x, ly + 1, Element::Cryo, 1);
        sim.place_brush(x, ly + 2, Element::Cryo, 1);
    }
    // Heater on one side for contrast
    sim.place_brush(lx + 2, ly + 5, Element::Heater, 1);
    sim.place_brush(lx + 3, ly + 5, Element::Heater, 1);

    // ═══ ZONE F (bottom-center): ACID PIT ═══
    let ax = mid - 8;
    let ay = floor - 6;
    for x in ax..(ax + 16) {
        sim.place_brush(x, ay, Element::Metal, 1);
    }
    for y in ay..(floor - 1) {
        sim.place_brush(ax, y, Element::Metal, 1);
        sim.place_brush(ax + 15, y, Element::Metal, 1);
    }
    for x in (ax + 1)..(ax + 15) {
        for y in (ay + 1)..(floor - 1) {
            sim.place_brush(x, y, Element::Acid, 1);
        }
    }
    // Wood plank dropping into acid
    for x in (ax + 4)..(ax + 12) {
        sim.place_brush(x, ay - 2, Element::Wood, 1);
        sim.place_brush(x, ay - 1, Element::Wood, 1);
    }

    // ═══ ZONE G (top-right): CLONER FOUNTAIN ═══
    let fx = w - 15;
    let fy = 3;
    sim.place_brush(fx, fy, Element::Cloner, 1);
    // Drop one sand grain onto cloner to activate
    sim.place_brush(fx, fy - 1, Element::Sand, 1);
    // Walls to channel the fountain
    for y in fy..(fy + 12) {
        sim.place_brush(fx - 3, y, Element::Wall, 1);
        sim.place_brush(fx + 3, y, Element::Wall, 1);
    }

    // ═══ ZONE H (scattered): DECORATIVE SNOW ═══
    use rand::Rng;
    let mut rng = rand::thread_rng();
    for _ in 0..40 {
        let x = rng.gen_range(2..w - 2);
        let y = rng.gen_range(0..4);
        sim.place_brush(x, y, Element::Snow, 1);
    }
}
