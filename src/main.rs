#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use slint::SharedString;
slint::include_modules!();

struct Config {
    max_rod: i32,
    leftover: i32,
    parts: i32,
    part_len: i32,
    remainings: i32,
}

fn parse_input(ui: &AppWindow) -> Result<Config, SharedString> {
    ui.set_status_text("".into());
    let parse = |s: &str, name: &str| {
        s.trim()
            .parse::<i32>()
            .map_err(|e| SharedString::from(format!("Ошибка '{}' в поле '{}'", e, name)))
    };

    Ok(Config {
        max_rod: parse(&ui.get_max_rod_length(), "Максимальная длина прута")?,
        leftover: parse(&ui.get_tail(), "Остаток")?,
        parts: parse(&ui.get_parts_count(), "Количество заготовок")?,
        part_len: parse(&ui.get_part_length(), "Длина заготовки")?,
        remainings: ui.get_remainings_count(),
    })
}

fn rod_suffix(count: i32) -> &'static str {
    if count % 10 == 1 && count % 100 != 11 {
        ""
    } else if (2..=4).contains(&(count % 10)) && !(12..=14).contains(&(count % 100)) {
        "а"
    } else {
        "ов"
    }
}

fn part_suffix(count: i32) -> &'static str {
    if count % 10 == 1 && count % 100 != 11 {
        "ь"
    } else if (2..=4).contains(&(count % 10)) && !(12..=14).contains(&(count % 100)) {
        "и"
    } else {
        "ей"
    }
}

fn formula(p: i32, o: i32, m: i32) -> SharedString {
    SharedString::from(format!(
        "\nПуть расчета:\n{} × n + {} ≤ {}\n{} × n ≤ {}\nN ≤ {:.3}\nМаксимум деталей на прут: n = {}\n",
        p,
        o,
        m,
        p,
        m - o,
        (m - o) as f64 / p as f64,
        (m - o) / p
    ))
}

fn calc_simple(cfg: &Config) -> SharedString {
    let max_per = (cfg.max_rod - cfg.leftover) / cfg.part_len;
    let full = cfg.parts / max_per;
    let rem = cfg.parts % max_per;
    let mut out = String::new();
    if full > 0 {
        out += &format!(
            "- {} прут{} по {} мм ({} детал{} с прута)\n",
            full,
            rod_suffix(full),
            max_per * cfg.part_len + cfg.leftover,
            max_per,
            part_suffix(max_per)
        );
    }
    if rem > 0 {
        out += &format!(
            "- 1 прут: {} мм ({} детал{} с прута)\n",
            rem * cfg.part_len + cfg.leftover,
            rem,
            part_suffix(rem)
        );
    }
    let rods_count = full + if rem > 0 { 1 } else { 0 };
    let total_mat = (full * max_per + rem) * cfg.part_len + rods_count * cfg.leftover;
    let total_left = rods_count * cfg.leftover;
    out += &format!(
        "Общий расход материала: {} мм\nОбщий остаток: {} мм",
        total_mat, total_left
    );
    SharedString::from(out) + &formula(cfg.part_len, cfg.leftover, cfg.max_rod)
}

fn calc_all_equal(cfg: &Config) -> SharedString {
    let max_per = (cfg.max_rod - cfg.leftover) / cfg.part_len;
    let mut best = (0, i32::MAX);
    for n in 1..=max_per {
        if n * cfg.part_len + cfg.leftover <= cfg.max_rod {
            let rods = (cfg.parts + n - 1) / n;
            if rods * n == cfg.parts && rods < best.1 {
                best = (n, rods);
            }
        }
    }
    if best.0 == 0 {
        return SharedString::from("Невозможно равномерно распределить детали по прутам.");
    }
    let (n, rods) = best;
    let len = n * cfg.part_len + cfg.leftover;
    let mat = rods * len;
    let left = rods * cfg.leftover;
    SharedString::from(format!(
        "- {} прут{} по {} мм ({} детал{} с прута)\nОбщий расход материала: {} мм\nОбщий остаток: {} мм",
        rods,
        rod_suffix(rods),
        len,
        n,
        part_suffix(n),
        mat,
        left
    )) + &formula(cfg.part_len, cfg.leftover, cfg.max_rod)
}

