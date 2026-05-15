use macroquad::prelude::*;
use crate::game_state;
use crate::ui_helper as ui;
use crate::constants as con;
use crate::tank;

pub enum hudAction {
    FeedFish,
    AddFish(usize),
    TestAddFish(usize),
    Save,
    Settings,
    FishStats,
    Store,
    Testing,
    AddPrestige,
    BuyFood,
    BuyTankCap,
    DebugIndexIncrease,
    DebugIndexDecrease,
    StoreScrollUp,
    StoreScrollDown,
    ChangeWater,
    TestChangeStat(tank::WaterParameter, bool),
    DebugShiftStatRight,
    DebugShiftStatLeft,
    DebugShiftStatPositive,
    DebugShiftStatNegative,
    TestToggleLight,
    None,
}

#[derive(PartialEq)]
pub enum BottomTab {
    FishStats,
    Store,
    Testing, // Add/Feed fish and other testing features
}

// ----- Local palette helpers (cosmetic only, doesn't touch ui_helper) -----
// Soft neutral panel backgrounds so sections read as distinct regions.
const PANEL_BG: Color        = Color { r: 0.94, g: 0.94, b: 0.96, a: 1.00 };
const PANEL_BG_ALT: Color    = Color { r: 0.88, g: 0.89, b: 0.93, a: 1.00 };
const HEADER_STRIP: Color    = Color { r: 0.20, g: 0.28, b: 0.42, a: 1.00 };
const TOP_BAR_BG: Color      = Color { r: 0.85, g: 0.87, b: 0.92, a: 1.00 };
const BUTTON_NEUTRAL: Color  = Color::new(0.75, 0.75, 0.78, 1.0);
const BUTTON_ACTION: Color   = Color::new(0.55, 0.78, 0.58, 1.0); // greenish for primary action
const BUTTON_DANGER: Color   = Color::new(0.85, 0.55, 0.55, 1.0); // for water change / toggle
const TAB_INACTIVE: Color    = Color::new(0.78, 0.80, 0.84, 1.0);
const TAB_ACTIVE: Color      = Color::new(0.30, 0.45, 0.70, 1.0);
const ROW_STRIPE: Color      = Color { r: 0.90, g: 0.91, b: 0.94, a: 1.00 };

fn parameter_colour(value: f64, min: f64, max: f64) -> Color {
    //Alter at somepoint to accoount for 0.00 being ideal
    let margin = (max - min) * 0.1;
    if value < min || value > max {
        RED
    } else if value < min + margin || value > max - margin {
        ORANGE
    } else {
        GREEN
    }
}

