use crate::elements::{Behavior, Element};
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GravityMode {
    Normal,
    Zero,
    Inverted,
}

impl GravityMode {
    pub fn next(self) -> Self {
        match self {
            GravityMode::Normal => GravityMode::Zero,
            GravityMode::Zero => GravityMode::Inverted,
            GravityMode::Inverted => GravityMode::Normal,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            GravityMode::Normal => "Normal",
            GravityMode::Zero => "Zero",
            GravityMode::Inverted => "Inverted",
        }
    }

    pub fn direction(self) -> i32 {
        match self {
            GravityMode::Normal => 1,
            GravityMode::Zero => 0,
            GravityMode::Inverted => -1,
        }
    }
}

#[derive(Clone)]
pub struct Particle {
    pub element: Element,
    pub lifetime: u16,
    pub extra: Option<Element>,
    moved: bool,
}

impl Particle {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            lifetime: element.lifetime().unwrap_or(0),
            extra: None,
            moved: false,
        }
    }
}

#[derive(Clone)]
pub struct Simulation {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Option<Particle>>,
    pub pressure: Vec<f32>,
    pub heat: Vec<f32>,
    pub vel_x: Vec<f32>,
    pub vel_y: Vec<f32>,
    pub paused: bool,
    pub tick: u64,
    pub gravity_mode: GravityMode,
    pub sim_speed: u8,
}

impl Simulation {
    #[inline]
    fn blocks_field_propagation(element: Element) -> bool {
        matches!(element, Element::Wall | Element::Ttan | Element::Dmnd)
    }