fn calc_complicated(cfg: &Config) -> SharedString {
    let max_per = (cfg.max_rod - cfg.leftover) / cfg.part_len;

    let total_full_rods_needed = cfg.parts / max_per; 
    let remaining_parts = cfg.parts % max_per;
    

    let full_rods = if remaining_parts > 0 {
        if total_full_rods_needed >= cfg.remainings {
            total_full_rods_needed - cfg.remainings + 1
        } else {
            0
        }
    } else {
        if total_full_rods_needed > cfg.remainings {
            total_full_rods_needed - cfg.remainings
        } else {
            total_full_rods_needed
        }
    };
    
    let parts_for_remaining_rods = cfg.parts - full_rods * max_per;
    
    let mut out = String::new();
    
    if full_rods > 0 {
        out += &format!(
            "- {} прут{} по {} мм ({} детал{} с прута)\n",
            full_rods,
            rod_suffix(full_rods),
            max_per * cfg.part_len + cfg.leftover,
            max_per,
            part_suffix(max_per)
        );
    }
    
    if parts_for_remaining_rods > 0 {
        let needed_remaining_rods = if parts_for_remaining_rods <= max_per {
            1 
        } else {
            cfg.remainings
        };
        
        if needed_remaining_rods == 1 {
            out += &format!(
                "- 1 прут по {} мм ({} детал{} с прута)\n",
                parts_for_remaining_rods * cfg.part_len + cfg.leftover,
                parts_for_remaining_rods,
                part_suffix(parts_for_remaining_rods)
            );
        } else {
            let parts_per_rod = (parts_for_remaining_rods + needed_remaining_rods - 1) / needed_remaining_rods;
            let full_remaining_rods = parts_for_remaining_rods / parts_per_rod;
            let last_rod_parts = parts_for_remaining_rods % parts_per_rod;
            
            if full_remaining_rods > 0 {
                out += &format!(
                    "- {} прут{} по {} мм ({} детал{} с прута)\n",
                    full_remaining_rods,
                    rod_suffix(full_remaining_rods),
                    parts_per_rod * cfg.part_len + cfg.leftover,
                    parts_per_rod,
                    part_suffix(parts_per_rod)
                );
            }
            
            if last_rod_parts > 0 {
                out += &format!(
                    "- 1 прут по {} мм ({} детал{} с прута)\n",
                    last_rod_parts * cfg.part_len + cfg.leftover,
                    last_rod_parts,
                    part_suffix(last_rod_parts)
                );
            }
        }
    }

    let mut total_mat = 0;
    let mut total_left = 0;

    if full_rods > 0 {
        total_mat += full_rods * (max_per * cfg.part_len + cfg.leftover);
        total_left += full_rods * cfg.leftover;
    }

    if parts_for_remaining_rods > 0 {
        let needed_remaining_rods = if parts_for_remaining_rods <= max_per {
            1
        } else {
            cfg.remainings
        };
        
        if needed_remaining_rods == 1 {
            total_mat += parts_for_remaining_rods * cfg.part_len + cfg.leftover;
            total_left += cfg.leftover;
        } else {
            let parts_per_rod = (parts_for_remaining_rods + needed_remaining_rods - 1) / needed_remaining_rods;
            let full_remaining_rods = parts_for_remaining_rods / parts_per_rod;
            let last_rod_parts = parts_for_remaining_rods % parts_per_rod;
            
            total_mat += full_remaining_rods * (parts_per_rod * cfg.part_len + cfg.leftover);
            total_left += full_remaining_rods * cfg.leftover;
            
            if last_rod_parts > 0 {
                total_mat += last_rod_parts * cfg.part_len + cfg.leftover;
                total_left += cfg.leftover;
            }
        }
    }
    
    out += &format!(
        "Общий расход материала: {} мм\nОбщий остаток: {} мм",
        total_mat, total_left
    );
    
    SharedString::from(out) + &formula(cfg.part_len, cfg.leftover, cfg.max_rod)
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    ui.on_calculate({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            match parse_input(&ui) {
                Err(e) => {
                    ui.set_simple_result("".into());
                    ui.set_all_equal_result("".into());
                    ui.set_complicated_result("".into());
                    ui.set_status_text(e);
                }
                Ok(cfg) => {
                    ui.set_simple_result(calc_simple(&cfg));
                    ui.set_all_equal_result(calc_all_equal(&cfg));
                    ui.set_complicated_result(calc_complicated(&cfg));
                    ui.set_status_text("".into());
                }
            }
        }
    });
    ui.run()?;
    Ok(())
}