pub fn draw_main_hud(gameState: &game_state::GameState, active_tab: &BottomTab) -> hudAction {
    let sw = screen_width();
    let sh = screen_height();

    // =====================================================================
    // TOP BAR — single visual strip with grouped controls
    // Left zone: Save | Water Change
    // Center zone: Rank | Prestige (big) | Food
    // Right zone: Settings
    // =====================================================================
    let top_bar_y = 0.0;
    let top_bar_h = sh * 0.10;
    draw_rectangle(0.0, top_bar_y, sw, top_bar_h, TOP_BAR_BG);
    draw_line(0.0, top_bar_h, sw, top_bar_h, 2.0, con::AREA_BORDER_COLOUR);

    // Uniform button geometry across the top bar so nothing looks staggered
    let tb_btn_w = sw * 0.09;
    let tb_btn_h = sh * 0.055;
    let tb_btn_y = top_bar_y + (top_bar_h - tb_btn_h) * 0.5;

    // -- Left cluster --
    let left_pad = sw * 0.015;
    let mut x_cursor = left_pad;

    if ui::draw_button_box(x_cursor, tb_btn_y, tb_btn_w, tb_btn_h, BUTTON_NEUTRAL, "Save", BLACK) {
        return hudAction::Save;
    }
    x_cursor += tb_btn_w + sw * 0.01;

    if ui::draw_button_box(x_cursor, tb_btn_y, tb_btn_w * 1.2, tb_btn_h, BUTTON_DANGER,
        &format!("Water Change {}%", &gameState.player.water_change_percent), BLACK) {
        return hudAction::ChangeWater;
    }

    // -- Right cluster --
    let right_pad = sw * 0.015;
    let mut rx_cursor = sw - right_pad - tb_btn_w;

    if ui::draw_button_box(rx_cursor, tb_btn_y, tb_btn_w, tb_btn_h, BUTTON_NEUTRAL, "Settings", BLACK) {
        return hudAction::Settings;
    }
    rx_cursor -= tb_btn_w + sw * 0.01;

    // Food button (right-of-center, before settings)
    if ui::draw_button_box(rx_cursor, tb_btn_y, tb_btn_w, tb_btn_h, BUTTON_ACTION,
        &format!("Food lvl {}", &gameState.player.current_food_level), BLACK) {
        return hudAction::FeedFish;
    }

    // -- Center cluster: Rank badge + Prestige badge --
    // Prestige is the headline number, so it gets the bigger box and bolder color.
    let prestige_w = sw * 0.18;
    let prestige_h = sh * 0.07;
    let prestige_cx = sw * 0.5;
    let prestige_cy = top_bar_y + top_bar_h * 0.5;
    ui::draw_centered_text_box(
        prestige_cx, prestige_cy,
        prestige_w, prestige_h,
        HEADER_STRIP,
        &format!("Prestige: {:.2}", gameState.player.current_prestige),
        WHITE,
    );

    // Rank sits just left of the prestige badge
    let rank_w = sw * 0.08;
    let rank_h = sh * 0.055;
    let rank_x = prestige_cx - prestige_w * 0.5 - rank_w - sw * 0.01;
    let rank_y = top_bar_y + (top_bar_h - rank_h) * 0.5;
    ui::draw_text_box(rank_x, rank_y, rank_w, rank_h, BUTTON_NEUTRAL,
        &format!("Rank: {}", gameState.player.rank), BLACK);

    // =====================================================================
    // SIDE PANEL — Tank Status (left of tank)
    // Uses same x/y region as before but with header strip + striped rows.
    // =====================================================================
    let side_x = 0.0;
    let side_y = sh * 0.125;
    let side_w = sw * con::STAT_AREA_WIDTH;
    let side_h = sh * con::STAT_AREA_HEIGHT;

    // Panel background
    draw_rectangle(side_x, side_y, side_w, side_h, PANEL_BG);

    // Header strip
    let header_h = sh * 0.04;
    draw_rectangle(side_x, side_y, side_w, header_h, HEADER_STRIP);
    let header_font = header_h * 0.55;
    let header_text = "Tank Status";
    let ht_size = measure_text(header_text, None, header_font as u16, 1.0);
    draw_text(
        header_text,
        side_x + side_w * 0.5 - ht_size.width * 0.5,
        side_y + header_h * 0.5 + ht_size.height * 0.5,
        header_font,
        WHITE,
    );

    // Stat rows with subtle alternating stripes
    let rows_top = side_y + header_h + sh * 0.01;
    let row_h = sh * 0.045;
    let row_inset_x = side_x + sw * 0.005;
    let row_inset_w = side_w - sw * 0.01;

    let stats: [(String, Color); 7] = [
        (
            format!("Tank occupancy: {} ({})",
                gameState.player.current_fish_owned, gameState.tank.max_fish),
            BLACK,
        ),
        (
            format!("Tank Temp: {:.1}°C", gameState.tank.water_parameters.temprature),
            parameter_colour(
                gameState.tank.water_parameters.temprature,
                gameState.tank.ideal_parameters.temprature_range.min,
                gameState.tank.ideal_parameters.temprature_range.max,
            ),
        ),
        (
            format!("Tank PH: {:.1}pH", gameState.tank.water_parameters.ph),
            parameter_colour(
                gameState.tank.water_parameters.ph,
                gameState.tank.ideal_parameters.ph_range.min,
                gameState.tank.ideal_parameters.ph_range.max,
            ),
        ),
        (
            format!("Tank GH: {:.1}°dGH", gameState.tank.water_parameters.gh),
            parameter_colour(
                gameState.tank.water_parameters.gh,
                gameState.tank.ideal_parameters.gh_range.min,
                gameState.tank.ideal_parameters.gh_range.max,
            ),
        ),
        (
            format!("Tank Ammonia: {:.1}ppm", gameState.tank.water_parameters.ammonia),
            parameter_colour(
                gameState.tank.water_parameters.ammonia,
                gameState.tank.ideal_parameters.ammonia_range.min,
                gameState.tank.ideal_parameters.ammonia_range.max,
            ),
        ),
        (
            format!("Tank Nitrite: {:.1}ppm", gameState.tank.water_parameters.nitrite),
            parameter_colour(
                gameState.tank.water_parameters.nitrite,
                gameState.tank.ideal_parameters.nitrite_range.min,
                gameState.tank.ideal_parameters.nitrite_range.max,
            ),
        ),
        (
            format!("Tank Nitrate: {:.1}ppm", gameState.tank.water_parameters.nitrate),
            parameter_colour(
                gameState.tank.water_parameters.nitrate,
                gameState.tank.ideal_parameters.nitrate_range.min,
                gameState.tank.ideal_parameters.nitrate_range.max,
            ),
        ),
    ];

    for (i, (text, color)) in stats.iter().enumerate() {
        let ry = rows_top + i as f32 * row_h;
        if i % 2 == 1 {
            draw_rectangle(row_inset_x, ry, row_inset_w, row_h, ROW_STRIPE);
        }
        ui::draw_stat(row_inset_x, ry, row_inset_w, row_h, text, *color);
    }

    // Side panel outer border
    draw_rectangle_lines(side_x, side_y, side_w, side_h, 5.0, con::AREA_BORDER_COLOUR);

    // =====================================================================
    // TANK — unchanged position/size, just retained
    // =====================================================================
    ui::draw_tank(sw * 0.25, sh * 0.125, sw * con::TANK_WIDTH, sh * con::TANK_HEIGHT);
    //Add shadow for when light are off
    if !gameState.tank.lighting.on {
        ui::draw_button_box(sw * 0.25, sh * 0.125, sw * con::TANK_WIDTH, sh * con::TANK_HEIGHT,
            Color::from_rgba(0, 0, 0, 50), "", BLACK);
    }

    // =====================================================================
    // BOTTOM AREA — backing panel + tab bar
    // =====================================================================
    let bottom_top = sh * 0.6;
    let bottom_h = sh * con::BOTTOM_AREA_HEIGHT;
    let tab_strip_h = sh * con::BOTTOM_TAB_AREA_HEIGHT;

    // Full bottom panel background (under content area, not the tab strip)
    draw_rectangle(0.0, bottom_top + tab_strip_h, sw, bottom_h - tab_strip_h, PANEL_BG);

    // Tab strip background
    draw_rectangle(0.0, bottom_top, sw, tab_strip_h, PANEL_BG_ALT);
    draw_line(0.0, bottom_top + tab_strip_h, sw, bottom_top + tab_strip_h,
              2.0, con::AREA_BORDER_COLOUR);

    // Three evenly-spaced tabs with the active one highlighted
    let tab_w = sw * 0.13;
    let tab_h = tab_strip_h * 0.75;
    let tab_y = bottom_top + (tab_strip_h - tab_h) * 0.5;
    let tab_gap = sw * 0.01;
    let total_tabs_w = tab_w * 3.0 + tab_gap * 2.0;
    let tabs_start_x = sw * 0.5 - total_tabs_w * 0.5;

    let tab_specs: [(BottomTab, &str); 3] = [
        (BottomTab::FishStats, "Fish Stats"),
        (BottomTab::Store,     "Store"),
        (BottomTab::Testing,   "Testing"),
    ];

    for (i, (tab, label)) in tab_specs.iter().enumerate() {
        let tx = tabs_start_x + i as f32 * (tab_w + tab_gap);
        let is_active = active_tab == tab;
        let (bg, fg) = if is_active { (TAB_ACTIVE, WHITE) } else { (TAB_INACTIVE, BLACK) };
        if ui::draw_button_box(tx, tab_y, tab_w, tab_h, bg, label, fg) {
            return match tab {
                BottomTab::FishStats => hudAction::FishStats,
                BottomTab::Store     => hudAction::Store,
                BottomTab::Testing   => hudAction::Testing,
            };
        }
    }

    // Outer border around whole bottom area
    draw_rectangle_lines(0.0, bottom_top, sw, bottom_h, 5.0, con::AREA_BORDER_COLOUR);

    // =====================================================================
    // BOTTOM CONTENT — switches by active tab
    // =====================================================================
    let content_top = bottom_top + tab_strip_h;
    let content_pad = sw * 0.015;

    match active_tab {
        &BottomTab::FishStats => {
            // Fish columns with a header strip on each card
            let fish_count = gameState.tank.fish.len();
            if fish_count == 0 {
                ui::draw_stat(0.0, content_top + sh * 0.05, sw, sh * 0.04,
                    "No fish in the tank yet.", BLACK);
            } else {
                let col_gap = sw * 0.005;
                let total_gap = col_gap * (fish_count - 1).max(0) as f32;
                let col_w = (sw - content_pad * 2.0 - total_gap) / fish_count as f32;
                let card_top = content_top + sh * 0.01;
                let card_h = sh * 0.27;
                let card_header_h = sh * 0.035;
                let row_inner_h = sh * 0.033;

                for (i, fish) in gameState.tank.fish.iter().enumerate() {
                    let cx = content_pad + i as f32 * (col_w + col_gap);

                    // Card background
                    draw_rectangle(cx, card_top, col_w, card_h, PANEL_BG_ALT);
                    // Header strip
                    draw_rectangle(cx, card_top, col_w, card_header_h, HEADER_STRIP);
                    let hs = format!("{}", fish.species);
                    let fs = card_header_h * 0.55;
                    let ts = measure_text(&hs, None, fs as u16, 1.0);
                    draw_text(
                        &hs,
                        cx + col_w * 0.5 - ts.width * 0.5,
                        card_top + card_header_h * 0.5 + ts.height * 0.5,
                        fs,
                        WHITE,
                    );

                    // Rows below header
                    let rows = [
                        format!("Age {} ({})", fish.age, fish.max_age),
                        format!("Hunger {:.2}", fish.hunger),
                        format!("Status {:?}", fish.status),
                        format!("PPS {}", fish.get_prestige()),
                        format!("Traits {:.5?}", fish.fish_traits[0]),
                        format!("Mods {:.5?}", fish.moddifiers[0]),
                    ];
                    let rows_top = card_top + card_header_h + sh * 0.005;
                    for (r, text) in rows.iter().enumerate() {
                        let ry = rows_top + r as f32 * row_inner_h;
                        if r % 2 == 1 {
                            draw_rectangle(cx + 2.0, ry, col_w - 4.0, row_inner_h, ROW_STRIPE);
                        }
                        ui::draw_stat(cx, ry, col_w, row_inner_h, text, BLACK);
                    }

                    // Card border
                    draw_rectangle_lines(cx, card_top, col_w, card_h,
                        3.0, con::AREA_BORDER_COLOUR);
                }
            }
        },

        &BottomTab::Store => {
            // ---- Left half: scrollable fish cards ----
            let cards_per_row = 2;
            let visible_rows = 2;
            let store_left = content_pad;
            let store_top = content_top + sh * 0.015;
            let store_width = sw * 0.5 - content_pad;
            let card_w = (store_width - sw * 0.005) / cards_per_row as f32;
            let card_h = sh * 0.13;

            // Left-section header
            let sec_header_h = sh * 0.03;
            draw_rectangle(store_left, content_top + sh * 0.002,
                store_width, sec_header_h, HEADER_STRIP);
            let sh_text = "Fish Store";
            let shf = sec_header_h * 0.6;
            let shs = measure_text(sh_text, None, shf as u16, 1.0);
            draw_text(sh_text,
                store_left + sw * 0.01,
                content_top + sh * 0.002 + sec_header_h * 0.5 + shs.height * 0.5,
                shf, WHITE);

            let scroll = gameState.debugger.store_scroll_offset;
            let fish_list = &gameState.fish_registry.fish;
            let total_fish = fish_list.len();

            // Scroll buttons stacked on the right edge of the store left section
            let scroll_btn_w = sw * 0.03;
            let scroll_btn_h = sh * 0.04;
            let scroll_btn_x = store_left + store_width - scroll_btn_w - sw * 0.003;

            if ui::draw_button_box(scroll_btn_x, store_top, scroll_btn_w, scroll_btn_h,
                BUTTON_NEUTRAL, "^", BLACK) {
                return hudAction::StoreScrollUp;
            }
            if ui::draw_button_box(scroll_btn_x,
                store_top + visible_rows as f32 * card_h - scroll_btn_h,
                scroll_btn_w, scroll_btn_h, BUTTON_NEUTRAL, "v", BLACK) {
                return hudAction::StoreScrollDown;
            }

            // Adjust card area to leave room for scroll column
            let card_area_w = store_width - scroll_btn_w - sw * 0.008;
            let adj_card_w = (card_area_w - sw * 0.005) / cards_per_row as f32;

            for i in 0..(cards_per_row * visible_rows) {
                let index = scroll * cards_per_row + i;
                if index >= total_fish { break; }

                let species = &fish_list[index];
                let col = (i % cards_per_row) as f32;
                let row = (i / cards_per_row) as f32;

                let x = store_left + col * (adj_card_w + sw * 0.005);
                let y = store_top + row * card_h;
                let card_inner_h = card_h - sh * 0.01;

                // Card background
                draw_rectangle(x, y, adj_card_w, card_inner_h, PANEL_BG_ALT);

                // Mini header strip on each card
                let mh = sh * 0.025;
                draw_rectangle(x, y, adj_card_w, mh, HEADER_STRIP);
                let title = format!("{}  (Tier {})", species.species, species.tier);
                let mf = mh * 0.6;
                let mt = measure_text(&title, None, mf as u16, 1.0);
                draw_text(&title,
                    x + sw * 0.005,
                    y + mh * 0.5 + mt.height * 0.5, mf, WHITE);

                ui::draw_stat(x, y + mh + sh * 0.005, adj_card_w, sh * 0.025,
                    &format!("Cost: {:.0}", gameState.economy.get_cost(species)), BLACK);
                ui::draw_stat(x, y + mh + sh * 0.030, adj_card_w, sh * 0.025,
                    &format!("Mod: {:?}", species.modifiers[0]), BLACK);

                // Buy button bottom-right of card
                let buy_w = adj_card_w * 0.4;
                let buy_h = sh * 0.03;
                let buy_x = x + adj_card_w - buy_w - sw * 0.003;
                let buy_y = y + card_inner_h - buy_h - sh * 0.003;
                if ui::draw_button_box(buy_x, buy_y, buy_w, buy_h,
                    BUTTON_ACTION, "Buy", BLACK) {
                    return hudAction::AddFish(index);
                }

                draw_rectangle_lines(x, y, adj_card_w, card_inner_h,
                    2.0, con::AREA_BORDER_COLOUR);
            }

            // ---- Right half: upgrades panel ----
            let up_left = sw * 0.52;
            let up_width = sw - up_left - content_pad;
            let up_top = content_top + sh * 0.002;

            // Section header
            draw_rectangle(up_left, up_top, up_width, sec_header_h, HEADER_STRIP);
            let uh_text = "Upgrades";
            let uhs = measure_text(uh_text, None, shf as u16, 1.0);
            draw_text(uh_text,
                up_left + sw * 0.01,
                up_top + sec_header_h * 0.5 + uhs.height * 0.5,
                shf, WHITE);

            // Two upgrade tiles side-by-side
            let tile_gap = sw * 0.01;
            let tile_w = (up_width - tile_gap) * 0.5;
            let tile_h = sh * 0.22;
            let tile_y = up_top + sec_header_h + sh * 0.01;

            // Food tile
            draw_rectangle(up_left, tile_y, tile_w, tile_h, PANEL_BG_ALT);
            draw_rectangle_lines(up_left, tile_y, tile_w, tile_h,
                2.0, con::AREA_BORDER_COLOUR);
            ui::draw_stat(up_left, tile_y + sh * 0.015, tile_w, sh * 0.035,
                "Upgrade Food", BLACK);
            ui::draw_stat(up_left, tile_y + sh * 0.055, tile_w, sh * 0.03,
                &format!("Level: {}", gameState.player.current_food_level), BLACK);
            let food_btn_w = tile_w * 0.7;
            let food_btn_h = sh * 0.05;
            if ui::draw_button_box(
                up_left + (tile_w - food_btn_w) * 0.5,
                tile_y + tile_h - food_btn_h - sh * 0.015,
                food_btn_w, food_btn_h,
                BUTTON_ACTION,
                &format!("Buy: {:.2}",
                    gameState.economy.get_food_cost(gameState.player.current_food_level)),
                BLACK,
            ) {
                return hudAction::BuyFood;
            }

            // Tank cap tile
            let cap_x = up_left + tile_w + tile_gap;
            draw_rectangle(cap_x, tile_y, tile_w, tile_h, PANEL_BG_ALT);
            draw_rectangle_lines(cap_x, tile_y, tile_w, tile_h,
                2.0, con::AREA_BORDER_COLOUR);
            ui::draw_stat(cap_x, tile_y + sh * 0.015, tile_w, sh * 0.035,
                "Upgrade Tank Cap", BLACK);
            ui::draw_stat(cap_x, tile_y + sh * 0.055, tile_w, sh * 0.03,
                &format!("Level: {}", gameState.player.tank_cap_level), BLACK);
            let cap_btn_w = tile_w * 0.7;
            if ui::draw_button_box(
                cap_x + (tile_w - cap_btn_w) * 0.5,
                tile_y + tile_h - food_btn_h - sh * 0.015,
                cap_btn_w, food_btn_h,
                BUTTON_ACTION,
                &format!("Buy: {:.2}",
                    gameState.economy.get_tank_cap_cost(gameState.player.tank_cap_level)),
                BLACK,
            ) {
                return hudAction::BuyTankCap;
            }
        },

        &BottomTab::Testing => {
            let selected = &gameState.fish_registry.fish[gameState.debugger.current_fish_debug_index];
            let selected_param = tank::WaterParameter::ALL[gameState.debugger.current_stat_debug_index];

            // Three logical clusters in equal columns
            let col_count = 3;
            let col_w = (sw - content_pad * 2.0 - sw * 0.02) / col_count as f32;
            let col_h = sh * 0.27;
            let col_top = content_top + sh * 0.015;
            let header_h = sh * 0.03;

            let cols_x = [
                content_pad,
                content_pad + col_w + sw * 0.01,
                content_pad + (col_w + sw * 0.01) * 2.0,
            ];

            let titles = ["Fish Spawner", "Water Stat Tuner", "Misc"];

            for i in 0..col_count {
                draw_rectangle(cols_x[i], col_top, col_w, col_h, PANEL_BG_ALT);
                draw_rectangle(cols_x[i], col_top, col_w, header_h, HEADER_STRIP);
                let f = header_h * 0.55;
                let ts = measure_text(titles[i], None, f as u16, 1.0);
                draw_text(titles[i],
                    cols_x[i] + col_w * 0.5 - ts.width * 0.5,
                    col_top + header_h * 0.5 + ts.height * 0.5,
                    f, WHITE);
            }

            // ---- Column 1: Fish Spawner ----
            let c1x = cols_x[0];
            let c1_inner_top = col_top + header_h + sh * 0.01;
            ui::draw_stat(c1x, c1_inner_top, col_w, sh * 0.035,
                &format!("Selected: {}", selected.species), BLACK);

            let arrow_w = col_w * 0.15;
            let arrow_h = sh * 0.05;
            let mid_y = c1_inner_top + sh * 0.06;

            if ui::draw_button_box(c1x + sw * 0.005, mid_y,
                arrow_w, arrow_h, BUTTON_NEUTRAL, "<", BLACK) {
                return hudAction::DebugIndexDecrease;
            }
            if ui::draw_button_box(c1x + col_w - arrow_w - sw * 0.005, mid_y,
                arrow_w, arrow_h, BUTTON_NEUTRAL, ">", BLACK) {
                return hudAction::DebugIndexIncrease;
            }

            let add_btn_w = col_w * 0.55;
            if ui::draw_button_box(
                c1x + (col_w - add_btn_w) * 0.5, mid_y,
                add_btn_w, arrow_h,
                BUTTON_ACTION, "Add Fish", BLACK,
            ) {
                return hudAction::TestAddFish(gameState.debugger.current_fish_debug_index);
            }

            // ---- Column 2: Water Stat Tuner ----
            let c2x = cols_x[1];
            let c2_inner_top = col_top + header_h + sh * 0.01;
            ui::draw_stat(c2x, c2_inner_top, col_w, sh * 0.035,
                &format!("Param: {:?}", selected_param), BLACK);
            ui::draw_stat(c2x, c2_inner_top + sh * 0.030, col_w, sh * 0.030,
                &format!("Dir: {}",
                    if gameState.debugger.stat_change_direction { "+" } else { "-" }),
                BLACK);

            let row1_y = c2_inner_top + sh * 0.07;
            if ui::draw_button_box(c2x + sw * 0.005, row1_y,
                arrow_w, arrow_h, BUTTON_NEUTRAL, "<", BLACK) {
                return hudAction::DebugShiftStatLeft;
            }
            if ui::draw_button_box(c2x + col_w - arrow_w - sw * 0.005, row1_y,
                arrow_w, arrow_h, BUTTON_NEUTRAL, ">", BLACK) {
                return hudAction::DebugShiftStatRight;
            }
            let change_w = col_w * 0.55;
            if ui::draw_button_box(
                c2x + (col_w - change_w) * 0.5, row1_y,
                change_w, arrow_h,
                BUTTON_ACTION, "Apply Change", BLACK,
            ) {
                return hudAction::TestChangeStat(selected_param,
                    gameState.debugger.stat_change_direction);
            }

            let row2_y = row1_y + arrow_h + sh * 0.01;
            let signbtn_w = col_w * 0.4;
            let gap = col_w * 0.04;
            let signs_total = signbtn_w * 2.0 + gap;
            let signs_start = c2x + (col_w - signs_total) * 0.5;

            if ui::draw_button_box(signs_start, row2_y,
                signbtn_w, arrow_h, BUTTON_DANGER, "- Direction", BLACK) {
                return hudAction::DebugShiftStatNegative;
            }
            if ui::draw_button_box(signs_start + signbtn_w + gap, row2_y,
                signbtn_w, arrow_h, BUTTON_ACTION, "+ Direction", BLACK) {
                return hudAction::DebugShiftStatPositive;
            }

            // ---- Column 3: Misc ----
            let c3x = cols_x[2];
            let c3_inner_top = col_top + header_h + sh * 0.01;
            ui::draw_stat(c3x, c3_inner_top, col_w, sh * 0.035,
                "Debug Helpers", BLACK);

            let misc_btn_w = col_w * 0.8;
            let misc_btn_h = sh * 0.045;
            let misc_x = c3x + (col_w - misc_btn_w) * 0.5;
            let misc_y1 = c3_inner_top + sh * 0.05;
            let misc_y2 = misc_y1 + misc_btn_h + sh * 0.01;
            let misc_y3 = misc_y2 + misc_btn_h + sh * 0.01;

            if ui::draw_button_box(misc_x, misc_y1, misc_btn_w, misc_btn_h,
                BUTTON_ACTION, "Add 1000 Prestige", BLACK) {
                return hudAction::AddPrestige;
            }
            if ui::draw_button_box(misc_x, misc_y2, misc_btn_w, misc_btn_h,
                BUTTON_NEUTRAL, "Toggle Light", BLACK) {
                return hudAction::TestToggleLight;
            }
            ui::draw_stat(c3x, misc_y3, col_w, sh * 0.03,
                "(more debug tools soon)", BLACK);

            // Borders on each column
            for i in 0..col_count {
                draw_rectangle_lines(cols_x[i], col_top, col_w, col_h,
                    3.0, con::AREA_BORDER_COLOUR);
            }
        },
    }

    // =====================================================================
    // NOTIFICATION POPUP (unchanged behavior, slightly nicer placement)
    // =====================================================================
    if gameState.notification.is_active() {
        ui::draw_centered_text_box(
            sw * 0.5, sh * 0.55,
            sw * 0.3, sh * 0.05,
            Color::from_rgba(0, 0, 0, 180),
            &gameState.notification.message,
            WHITE,
        );
    }

    hudAction::None
}