    #[inline]
    fn is_field_blocker_idx(&self, idx: usize) -> bool {
        self.grid[idx]
            .as_ref()
            .is_some_and(|p| Self::blocks_field_propagation(p.element))
    }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![None; width * height],
            pressure: vec![0.0; width * height],
            heat: vec![0.22; width * height],
            vel_x: vec![0.0; width * height],
            vel_y: vec![0.0; width * height],
            paused: false,
            tick: 0,
            gravity_mode: GravityMode::Normal,
            sim_speed: 1,
        }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

    pub fn get(&self, x: usize, y: usize) -> &Option<Particle> {
        &self.grid[self.idx(x, y)]
    }

    pub fn pressure_at(&self, x: usize, y: usize) -> f32 {
        self.pressure[self.idx(x, y)]
    }

    pub fn heat_at(&self, x: usize, y: usize) -> f32 {
        self.heat[self.idx(x, y)]
    }

    pub fn velocity_at(&self, x: usize, y: usize) -> (f32, f32) {
        let idx = self.idx(x, y);
        (self.vel_x[idx], self.vel_y[idx])
    }

    pub fn pressure_stats(&self) -> (f32, f32) {
        let mut min_v = f32::INFINITY;
        let mut max_v = f32::NEG_INFINITY;
        for v in &self.pressure {
            min_v = min_v.min(*v);
            max_v = max_v.max(*v);
        }
        (min_v, max_v)
    }

    pub fn heat_stats(&self) -> (f32, f32) {
        let mut min_v = f32::INFINITY;
        let mut max_v = f32::NEG_INFINITY;
        for v in &self.heat {
            min_v = min_v.min(*v);
            max_v = max_v.max(*v);
        }
        (min_v, max_v)
    }

    pub fn velocity_stats(&self) -> (f32, f32) {
        let mut avg = 0.0f32;
        let mut max_v = 0.0f32;
        for i in 0..self.vel_x.len() {
            let speed = (self.vel_x[i] * self.vel_x[i] + self.vel_y[i] * self.vel_y[i]).sqrt();
            avg += speed;
            if speed > max_v {
                max_v = speed;
            }
        }
        if self.vel_x.is_empty() {
            (0.0, 0.0)
        } else {
            (avg / self.vel_x.len() as f32, max_v)
        }
    }

    fn element_at(&self, x: i32, y: i32) -> Option<Element> {
        if self.in_bounds(x, y) {
            self.grid[y as usize * self.width + x as usize]
                .as_ref()
                .map(|p| p.element)
        } else {
            None
        }
    }

    fn is_dlay_element(element: Element) -> bool {
        element.name().eq_ignore_ascii_case("DLAY")
    }

    fn is_inst_element(element: Element) -> bool {
        element.name().eq_ignore_ascii_case("INST")
    }

    pub fn erase(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let idx = self.idx(x, y);
            if self.grid[idx]
                .as_ref()
                .is_none_or(|p| p.element != Element::Wall)
            {
                self.grid[idx] = None;
            }
        }
    }

    pub fn clear(&mut self) {
        self.grid.fill(None);
        self.pressure.fill(0.0);
        self.heat.fill(0.22);
        self.vel_x.fill(0.0);
        self.vel_y.fill(0.0);
        self.tick = 0;
        self.gravity_mode = GravityMode::Normal;
        self.sim_speed = 1;
    }

    pub fn particle_count(&self) -> usize {
        self.grid.iter().filter(|c| c.is_some()).count()
    }

    pub fn element_counts(&self) -> Vec<(Element, usize)> {
        let mut map: HashMap<Element, usize> = HashMap::new();
        for p in self.grid.iter().flatten() {
            *map.entry(p.element).or_insert(0) += 1;
        }
        let mut sorted: Vec<_> = map.into_iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(&a.1));
        sorted
    }

    pub fn electric_activity(&self) -> usize {
        let mut active = 0usize;
        for (i, cell) in self.grid.iter().enumerate() {
            if let Some(p) = cell {
                if p.element == Element::Spark || (p.element.is_conductor() && self.heat[i] > 0.24)
                {
                    active += 1;
                }
            }
        }
        active
    }

    pub fn update(&mut self) {
        if !self.paused {
            for _ in 0..self.sim_speed.max(1) {
                self.step_once();
            }
        }
    }

    pub fn step_once(&mut self) {
        self.tick += 1;

        let mut rng = rand::thread_rng();

        for p in self.grid.iter_mut().flatten() {
            p.moved = false;
        }

        match self.gravity_mode {
            GravityMode::Inverted => {
                for y in 0..self.height {
                    let left_to_right: bool = rng.gen();
                    if left_to_right {
                        for x in 0..self.width {
                            self.update_particle(x, y);
                        }
                    } else {
                        for x in (0..self.width).rev() {
                            self.update_particle(x, y);
                        }
                    }
                }
            }
            _ => {
                for y in (0..self.height).rev() {
                    let left_to_right: bool = rng.gen();
                    if left_to_right {
                        for x in 0..self.width {
                            self.update_particle(x, y);
                        }
                    } else {
                        for x in (0..self.width).rev() {
                            self.update_particle(x, y);
                        }
                    }
                }
            }
        }

        self.process_reactions();
        self.update_fields();
        self.apply_thermal_phase_changes();
    }

    pub fn serialize(&self) -> String {
        let mut out = String::new();
        out.push_str("PTSLT3\n");
        out.push_str(&format!(
            "{} {} {} {} {}\n",
            self.width,
            self.height,
            self.tick,
            self.gravity_mode as u8,
            self.sim_speed.max(1)
        ));

        for cell in &self.grid {
            match cell {
                Some(p) => {
                    let extra = p.extra.map(|e| e.id()).unwrap_or(u16::MAX);
                    out.push_str(&format!("{}:{}:{} ", p.element.id(), extra, p.lifetime));
                }
                None => out.push_str("-1 "),
            }
        }
        out.push('\n');

        for v in &self.pressure {
            out.push_str(&format!("{v:.6} "));
        }
        out.push('\n');

        for v in &self.heat {
            out.push_str(&format!("{v:.6} "));
        }
        out.push('\n');

        for v in &self.vel_x {
            out.push_str(&format!("{v:.6} "));
        }
        out.push('\n');

        for v in &self.vel_y {
            out.push_str(&format!("{v:.6} "));
        }
        out.push('\n');
        out
    }

    pub fn deserialize(src: &str) -> Option<Self> {
        let mut lines = src.lines();
        let magic = lines.next()?;
        let has_velocity = magic == "PTSLT2" || magic == "PTSLT3";
        if !(magic == "PTSLT1" || magic == "PTSLT2" || magic == "PTSLT3") {
            return None;
        }
        let header = lines.next()?;
        let mut h = header.split_whitespace();
        let width: usize = h.next()?.parse().ok()?;
        let height: usize = h.next()?.parse().ok()?;
        let tick: u64 = h.next()?.parse().ok()?;
        let gravity_mode = if magic == "PTSLT3" {
            match h.next().and_then(|v| v.parse::<u8>().ok()).unwrap_or(0) {
                0 => GravityMode::Normal,
                1 => GravityMode::Zero,
                2 => GravityMode::Inverted,
                _ => GravityMode::Normal,
            }
        } else {
            GravityMode::Normal
        };
        let sim_speed = if magic == "PTSLT3" {
            match h.next().and_then(|v| v.parse::<u8>().ok()).unwrap_or(1) {
                2 => 2,
                4 => 4,
                _ => 1,
            }
        } else {
            1
        };
        let count = width.checked_mul(height)?;

        let grid_line = lines.next()?;
        let pressure_line = lines.next()?;
        let heat_line = lines.next()?;
        let vel_x_line = if has_velocity { lines.next() } else { None };
        let vel_y_line = if has_velocity { lines.next() } else { None };

        let mut grid = Vec::with_capacity(count);
        for token in grid_line.split_whitespace().take(count) {
            if token == "-1" {
                grid.push(None);
            } else {
                let mut parts = token.split(':');
                let eid: u16 = parts.next()?.parse().ok()?;
                let xid: u16 = parts.next()?.parse().ok()?;
                let lifetime: u16 = parts.next()?.parse().ok()?;
                let element = Element::from_id(eid)?;
                let extra = if xid == u16::MAX {
                    None
                } else {
                    Element::from_id(xid)
                };
                grid.push(Some(Particle {
                    element,
                    lifetime,
                    extra,
                    moved: false,
                }));
            }
        }
        if grid.len() != count {
            return None;
        }

        let pressure: Vec<f32> = pressure_line
            .split_whitespace()
            .take(count)
            .map(|v| v.parse::<f32>().ok())
            .collect::<Option<Vec<_>>>()?;
        let heat: Vec<f32> = heat_line
            .split_whitespace()
            .take(count)
            .map(|v| v.parse::<f32>().ok())
            .collect::<Option<Vec<_>>>()?;

        if pressure.len() != count || heat.len() != count {
            return None;
        }

        let vel_x: Vec<f32> = if let Some(line) = vel_x_line {
            let v = line
                .split_whitespace()
                .take(count)
                .map(|n| n.parse::<f32>().ok())
                .collect::<Option<Vec<_>>>()?;
            if v.len() != count {
                return None;
            }
            v
        } else {
            vec![0.0; count]
        };

        let vel_y: Vec<f32> = if let Some(line) = vel_y_line {
            let v = line
                .split_whitespace()
                .take(count)
                .map(|n| n.parse::<f32>().ok())
                .collect::<Option<Vec<_>>>()?;
            if v.len() != count {
                return None;
            }
            v
        } else {
            vec![0.0; count]
        };

        Some(Self {
            width,
            height,
            grid,
            pressure,
            heat,
            vel_x,
            vel_y,
            paused: false,
            tick,
            gravity_mode,
            sim_speed,
        })
    }

    fn update_fields(&mut self) {
        let mut next_pressure = self.pressure.clone();
        let mut next_heat = self.heat.clone();
        let mut next_vx = self.vel_x.clone();
        let mut next_vy = self.vel_y.clone();

        for y in 1..(self.height.saturating_sub(1)) {
            for x in 1..(self.width.saturating_sub(1)) {
                let idx = self.idx(x, y);
                let up = self.idx(x, y - 1);
                let down = self.idx(x, y + 1);
                let left = self.idx(x - 1, y);
                let right = self.idx(x + 1, y);

                if self.is_field_blocker_idx(idx) {
                    next_pressure[idx] = 0.0;
                    next_vx[idx] = 0.0;
                    next_vy[idx] = 0.0;
                    continue;
                }

                let mut p_sum = 0.0;
                let mut h_sum = 0.0;
                let mut contributors = 0.0;
                for nidx in [up, down, left, right] {
                    if self.is_field_blocker_idx(nidx) {
                        continue;
                    }
                    p_sum += self.pressure[nidx];
                    h_sum += self.heat[nidx];
                    contributors += 1.0;
                }

                let p_avg = if contributors > 0.0 {
                    p_sum / contributors
                } else {
                    self.pressure[idx]
                };
                let h_avg = if contributors > 0.0 {
                    h_sum / contributors
                } else {
                    self.heat[idx]
                };

                next_pressure[idx] = self.pressure[idx] * 0.92 + p_avg * 0.08;
                next_pressure[idx] *= 0.985;

                next_heat[idx] = self.heat[idx] * 0.96 + h_avg * 0.04;

                let left_p = if self.is_field_blocker_idx(left) {
                    self.pressure[idx]
                } else {
                    self.pressure[left]
                };
                let right_p = if self.is_field_blocker_idx(right) {
                    self.pressure[idx]
                } else {
                    self.pressure[right]
                };
                let up_p = if self.is_field_blocker_idx(up) {
                    self.pressure[idx]
                } else {
                    self.pressure[up]
                };
                let down_p = if self.is_field_blocker_idx(down) {
                    self.pressure[idx]
                } else {
                    self.pressure[down]
                };

                let grad_x = right_p - left_p;
                let grad_y = down_p - up_p;
                next_vx[idx] = self.vel_x[idx] * 0.94 - grad_x * 0.35;
                next_vy[idx] = self.vel_y[idx] * 0.94 - grad_y * 0.35;

                if let Some(p) = &self.grid[idx] {
                    match p.element {
                        Element::Lava => next_heat[idx] += 0.02,
                        Element::Fire => next_heat[idx] += 0.03,
                        Element::Cflm => next_heat[idx] += 0.02,
                        Element::Plasma => next_heat[idx] += 0.06,
                        Element::Heater => next_heat[idx] += 0.02,
                        Element::Cooler => next_heat[idx] -= 0.02,
                        Element::Cryo => next_heat[idx] -= 0.03,
                        Element::Spark => next_heat[idx] += 0.025,
                        Element::Sing => {
                            next_pressure[idx] = (next_pressure[idx] - 0.08).clamp(-1.0, 1.0)
                        }
                        _ => {}
                    }
                } else {
                    next_heat[idx] = next_heat[idx] * 0.98 + 0.22 * 0.02;
                }

                next_heat[idx] = next_heat[idx].clamp(0.0, 1.0);
                next_pressure[idx] = next_pressure[idx].clamp(-1.0, 1.0);
                next_vx[idx] = next_vx[idx].clamp(-1.5, 1.5);
                next_vy[idx] = next_vy[idx].clamp(-1.5, 1.5);
            }
        }

        self.pressure = next_pressure;
        self.heat = next_heat;
        self.vel_x = next_vx;
        self.vel_y = next_vy;
    }

    fn apply_thermal_phase_changes(&mut self) {
        for i in 0..self.grid.len() {
            let (element, heat) = match &self.grid[i] {
                Some(p) => (p.element, self.heat[i]),
                None => continue,
            };

            if let Some(next_elem) = element.thermal_phase(heat) {
                self.grid[i] = Some(Particle::new(next_elem));
                if next_elem == Element::Fire || next_elem == Element::Plasma {
                    self.pressure[i] = (self.pressure[i] + 0.14).clamp(-1.0, 1.0);
                }
            } else if let Some(t) = element.ignite_temp() {
                if heat > t {
                    self.grid[i] = Some(Particle::new(Element::Fire));
                    self.pressure[i] = (self.pressure[i] + 0.1).clamp(-1.0, 1.0);
                }
            }
        }
    }

    fn update_particle(&mut self, x: usize, y: usize) {
        let idx = self.idx(x, y);
        let particle = match &self.grid[idx] {
            Some(p) if !p.moved => p,
            _ => return,
        };

        let element = particle.element;

        if element == Element::Phot {
            self.update_phot(x, y);
            return;
        }
        if element == Element::Neut {
            self.update_neut(x, y);
            return;
        }
        if element == Element::Prot {
            self.update_prot(x, y);
            return;
        }
        if element == Element::Bizrg {
            self.update_bizrg(x, y);
            return;
        }
        if element == Element::WarpG {
            self.update_warpg(x, y);
            return;
        }

        if element.lifetime().is_some() && !Self::is_dlay_element(element) {
            let life = self.grid[idx].as_ref().unwrap().lifetime;
            if life == 0 {
                let mut rng = rand::thread_rng();
                self.grid[idx] = match element {
                    Element::Fire => Some(Particle::new(Element::Smoke)),
                    Element::Cflm => Some(Particle::new(Element::Smoke)),
                    Element::Plasma => {
                        if rng.gen_bool(0.3) {
                            Some(Particle::new(Element::Fire))
                        } else {
                            None
                        }
                    }
                    Element::Steam => {
                        if rng.gen_bool(0.25) {
                            Some(Particle::new(Element::Water))
                        } else {
                            None
                        }
                    }
                    Element::Bizrg => {
                        if rng.gen_bool(0.4) {
                            Some(Particle::new(Element::Smoke))
                        } else {
                            None
                        }
                    }
                    Element::WarpG => None,
                    Element::Prot => None,
                    Element::Sing => Some(Particle::new(Element::Fire)),
                    _ => None,
                };
                return;
            }
            self.grid[idx].as_mut().unwrap().lifetime -= 1;
        }

        match element.behavior() {
            Behavior::Powder => self.update_powder(x, y),
            Behavior::Liquid => self.update_liquid(x, y),
            Behavior::Gas => self.update_gas(x, y),
            Behavior::Solid => {}
        }
    }

    fn update_powder(&mut self, x: usize, y: usize) {
        let xi = x as i32;
        let yi = y as i32;
        let (d1, d2) = random_dirs();
        let g = self.gravity_mode.direction();

        if g == 0 {
            if self.try_move(x, y, xi + d1, yi) || self.try_move(x, y, xi + d2, yi) {
                return;
            }
            if self.try_move(x, y, xi + d1, yi + 1) || self.try_move(x, y, xi + d2, yi + 1) {
                return;
            }
            if self.try_move(x, y, xi + d1, yi - 1) || self.try_move(x, y, xi + d2, yi - 1) {
                return;
            }
            return;
        }

        if self.try_move(x, y, xi, yi + g) {
            return;
        }
        if self.try_move(x, y, xi + d1, yi + g) {
            return;
        }
        if self.try_move(x, y, xi + d2, yi + g) {}
    }

    fn update_liquid(&mut self, x: usize, y: usize) {
        let xi = x as i32;
        let yi = y as i32;
        let (d1, d2) = random_dirs();
        let g = self.gravity_mode.direction();

        let v_bias_y = self.vertical_velocity_bias(x, y);

        if g == 0 {
            if self.try_move(x, y, xi + d1, yi) || self.try_move(x, y, xi + d2, yi) {
                return;
            }
            if self.try_move(x, y, xi, yi + 1) || self.try_move(x, y, xi, yi - 1) {
                return;
            }
            if self.try_move(x, y, xi, yi + v_bias_y) {
                return;
            }
        } else if self.try_move(x, y, xi, yi + g + v_bias_y) || self.try_move(x, y, xi, yi + g) {
            return;
        }
        if self.try_move(x, y, xi + d1, yi + g) {
            return;
        }
        if self.try_move(x, y, xi + d2, yi + g) {
            return;
        }

        let disp = self.grid[self.idx(x, y)]
            .as_ref()
            .map(|p| p.element.dispersion())
            .unwrap_or(0);

        let pressure_bias = self.side_pressure_bias(x, y);

        for i in 1..=disp {
            if self.try_move(x, y, xi + pressure_bias * i, yi)
                || self.try_move(x, y, xi + d1 * i, yi)
            {
                return;
            }
        }
        for i in 1..=disp {
            if self.try_move(x, y, xi + d2 * i, yi)
                || self.try_move(x, y, xi - pressure_bias * i, yi)
            {
                return;
            }
        }
    }

    fn update_gas(&mut self, x: usize, y: usize) {
        let xi = x as i32;
        let yi = y as i32;
        let (d1, d2) = random_dirs();

        let v_bias_y = self.vertical_velocity_bias(x, y);

        if self.try_move(x, y, xi, yi - 1 + v_bias_y) || self.try_move(x, y, xi, yi - 1) {
            return;
        }
        if self.try_move(x, y, xi + d1, yi - 1) {
            return;
        }
        if self.try_move(x, y, xi + d2, yi - 1) {
            return;
        }
        let pressure_bias = self.side_pressure_bias(x, y);
        if self.try_move(x, y, xi + pressure_bias, yi) || self.try_move(x, y, xi + d1, yi) {
            return;
        }
        if self.try_move(x, y, xi + d2, yi) || self.try_move(x, y, xi - pressure_bias, yi) {
            return;
        }
        if rand::thread_rng().gen_bool(0.05) {
            self.try_move(x, y, xi, yi + 1);
        }
    }

    fn try_move(&mut self, fx: usize, fy: usize, tx: i32, ty: i32) -> bool {
        if !self.in_bounds(tx, ty) {
            return false;
        }
        let tx = tx as usize;
        let ty = ty as usize;
        let fi = self.idx(fx, fy);
        let ti = self.idx(tx, ty);

        if self.grid[ti].is_none() {
            self.grid[ti] = self.grid[fi].take();
            self.grid[ti].as_mut().unwrap().moved = true;
            self.pressure[ti] += self.pressure[fi] * 0.2;
            return true;
        }

        let from_density = self.grid[fi].as_ref().unwrap().element.density();
        let to = self.grid[ti].as_ref().unwrap();
        let to_density = to.element.density();
        let to_behavior = to.element.behavior();

        if from_density > to_density
            && matches!(to_behavior, Behavior::Liquid | Behavior::Gas)
            && !to.moved
        {
            self.grid.swap(fi, ti);
            self.grid[ti].as_mut().unwrap().moved = true;
            self.grid[fi].as_mut().unwrap().moved = true;
            return true;
        }

        false
    }

    fn phot_dir_token(dx: i32, dy: i32) -> Element {
        match (dx, dy) {
            (-1, 0) => Element::Brick,
            (0, -1) => Element::Cooler,
            (0, 1) => Element::Heater,
            _ => Element::Wall,
        }
    }

    fn phot_dir_from(extra: Option<Element>) -> (i32, i32) {
        match extra {
            Some(Element::Brick) => (-1, 0),
            Some(Element::Cooler) => (0, -1),
            Some(Element::Heater) => (0, 1),
            _ => (1, 0),
        }
    }

    fn update_phot(&mut self, x: usize, y: usize) {
        let idx = self.idx(x, y);
        let (dx, dy) = Self::phot_dir_from(self.grid[idx].as_ref().and_then(|p| p.extra));
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if !self.in_bounds(nx, ny) {
            self.grid[idx] = None;
            return;
        }

        let nidx = self.idx(nx as usize, ny as usize);
        if self.grid[nidx].is_none() {
            self.grid[nidx] = self.grid[idx].take();
            if let Some(p) = &mut self.grid[nidx] {
                p.moved = true;
                if p.extra.is_none() {
                    p.extra = Some(Self::phot_dir_token(dx, dy));
                }
            }
            self.heat[nidx] = (self.heat[nidx] + 0.006).clamp(0.0, 1.0);
            return;
        }

        if let Some(next) = self.grid[nidx].as_ref() {
            if next.element == Element::Glass || next.element == Element::Phot {
                self.grid[nidx] = self.grid[idx].take();
                if let Some(p) = &mut self.grid[nidx] {
                    p.moved = true;
                }
            } else {
                self.grid[idx] = None;
                self.pressure[nidx] = (self.pressure[nidx] + 0.04).clamp(-1.0, 1.0);
                self.heat[nidx] = (self.heat[nidx] + 0.02).clamp(0.0, 1.0);
            }
        }
    }

    fn update_neut(&mut self, x: usize, y: usize) {
        let idx = self.idx(x, y);
        let (dx, dy) = Self::phot_dir_from(self.grid[idx].as_ref().and_then(|p| p.extra));
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if !self.in_bounds(nx, ny) {
            self.grid[idx] = None;
            return;
        }

        let nidx = self.idx(nx as usize, ny as usize);
        let target = self.grid[nidx].as_ref().map(|p| p.element);
        if target.is_none() || target == Some(Element::Neut) || target == Some(Element::Phot) {
            self.grid[nidx] = self.grid[idx].take();
            if let Some(p) = &mut self.grid[nidx] {
                p.moved = true;
                if p.extra.is_none() {
                    p.extra = Some(Self::phot_dir_token(dx, dy));
                }
            }
            return;
        }

        if target == Some(Element::Uran) {
            self.grid[idx] = None;
            self.grid[nidx] = Some(Particle::new(Element::Fire));
            self.heat[nidx] = (self.heat[nidx] + 0.25).clamp(0.0, 1.0);
            self.pressure[nidx] = (self.pressure[nidx] + 0.18).clamp(-1.0, 1.0);
            let mut rng = rand::thread_rng();
            for (sdx, sdy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let sx = nx + sdx;
                let sy = ny + sdy;
                if self.in_bounds(sx, sy) {
                    let si = self.idx(sx as usize, sy as usize);
                    self.heat[si] = (self.heat[si] + 0.15).clamp(0.0, 1.0);
                    if self.grid[si].is_none() && rng.gen_bool(0.7) {
                        let mut p = Particle::new(Element::Neut);
                        p.extra = Some(Self::phot_dir_token(sdx, sdy));
                        self.grid[si] = Some(p);
                    }
                }
            }
            return;
        }

        if target.is_some_and(|e| !e.is_special()) {
            self.grid[nidx] = self.grid[idx].take();
            if let Some(p) = &mut self.grid[nidx] {
                p.moved = true;
            }
        } else {
            self.grid[idx] = None;
        }
    }

    fn update_prot(&mut self, x: usize, y: usize) {
        let idx = self.idx(x, y);
        let (dx, dy) = Self::phot_dir_from(self.grid[idx].as_ref().and_then(|p| p.extra));
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;

        if !self.in_bounds(nx, ny) {
            self.grid[idx] = None;
            return;
        }

        let nidx = self.idx(nx as usize, ny as usize);
        if self.grid[nidx].is_none() {
            self.grid[nidx] = self.grid[idx].take();
            if let Some(p) = &mut self.grid[nidx] {
                p.moved = true;
                if p.extra.is_none() {
                    p.extra = Some(Self::phot_dir_token(dx, dy));
                }
            }
            self.heat[nidx] = (self.heat[nidx] + 0.01).clamp(0.0, 1.0);
            return;
        }

        if let Some(target) = self.grid[nidx].as_ref().map(|p| p.element) {
            if target == Element::Phot || target == Element::Prot {
                self.grid[nidx] = self.grid[idx].take();
                if let Some(p) = &mut self.grid[nidx] {
                    p.moved = true;
                }
                return;
            }
            if target.is_conductor() {
                self.grid[nidx] = Some(Particle::new(Element::Spark));
                self.heat[nidx] = (self.heat[nidx] + 0.06).clamp(0.0, 1.0);
                self.grid[idx] = None;
                return;
            }
            if target.is_special() {
                self.grid[idx] = None;
                return;
            }
            self.grid[nidx] = Some(Particle::new(Element::Fire));
            self.pressure[nidx] = (self.pressure[nidx] + 0.1).clamp(-1.0, 1.0);
            self.heat[nidx] = (self.heat[nidx] + 0.1).clamp(0.0, 1.0);
            self.grid[idx] = None;
        }
    }

    fn update_bizrg(&mut self, x: usize, y: usize) {
        let xi = x as i32;
        let yi = y as i32;
        let mut rng = rand::thread_rng();
        for _ in 0..3 {
            let dx = rng.gen_range(-1..=1);
            let dy = rng.gen_range(-1..=1);
            if dx == 0 && dy == 0 {
                continue;
            }
            if self.try_move(x, y, xi + dx, yi + dy) {
                return;
            }
        }
        self.update_gas(x, y);
    }

    fn update_warpg(&mut self, x: usize, y: usize) {
        let xi = x as i32;
        let yi = y as i32;
        self.update_gas(x, y);
        let mut rng = rand::thread_rng();
        if !rng.gen_bool(0.12) {
            return;
        }

        let idx = self.idx(x, y);
        if self.grid[idx]
            .as_ref()
            .is_none_or(|p| p.element != Element::WarpG)
        {
            return;
        }

        let mut teleported = false;
        for _ in 0..5 {
            let nx = xi + rng.gen_range(-6..=6);
            let ny = yi + rng.gen_range(-4..=4);
            if !self.in_bounds(nx, ny) {
                continue;
            }
            let nidx = self.idx(nx as usize, ny as usize);
            if self.grid[nidx].is_none() {
                self.grid[nidx] = self.grid[idx].take();
                if let Some(p) = &mut self.grid[nidx] {
                    p.moved = true;
                }
                self.pressure[nidx] = (self.pressure[nidx] + 0.05).clamp(-1.0, 1.0);
                teleported = true;
                break;
            }
        }
        if !teleported {
            if let Some(p) = &mut self.grid[idx] {
                p.moved = true;
            }
        }
    }

    fn process_reactions(&mut self) {
        let mut rng = rand::thread_rng();
        let mut changes: Vec<(usize, usize, Option<Element>)> = Vec::new();
        let mut spawns: Vec<(usize, usize, Element)> = Vec::new();
        let mut clone_updates: Vec<(usize, usize, Element)> = Vec::new();
        let mut switch_updates: Vec<(usize, usize, bool)> = Vec::new();
        let mut pipe_item_updates: Vec<(usize, usize, Option<Element>)> = Vec::new();
        let mut dlay_lifetime_updates: Vec<(usize, usize, u16)> = Vec::new();
        let mut inst_sources: Vec<(usize, usize)> = Vec::new();

        let mut portal_outs: Vec<(usize, usize)> = Vec::new();
        let mut wifi_a: Vec<(usize, usize)> = Vec::new();
        let mut wifi_b: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(p) = &self.grid[self.idx(x, y)] {
                    match p.element {
                        Element::PortalOut => portal_outs.push((x, y)),
                        Element::WifiA => wifi_a.push((x, y)),
                        Element::WifiB => wifi_b.push((x, y)),
                        _ => {}
                    }
                }
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.idx(x, y);
                let (element, extra) = match &self.grid[idx] {
                    Some(p) => (p.element, p.extra),
                    None => continue,
                };

                let neighbors: [(i32, i32); 4] = [
                    (x as i32, y as i32 - 1),
                    (x as i32, y as i32 + 1),
                    (x as i32 - 1, y as i32),
                    (x as i32 + 1, y as i32),
                ];

                let neighbors8: [(i32, i32); 8] = [
                    (x as i32 - 1, y as i32 - 1),
                    (x as i32, y as i32 - 1),
                    (x as i32 + 1, y as i32 - 1),
                    (x as i32 - 1, y as i32),
                    (x as i32 + 1, y as i32),
                    (x as i32 - 1, y as i32 + 1),
                    (x as i32, y as i32 + 1),
                    (x as i32 + 1, y as i32 + 1),
                ];

                let is_dlay = Self::is_dlay_element(element);
                let is_inst = Self::is_inst_element(element);

                if is_inst {
                    let mut powered = false;
                    for &(nx, ny) in &neighbors {
                        if self.element_at(nx, ny) == Some(Element::Spark)
                            || self.element_at(nx, ny) == Some(Element::Battery)
                        {
                            powered = true;
                            break;
                        }
                    }
                    if powered {
                        inst_sources.push((x, y));
                    }
                }

                if is_dlay {
                    let mut triggered = false;
                    for &(nx, ny) in &neighbors {
                        if self.element_at(nx, ny) == Some(Element::Spark) {
                            triggered = true;
                            break;
                        }
                    }

                    let life = self.grid[idx].as_ref().map_or(0, |p| p.lifetime);
                    if life == 0 {
                        if triggered {
                            let delay = element.lifetime().unwrap_or(8).max(1);
                            dlay_lifetime_updates.push((x, y, delay));
                        }
                    } else if life == 1 {
                        for &(nx, ny) in &neighbors {
                            if self.in_bounds(nx, ny) {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    let ni = self.idx(nx as usize, ny as usize);
                                    let can_conduct = ne.is_conductor()
                                        && !ne.is_insulator()
                                        && ne != Element::Battery
                                        && (ne != Element::Hswc || self.heat[ni] > 0.55);
                                    if can_conduct {
                                        spawns.push((nx as usize, ny as usize, Element::Spark));
                                    }
                                }
                            }
                        }
                        dlay_lifetime_updates.push((x, y, 0));
                    } else {
                        dlay_lifetime_updates.push((x, y, life.saturating_sub(1)));
                    }
                    continue;
                }

                match element {
                    Element::Wire => {
                        let mut energized = false;
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                if ne == Element::Spark
                                    || ne == Element::Battery
                                    || ne == Element::Fire
                                    || ne == Element::Plasma
                                {
                                    energized = true;
                                }
                            }
                        }
                        if energized {
                            self.heat[idx] = (self.heat[idx] + 0.08).clamp(0.0, 1.0);
                            for &(nx, ny) in &neighbors {
                                if self.in_bounds(nx, ny) {
                                    if let Some(ne) = self.element_at(nx, ny) {
                                        let ni = self.idx(nx as usize, ny as usize);
                                        let can_conduct = ne.is_conductor()
                                            && !ne.is_insulator()
                                            && ne != Element::Battery
                                            && (ne != Element::Hswc || self.heat[ni] > 0.55);
                                        if can_conduct && rng.gen_bool(0.45) {
                                            spawns.push((nx as usize, ny as usize, Element::Spark));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Element::Pscn => {
                        let mut pulsed = false;
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Spark)
                                || self.element_at(nx, ny) == Some(Element::Battery)
                            {
                                pulsed = true;
                            }
                        }
                        if pulsed {
                            let nx = x as i32;
                            let ny = y as i32 + 1;
                            if self.in_bounds(nx, ny) {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if ne.is_conductor() && ne != Element::Battery {
                                        spawns.push((nx as usize, ny as usize, Element::Spark));
                                    }
                                }
                            }
                        }
                    }
                    Element::Nscn => {
                        let mut pulsed = false;
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Spark)
                                || self.element_at(nx, ny) == Some(Element::Battery)
                            {
                                pulsed = true;
                            }
                        }
                        if pulsed {
                            let nx = x as i32;
                            let ny = y as i32 - 1;
                            if self.in_bounds(nx, ny) {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if ne.is_conductor() && ne != Element::Battery {
                                        spawns.push((nx as usize, ny as usize, Element::Spark));
                                    }
                                }
                            }
                        }
                    }
                    Element::Swch => {
                        let mut toggle_on = false;
                        let mut toggle_off = false;
                        for &(nx, ny) in &neighbors {
                            match self.element_at(nx, ny) {
                                Some(Element::Spark) | Some(Element::Pscn) => toggle_on = true,
                                Some(Element::Nscn) => toggle_off = true,
                                _ => {}
                            }
                        }
                        if toggle_on {
                            switch_updates.push((x, y, true));
                        }
                        if toggle_off {
                            switch_updates.push((x, y, false));
                        }
                        let is_on = extra == Some(Element::Spark) || toggle_on;
                        if is_on && self.tick.is_multiple_of(5) {
                            for &(nx, ny) in &neighbors {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if ne.is_conductor() && ne != Element::Battery {
                                        spawns.push((nx as usize, ny as usize, Element::Spark));
                                    }
                                }
                            }
                        }
                    }
                    Element::Battery => {
                        if self.tick.is_multiple_of(8) {
                            for &(nx, ny) in &neighbors {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if ne.is_conductor()
                                        && !ne.is_insulator()
                                        && ne != Element::Battery
                                    {
                                        spawns.push((nx as usize, ny as usize, Element::Spark));
                                    }
                                }
                            }
                        }
                    }
                    Element::Fuse => {
                        let mut burning = false;
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                if ne == Element::Fire
                                    || ne == Element::Spark
                                    || ne == Element::Plasma
                                {
                                    burning = true;
                                }
                            }
                        }
                        if burning {
                            changes.push((x, y, Some(Element::Fire)));
                            for &(nx, ny) in &neighbors {
                                if self.in_bounds(nx, ny)
                                    && self.element_at(nx, ny).is_none()
                                    && rng.gen_bool(0.2)
                                {
                                    spawns.push((nx as usize, ny as usize, Element::Spark));
                                }
                            }
                        }
                    }
                    Element::Spark => {
                        let spark_life = self.grid[idx].as_ref().map(|p| p.lifetime).unwrap_or(0);
                        let pulse = if spark_life > 4 {
                            0.9
                        } else if spark_life > 2 {
                            0.65
                        } else {
                            0.4
                        };
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                let ni = self.idx(nx as usize, ny as usize);
                                if ne.is_conductor()
                                    && !ne.is_insulator()
                                    && ne != Element::Battery
                                    && (ne != Element::Hswc || self.heat[ni] > 0.55)
                                    && rng.gen_bool(pulse)
                                {
                                    spawns.push((nx as usize, ny as usize, Element::Spark));
                                }
                                if ne.flammable() && rng.gen_bool(0.3) {
                                    changes.push((nx as usize, ny as usize, Some(Element::Fire)));
                                }
                            }
                        }
                    }
                    Element::Fire | Element::Plasma => {
                        let ignite_chance = if element == Element::Plasma {
                            0.25
                        } else {
                            0.08
                        };
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                if ne.flammable() && rng.gen_bool(ignite_chance) {
                                    if ne == Element::Gunpowder
                                        || ne == Element::Hydrogen
                                        || ne == Element::Methane
                                    {
                                        let radius = if ne == Element::Hydrogen {
                                            6
                                        } else if ne == Element::Methane {
                                            5
                                        } else {
                                            4
                                        };
                                        changes.push((
                                            nx as usize,
                                            ny as usize,
                                            Some(Element::Fire),
                                        ));
                                        let r2 = radius * radius;
                                        for ey in -radius..=radius {
                                            for ex in -radius..=radius {
                                                if ex * ex + ey * ey <= r2 {
                                                    let sx = nx + ex;
                                                    let sy = ny + ey;
                                                    if self.in_bounds(sx, sy) && rng.gen_bool(0.5) {
                                                        let pi = self.idx(sx as usize, sy as usize);
                                                        self.pressure[pi] = (self.pressure[pi]
                                                            + 0.3)
                                                            .clamp(-1.0, 1.0);
                                                        spawns.push((
                                                            sx as usize,
                                                            sy as usize,
                                                            if ne == Element::Hydrogen {
                                                                Element::Plasma
                                                            } else {
                                                                Element::Fire
                                                            },
                                                        ));
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        changes.push((
                                            nx as usize,
                                            ny as usize,
                                            Some(Element::Fire),
                                        ));
                                    }
                                }
                                if ne == Element::Oxygen && rng.gen_bool(0.35) {
                                    changes.push((nx as usize, ny as usize, Some(Element::Smoke)));
                                    for &(ox, oy) in &neighbors {
                                        if self.in_bounds(ox, oy)
                                            && self.element_at(ox, oy).is_none()
                                            && rng.gen_bool(0.2)
                                        {
                                            spawns.push((ox as usize, oy as usize, Element::Fire));
                                        }
                                    }
                                }
                                if element == Element::Plasma
                                    && !ne.is_special()
                                    && ne != Element::Fire
                                    && ne != Element::Plasma
                                    && ne != Element::Lava
                                    && rng.gen_bool(0.1)
                                {
                                    changes.push((nx as usize, ny as usize, None));
                                }
                            }
                        }
                    }
                    Element::Bomb => {
                        let mut trigger = false;
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                if ne == Element::Spark
                                    || ne == Element::Fire
                                    || ne == Element::Plasma
                                    || ne == Element::Embr
                                {
                                    trigger = true;
                                }
                            }
                        }
                        if trigger {
                            changes.push((x, y, Some(Element::Fire)));
                            let radius = 7;
                            let r2 = radius * radius;
                            for ey in -radius..=radius {
                                for ex in -radius..=radius {
                                    if ex * ex + ey * ey <= r2 {
                                        let sx = x as i32 + ex;
                                        let sy = y as i32 + ey;
                                        if self.in_bounds(sx, sy) {
                                            let pi = self.idx(sx as usize, sy as usize);
                                            self.pressure[pi] =
                                                (self.pressure[pi] + 0.45).clamp(-1.0, 1.0);
                                            self.heat[pi] = (self.heat[pi] + 0.30).clamp(0.0, 1.0);
                                            if rng.gen_bool(0.65) {
                                                spawns.push((
                                                    sx as usize,
                                                    sy as usize,
                                                    Element::Embr,
                                                ));
                                            } else if rng.gen_bool(0.7) {
                                                spawns.push((
                                                    sx as usize,
                                                    sy as usize,
                                                    Element::Fire,
                                                ));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Element::Dest => {
                        if self.tick.is_multiple_of(3) {
                            let radius = 3;
                            let r2 = radius * radius;
                            for ey in -radius..=radius {
                                for ex in -radius..=radius {
                                    if ex * ex + ey * ey <= r2 {
                                        let sx = x as i32 + ex;
                                        let sy = y as i32 + ey;
                                        if self.in_bounds(sx, sy) {
                                            let se = self.element_at(sx, sy);
                                            if se.is_some_and(|e| {
                                                e != Element::Dest && !e.is_special()
                                            }) {
                                                changes.push((sx as usize, sy as usize, None));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Element::Fan => {
                        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                            let sx = x as i32 + dx;
                            let sy = y as i32 + dy;
                            if self.in_bounds(sx, sy) {
                                let si = self.idx(sx as usize, sy as usize);
                                if !self.is_field_blocker_idx(si) {
                                    self.vel_x[si] =
                                        (self.vel_x[si] + dx as f32 * 0.15).clamp(-1.5, 1.5);
                                    self.vel_y[si] =
                                        (self.vel_y[si] + dy as f32 * 0.15).clamp(-1.5, 1.5);
                                    self.pressure[si] = (self.pressure[si] + 0.01).clamp(-1.0, 1.0);
                                }
                            }
                        }
                    }
                    Element::Grav => {
                        for ey in -8..=8 {
                            for ex in -8..=8 {
                                let sx = x as i32 + ex;
                                let sy = y as i32 + ey;
                                if !self.in_bounds(sx, sy) {
                                    continue;
                                }
                                let d2 = (ex * ex + ey * ey).max(1) as f32;
                                let force = 0.65 / d2.sqrt();
                                let si = self.idx(sx as usize, sy as usize);
                                self.vel_x[si] =
                                    (self.vel_x[si] - ex as f32 * force * 0.1).clamp(-1.5, 1.5);
                                self.vel_y[si] =
                                    (self.vel_y[si] - ey as f32 * force * 0.1).clamp(-1.5, 1.5);
                            }
                        }
                    }
                    Element::Bhol => {
                        for ey in -3..=3 {
                            for ex in -3..=3 {
                                let sx = x as i32 + ex;
                                let sy = y as i32 + ey;
                                if self.in_bounds(sx, sy) {
                                    let si = self.idx(sx as usize, sy as usize);
                                    self.pressure[si] = (self.pressure[si] - 0.10).clamp(-1.0, 1.0);
                                    if self.grid[si].as_ref().is_some_and(|p| {
                                        !p.element.is_special() && p.element != Element::Bhol
                                    }) {
                                        changes.push((sx as usize, sy as usize, None));
                                    }
                                }
                            }
                        }
                    }
                    Element::Whol => {
                        for ey in -4..=4 {
                            for ex in -4..=4 {
                                let sx = x as i32 + ex;
                                let sy = y as i32 + ey;
                                if self.in_bounds(sx, sy) {
                                    let si = self.idx(sx as usize, sy as usize);
                                    let d2 = (ex * ex + ey * ey).max(1) as f32;
                                    self.pressure[si] =
                                        (self.pressure[si] + 0.08 / d2.sqrt()).clamp(-1.0, 1.0);
                                    self.vel_x[si] =
                                        (self.vel_x[si] + ex as f32 * 0.012).clamp(-1.5, 1.5);
                                    self.vel_y[si] =
                                        (self.vel_y[si] + ey as f32 * 0.012).clamp(-1.5, 1.5);
                                }
                            }
                        }
                    }
                    Element::Uran => {
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Neut) && rng.gen_bool(0.45)
                            {
                                for (sdx, sdy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                                    let sx = x as i32 + sdx;
                                    let sy = y as i32 + sdy;
                                    if self.in_bounds(sx, sy) {
                                        let si = self.idx(sx as usize, sy as usize);
                                        self.heat[si] = (self.heat[si] + 0.20).clamp(0.0, 1.0);
                                        if self.grid[si].is_none() {
                                            let mut p = Particle::new(Element::Neut);
                                            p.extra = Some(Self::phot_dir_token(sdx, sdy));
                                            self.grid[si] = Some(p);
                                        }
                                    }
                                }
                                self.heat[idx] = (self.heat[idx] + 0.24).clamp(0.0, 1.0);
                                self.pressure[idx] = (self.pressure[idx] + 0.12).clamp(-1.0, 1.0);
                            }
                        }
                    }
                    Element::Aray => {
                        if self.tick.is_multiple_of(6) {
                            for step in 1..=12 {
                                let nx = x as i32 + step;
                                let ny = y as i32;
                                if !self.in_bounds(nx, ny) {
                                    break;
                                }
                                if self.element_at(nx, ny).is_none() {
                                    if rng.gen_bool(0.8) {
                                        spawns.push((nx as usize, ny as usize, Element::Spark));
                                    }
                                } else {
                                    if let Some(ne) = self.element_at(nx, ny) {
                                        if ne.is_conductor() {
                                            spawns.push((nx as usize, ny as usize, Element::Spark));
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                    }
                    Element::Gol => {
                        let mut neighbors_alive = 0;
                        for &(nx, ny) in &neighbors8 {
                            if self.element_at(nx, ny) == Some(Element::Gol) {
                                neighbors_alive += 1;
                            }
                        }
                        if !(neighbors_alive == 2 || neighbors_alive == 3) {
                            changes.push((x, y, None));
                        }
                    }
                    Element::Iron => {
                        for &(nx, ny) in &neighbors {
                            if (self.element_at(nx, ny) == Some(Element::Water)
                                || self.element_at(nx, ny) == Some(Element::SaltWater)
                                || self.element_at(nx, ny) == Some(Element::Mrcr))
                                && rng.gen_bool(0.05)
                            {
                                changes.push((x, y, Some(Element::Rust)));
                            }
                        }
                    }
                    Element::Soap => {
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Water) && rng.gen_bool(0.05)
                            {
                                changes.push((x, y, Some(Element::GelL)));
                                changes.push((nx as usize, ny as usize, Some(Element::Bizrg)));
                            }
                        }
                    }
                    Element::Gel => {
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Water) && rng.gen_bool(0.06)
                            {
                                changes.push((nx as usize, ny as usize, None));
                                if rng.gen_bool(0.5) {
                                    spawns.push((x, y, Element::Gel));
                                }
                            }
                        }
                    }
                    Element::Mrcr => {
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Iron) && rng.gen_bool(0.08)
                            {
                                changes.push((nx as usize, ny as usize, Some(Element::Rust)));
                            }
                        }
                    }
                    Element::Wax => {
                        if self.heat[idx] > 0.34 {
                            changes.push((x, y, Some(Element::Desl)));
                        }
                    }
                    Element::Dray => {
                        let mut triggered = false;
                        let mut dir = (1, 0);
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Spark) {
                                dir = (x as i32 - nx, y as i32 - ny);
                                triggered = true;
                                break;
                            }
                        }
                        if !triggered {
                            dir = Self::phot_dir_from(extra);
                        }
                        if triggered {
                            let mut copied: Option<Element> = None;
                            for step in 1..=16 {
                                let sx = x as i32 + dir.0 * step;
                                let sy = y as i32 + dir.1 * step;
                                if !self.in_bounds(sx, sy) {
                                    break;
                                }
                                let se = self.element_at(sx, sy);
                                if copied.is_none() {
                                    if let Some(e) = se {
                                        if !e.is_special() && e != Element::Spark {
                                            copied = Some(e);
                                        }
                                    }
                                    continue;
                                }
                                if se.is_none() {
                                    if let Some(copy_elem) = copied {
                                        spawns.push((sx as usize, sy as usize, copy_elem));
                                    }
                                    break;
                                }
                            }
                        }
                    }
                    Element::Cray => {
                        let mut triggered = false;
                        let mut dir = (1, 0);
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Spark) {
                                dir = (x as i32 - nx, y as i32 - ny);
                                triggered = true;
                                break;
                            }
                        }
                        if !triggered {
                            dir = Self::phot_dir_from(extra);
                        }
                        if triggered {
                            let to_make = extra.unwrap_or(Element::Fire);
                            for step in 1..=14 {
                                let sx = x as i32 + dir.0 * step;
                                let sy = y as i32 + dir.1 * step;
                                if !self.in_bounds(sx, sy) {
                                    break;
                                }
                                if self.element_at(sx, sy).is_none() {
                                    spawns.push((sx as usize, sy as usize, to_make));
                                    break;
                                }
                            }
                        } else if extra.is_none() {
                            for &(nx, ny) in &neighbors {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if !ne.is_special() && ne != Element::Spark {
                                        clone_updates.push((x, y, ne));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Element::Dtec => {
                        let mut detect = false;
                        for &(nx, ny) in &neighbors8 {
                            if let Some(ne) = self.element_at(nx, ny) {
                                if ne != Element::Dtec && ne != Element::Spark {
                                    detect = true;
                                }
                            }
                        }
                        if detect {
                            for &(nx, ny) in &neighbors {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if ne.is_conductor() {
                                        spawns.push((nx as usize, ny as usize, Element::Spark));
                                    }
                                }
                            }
                        }
                    }
                    Element::Tsns => {
                        let mut hot = self.heat[idx] > 0.62;
                        for &(nx, ny) in &neighbors {
                            if self.in_bounds(nx, ny)
                                && self.heat[self.idx(nx as usize, ny as usize)] > 0.62
                            {
                                hot = true;
                            }
                        }
                        if hot {
                            for &(nx, ny) in &neighbors {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if ne.is_conductor() {
                                        spawns.push((nx as usize, ny as usize, Element::Spark));
                                    }
                                }
                            }
                        }
                    }
                    Element::Psns => {
                        let mut high_p = self.pressure[idx].abs() > 0.22;
                        for &(nx, ny) in &neighbors {
                            if self.in_bounds(nx, ny)
                                && self.pressure[self.idx(nx as usize, ny as usize)].abs() > 0.22
                            {
                                high_p = true;
                            }
                        }
                        if high_p {
                            for &(nx, ny) in &neighbors {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if ne.is_conductor() {
                                        spawns.push((nx as usize, ny as usize, Element::Spark));
                                    }
                                }
                            }
                        }
                    }
                    Element::Pcln | Element::Pbcn => {
                        let mut triggered = false;
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Spark)
                                || self.element_at(nx, ny) == Some(Element::Battery)
                            {
                                triggered = true;
                            }
                            if extra.is_none() {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if !ne.is_special()
                                        && ne != Element::Pcln
                                        && ne != Element::Pbcn
                                    {
                                        clone_updates.push((x, y, ne));
                                    }
                                }
                            }
                        }
                        if triggered {
                            let out = if element == Element::Pbcn {
                                extra.map(|e| {
                                    if e == Element::Metal || e == Element::Iron {
                                        Element::Brmt
                                    } else {
                                        e
                                    }
                                })
                            } else {
                                extra
                            };
                            if let Some(stored) = out {
                                for &(nx, ny) in &neighbors {
                                    if self.in_bounds(nx, ny)
                                        && self.element_at(nx, ny).is_none()
                                        && rng.gen_bool(0.2)
                                    {
                                        spawns.push((nx as usize, ny as usize, stored));
                                    }
                                }
                            }
                        }
                    }
                    Element::Pump => {
                        let mut powered = false;
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Spark)
                                || self.element_at(nx, ny) == Some(Element::Battery)
                            {
                                powered = true;
                            }
                        }
                        if powered {
                            for ey in -2..=2 {
                                for ex in -2..=2 {
                                    let sx = x as i32 + ex;
                                    let sy = y as i32 + ey;
                                    if self.in_bounds(sx, sy) {
                                        let si = self.idx(sx as usize, sy as usize);
                                        self.pressure[si] =
                                            (self.pressure[si] + 0.08).clamp(-1.0, 1.0);
                                        self.vel_x[si] = (self.vel_x[si] + 0.08).clamp(-1.5, 1.5);
                                    }
                                }
                            }
                        }
                    }
                    Element::Gpmp => {
                        let mut powered = false;
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Spark)
                                || self.element_at(nx, ny) == Some(Element::Battery)
                            {
                                powered = true;
                            }
                        }
                        if powered {
                            for ey in -5..=5 {
                                for ex in -5..=5 {
                                    let sx = x as i32 + ex;
                                    let sy = y as i32 + ey;
                                    if self.in_bounds(sx, sy) {
                                        let si = self.idx(sx as usize, sy as usize);
                                        let d2 = (ex * ex + ey * ey).max(1) as f32;
                                        let force = 0.22 / d2.sqrt();
                                        self.vel_x[si] =
                                            (self.vel_x[si] - ex as f32 * force).clamp(-1.5, 1.5);
                                        self.vel_y[si] =
                                            (self.vel_y[si] - ey as f32 * force).clamp(-1.5, 1.5);
                                    }
                                }
                            }
                        }
                    }
                    Element::Pipe => {
                        if extra.is_none() {
                            for &(nx, ny) in &neighbors {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if !ne.is_special()
                                        && ne != Element::Pipe
                                        && ne != Element::Spark
                                    {
                                        changes.push((nx as usize, ny as usize, None));
                                        pipe_item_updates.push((x, y, Some(ne)));
                                        break;
                                    }
                                }
                            }
                        } else if let Some(stored) = extra {
                            let mut moved_item = false;
                            for &(nx, ny) in &neighbors {
                                if self.element_at(nx, ny) == Some(Element::Pipe) {
                                    let ni = self.idx(nx as usize, ny as usize);
                                    if self.grid[ni].as_ref().and_then(|p| p.extra).is_none() {
                                        pipe_item_updates.push((
                                            nx as usize,
                                            ny as usize,
                                            Some(stored),
                                        ));
                                        pipe_item_updates.push((x, y, None));
                                        moved_item = true;
                                        break;
                                    }
                                }
                            }
                            if !moved_item {
                                for &(nx, ny) in &neighbors {
                                    if self.in_bounds(nx, ny)
                                        && self.element_at(nx, ny).is_none()
                                        && rng.gen_bool(0.25)
                                    {
                                        spawns.push((nx as usize, ny as usize, stored));
                                        pipe_item_updates.push((x, y, None));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Element::PortalIn => {
                        if !portal_outs.is_empty() {
                            for &(nx, ny) in &neighbors {
                                if let Some(ne) = self.element_at(nx, ny) {
                                    if ne != Element::PortalIn
                                        && ne != Element::PortalOut
                                        && !ne.is_special()
                                    {
                                        changes.push((nx as usize, ny as usize, None));
                                        let &(ox, oy) =
                                            &portal_outs[rng.gen_range(0..portal_outs.len())];
                                        let options = [
                                            (ox as i32 + 1, oy as i32),
                                            (ox as i32 - 1, oy as i32),
                                            (ox as i32, oy as i32 + 1),
                                            (ox as i32, oy as i32 - 1),
                                        ];
                                        for (tx, ty) in options {
                                            if self.in_bounds(tx, ty)
                                                && self.element_at(tx, ty).is_none()
                                            {
                                                spawns.push((tx as usize, ty as usize, ne));
                                                break;
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    Element::WifiA => {
                        let mut powered = false;
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Spark)
                                || self.element_at(nx, ny) == Some(Element::Battery)
                            {
                                powered = true;
                            }
                        }
                        if powered {
                            for &(wx, wy) in &wifi_b {
                                spawns.push((wx, wy, Element::Spark));
                            }
                        }
                    }
                    Element::WifiB => {
                        let mut powered = false;
                        for &(nx, ny) in &neighbors {
                            if self.element_at(nx, ny) == Some(Element::Spark)
                                || self.element_at(nx, ny) == Some(Element::Battery)
                            {
                                powered = true;
                            }
                        }
                        if powered {
                            for &(wx, wy) in &wifi_a {
                                spawns.push((wx, wy, Element::Spark));
                            }
                        }
                    }
                    Element::Sing => {
                        let life = self.grid[idx].as_ref().map_or(0, |p| p.lifetime);
                        if life > 18 {
                            for ey in -7..=7 {
                                for ex in -7..=7 {
                                    let sx = x as i32 + ex;
                                    let sy = y as i32 + ey;
                                    if !self.in_bounds(sx, sy) {
                                        continue;
                                    }
                                    let si = self.idx(sx as usize, sy as usize);
                                    let d2 = (ex * ex + ey * ey).max(1) as f32;
                                    let force = 0.5 / d2.sqrt();
                                    self.vel_x[si] = (self.vel_x[si] - ex as f32 * force * 0.22)
                                        .clamp(-1.5, 1.5);
                                    self.vel_y[si] = (self.vel_y[si] - ey as f32 * force * 0.22)
                                        .clamp(-1.5, 1.5);
                                    self.pressure[si] =
                                        (self.pressure[si] - 0.02 / d2.sqrt()).clamp(-1.0, 1.0);
                                    if d2 < 6.0
                                        && self.grid[si].as_ref().is_some_and(|p| {
                                            p.element != Element::Sing && !p.element.is_special()
                                        })
                                    {
                                        changes.push((sx as usize, sy as usize, None));
                                    }
                                }
                            }
                        } else if life <= 2 {
                            let radius = 8;
                            let r2 = radius * radius;
                            changes.push((x, y, Some(Element::Fire)));
                            for ey in -radius..=radius {
                                for ex in -radius..=radius {
                                    if ex * ex + ey * ey > r2 {
                                        continue;
                                    }
                                    let sx = x as i32 + ex;
                                    let sy = y as i32 + ey;
                                    if self.in_bounds(sx, sy) {
                                        let si = self.idx(sx as usize, sy as usize);
                                        self.pressure[si] =
                                            (self.pressure[si] + 0.35).clamp(-1.0, 1.0);
                                        self.heat[si] = (self.heat[si] + 0.28).clamp(0.0, 1.0);
                                        if rng.gen_bool(0.55) {
                                            spawns.push((
                                                sx as usize,
                                                sy as usize,
                                                Element::Plasma,
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Element::Water => {
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                match ne {
                                    Element::Fire | Element::Plasma => {
                                        changes.push((
                                            nx as usize,
                                            ny as usize,
                                            Some(Element::Steam),
                                        ));
                                    }
                                    Element::Lava => {
                                        if rng.gen_bool(0.2) {
                                            changes.push((
                                                nx as usize,
                                                ny as usize,
                                                Some(Element::Stone),
                                            ));
                                            changes.push((x, y, Some(Element::Steam)));
                                        }
                                    }
                                    Element::Salt => {
                                        if rng.gen_bool(0.05) {
                                            changes.push((nx as usize, ny as usize, None));
                                            changes.push((x, y, Some(Element::SaltWater)));
                                        }
                                    }
                                    Element::Cryo => {
                                        if rng.gen_bool(0.2) {
                                            changes.push((x, y, Some(Element::Ice)));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Element::Lava => {
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                match ne {
                                    Element::Water | Element::Ice | Element::Snow => {
                                        if rng.gen_bool(0.15) {
                                            changes.push((
                                                nx as usize,
                                                ny as usize,
                                                Some(Element::Stone),
                                            ));
                                            if self.in_bounds(nx, ny - 1)
                                                && self.element_at(nx, ny - 1).is_none()
                                            {
                                                spawns.push((
                                                    nx as usize,
                                                    (ny - 1) as usize,
                                                    Element::Steam,
                                                ));
                                            }
                                        }
                                    }
                                    Element::Glass => {
                                        if rng.gen_bool(0.02) {
                                            changes.push((
                                                nx as usize,
                                                ny as usize,
                                                Some(Element::Lava),
                                            ));
                                        }
                                    }
                                    _ => {
                                        if ne.flammable() && rng.gen_bool(0.15) {
                                            changes.push((
                                                nx as usize,
                                                ny as usize,
                                                Some(Element::Fire),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Element::Cryo => {
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                match ne {
                                    Element::Fire | Element::Plasma => {
                                        changes.push((
                                            nx as usize,
                                            ny as usize,
                                            Some(Element::Smoke),
                                        ));
                                    }
                                    Element::Lava => {
                                        if rng.gen_bool(0.2) {
                                            changes.push((
                                                nx as usize,
                                                ny as usize,
                                                Some(Element::Stone),
                                            ));
                                            changes.push((x, y, Some(Element::Steam)));
                                        }
                                    }
                                    Element::Water | Element::SaltWater => {
                                        if rng.gen_bool(0.12) {
                                            changes.push((
                                                nx as usize,
                                                ny as usize,
                                                Some(Element::Ice),
                                            ));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    Element::Acid => {
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                if !ne.is_special()
                                    && ne != Element::Acid
                                    && ne != Element::Fire
                                    && ne != Element::Smoke
                                    && ne != Element::Steam
                                    && ne != Element::Glass
                                    && rng.gen_bool(0.04)
                                {
                                    changes.push((nx as usize, ny as usize, None));
                                    if rng.gen_bool(0.3) {
                                        changes.push((x, y, None));
                                    }
                                }
                            }
                        }
                    }
                    Element::Ice | Element::Snow => {
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                if (ne == Element::Fire
                                    || ne == Element::Lava
                                    || ne == Element::Plasma)
                                    && rng.gen_bool(0.08)
                                {
                                    changes.push((x, y, Some(Element::Water)));
                                }
                            }
                        }
                    }
                    Element::Cloner => {
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                if extra.is_none() && !ne.is_special() && ne != Element::Cloner {
                                    clone_updates.push((x, y, ne));
                                    break;
                                }
                            }
                        }
                        if let Some(stored) = extra {
                            for &(nx, ny) in &neighbors {
                                if self.in_bounds(nx, ny)
                                    && self.element_at(nx, ny).is_none()
                                    && rng.gen_bool(0.08)
                                {
                                    spawns.push((nx as usize, ny as usize, stored));
                                }
                            }
                        }
                    }
                    Element::Void => {
                        for &(nx, ny) in &neighbors {
                            if let Some(ne) = self.element_at(nx, ny) {
                                if !ne.is_special()
                                    && ne != Element::Wall
                                    && ne != Element::Void
                                    && ne != Element::Dmnd
                                {
                                    changes.push((nx as usize, ny as usize, None));
                                }
                            }
                        }
                    }
                    _ => {}
                }

                if self.heat[idx] > 0.65 && element.flammable() && rng.gen_bool(0.06) {
                    changes.push((x, y, Some(Element::Fire)));
                }
            }
        }

        if !inst_sources.is_empty() {
            let mut visited = vec![false; self.width * self.height];
            let mut stack = inst_sources;
            while let Some((x, y)) = stack.pop() {
                if x >= self.width || y >= self.height {
                    continue;
                }
                let idx = self.idx(x, y);
                if visited[idx] {
                    continue;
                }
                let Some(cell) = &self.grid[idx] else {
                    continue;
                };
                if !Self::is_inst_element(cell.element) {
                    continue;
                }
                visited[idx] = true;

                for (nx, ny) in [
                    (x as i32, y as i32 - 1),
                    (x as i32, y as i32 + 1),
                    (x as i32 - 1, y as i32),
                    (x as i32 + 1, y as i32),
                ] {
                    if !self.in_bounds(nx, ny) {
                        continue;
                    }
                    let ni = self.idx(nx as usize, ny as usize);
                    if let Some(ne) = self.element_at(nx, ny) {
                        if Self::is_inst_element(ne) {
                            if !visited[ni] {
                                stack.push((nx as usize, ny as usize));
                            }
                        } else {
                            let can_conduct = ne.is_conductor()
                                && !ne.is_insulator()
                                && ne != Element::Battery
                                && (ne != Element::Hswc || self.heat[ni] > 0.55);
                            if can_conduct {
                                spawns.push((nx as usize, ny as usize, Element::Spark));
                            }
                        }
                    }
                }
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[self.idx(x, y)].is_some() {
                    continue;
                }
                let mut neighbors_alive = 0;
                for ny in (y as i32 - 1)..=(y as i32 + 1) {
                    for nx in (x as i32 - 1)..=(x as i32 + 1) {
                        if nx == x as i32 && ny == y as i32 {
                            continue;
                        }
                        if self.in_bounds(nx, ny) && self.element_at(nx, ny) == Some(Element::Gol) {
                            neighbors_alive += 1;
                        }
                    }
                }
                if neighbors_alive == 3 {
                    spawns.push((x, y, Element::Gol));
                }
            }
        }

        for (x, y, elem) in changes {
            if x < self.width && y < self.height {
                let idx = self.idx(x, y);
                match elem {
                    Some(e) => self.grid[idx] = Some(Particle::new(e)),
                    None => self.grid[idx] = None,
                }
            }
        }

        for (x, y, stored) in clone_updates {
            let idx = self.idx(x, y);
            if let Some(p) = &mut self.grid[idx] {
                p.extra = Some(stored);
            }
        }

        for (x, y, on) in switch_updates {
            let idx = self.idx(x, y);
            if let Some(p) = &mut self.grid[idx] {
                if p.element == Element::Swch {
                    p.extra = if on { Some(Element::Spark) } else { None };
                }
            }
        }

        for (x, y, item) in pipe_item_updates {
            let idx = self.idx(x, y);
            if let Some(p) = &mut self.grid[idx] {
                if p.element == Element::Pipe {
                    p.extra = item;
                }
            }
        }

        for (x, y, life) in dlay_lifetime_updates {
            let idx = self.idx(x, y);
            if let Some(p) = &mut self.grid[idx] {
                if Self::is_dlay_element(p.element) {
                    p.lifetime = life;
                }
            }
        }

        for (x, y, elem) in spawns {
            if x < self.width && y < self.height {
                let idx = self.idx(x, y);
                if self.grid[idx].is_none() {
                    let mut p = Particle::new(elem);
                    if elem == Element::Phot || elem == Element::Neut {
                        p.extra = Some(Self::phot_dir_token(1, 0));
                    }
                    self.grid[idx] = Some(p);
                }
            }
        }
    }

    fn side_pressure_bias(&self, x: usize, y: usize) -> i32 {
        let idx = self.idx(x, y);
        let left = if x > 0 {
            self.pressure[self.idx(x - 1, y)]
        } else {
            0.0
        };
        let right = if x + 1 < self.width {
            self.pressure[self.idx(x + 1, y)]
        } else {
            0.0
        };
        let flow_x = self.vel_x[idx] * 0.25;
        if right + flow_x - left > 0.02 {
            1
        } else if left - (right + flow_x) > 0.02 {
            -1
        } else {
            0
        }
    }

    fn vertical_velocity_bias(&self, x: usize, y: usize) -> i32 {
        let v = self.vel_y[self.idx(x, y)];
        if v > 0.35 {
            1
        } else if v < -0.35 {
            -1
        } else {
            0
        }
    }

    pub fn place_brush(&mut self, cx: usize, cy: usize, element: Element, radius: usize) {
        let ri = radius as i32;
        let mut rng = rand::thread_rng();
        for dy in -ri..=ri {
            for dx in -ri..=ri {
                if dx * dx + dy * dy <= ri * ri {
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    if self.in_bounds(nx, ny) {
                        let idx = self.idx(nx as usize, ny as usize);
                        if self.grid[idx].is_none() && (radius <= 1 || rng.gen_bool(0.8)) {
                            self.grid[idx] = Some(Particle::new(element));
                            self.heat[idx] = match element {
                                Element::Fire => 0.9,
                                Element::Cflm => 0.82,
                                Element::Plasma => 1.0,
                                Element::Lava => 0.8,
                                Element::Ice | Element::Snow | Element::Cooler | Element::Cryo => {
                                    0.03
                                }
                                Element::Heater => 0.7,
                                Element::Wax => 0.18,
                                Element::Desl => 0.20,
                                Element::GelL
                                | Element::Water
                                | Element::SaltWater
                                | Element::Oil
                                | Element::Alcohol => 0.22,
                                Element::Sing => 0.35,
                                _ => self.heat[idx].max(0.22),
                            };
                        }
                    }
                }
            }
        }
    }

    pub fn erase_brush(&mut self, cx: usize, cy: usize, radius: usize) {
        let ri = radius as i32;
        for dy in -ri..=ri {
            for dx in -ri..=ri {
                if dx * dx + dy * dy <= ri * ri {
                    let nx = cx as i32 + dx;
                    let ny = cy as i32 + dy;
                    if self.in_bounds(nx, ny) {
                        self.erase(nx as usize, ny as usize);
                    }
                }
            }
        }
    }
}

fn random_dirs() -> (i32, i32) {
    if rand::thread_rng().gen::<bool>() {
        (-1, 1)
    } else {
        (1, -1)
    }
}
