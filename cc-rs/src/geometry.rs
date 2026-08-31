use crate::events::fcl_button_has_payload;
use crate::utils::*;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
pub struct Rect {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

pub fn fcl_button_rect(button: &Value, aspect: f64) -> Rect {
    let base_info = button
        .get("baseInfo")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let screen_h = 10000.0f64;
    let screen_w = screen_h * 0.1f64.max(clamp_float(&gof_num(aspect), 16.0 / 9.0));

    let width: f64;
    let height: f64;
    if to_string_v(&get_or(&base_info, "sizeType", json!(""))) == "ABSOLUTE" {
        width =
            1.0f64.max(clamp_zl_dp(&get_or(&base_info, "absoluteWidth", inum(50)), 50.0) * 10.0);
        height = 1.0f64
            .max(clamp_zl_dp(&get_or(&base_info, "absoluteHeight", inum(50)), 50.0) * 10.0);
    } else {
        let pw = base_info
            .get("percentageWidth")
            .filter(|v| v.is_object())
            .cloned()
            .unwrap_or_else(obj);
        let ph = base_info
            .get("percentageHeight")
            .filter(|v| v.is_object())
            .cloned()
            .unwrap_or_else(obj);
        let width_ref = if to_string_v(&get_or(&pw, "reference", json!(""))) == "SCREEN_HEIGHT" {
            screen_h
        } else {
            screen_w
        };
        let height_ref = if to_string_v(&get_or(&ph, "reference", json!(""))) == "SCREEN_HEIGHT" {
            screen_h
        } else {
            screen_w
        };
        width = 1.0f64
            .max(width_ref * clamp_int(&get_or(&pw, "size", inum(50)), 50) as f64 / 1000.0);
        height = 1.0f64
            .max(height_ref * clamp_int(&get_or(&ph, "size", inum(50)), 50) as f64 / 1000.0);
    }

    let x = (screen_w - width) * clamp_int(&get_or(&base_info, "xPosition", inum(0)), 0) as f64
        / 1000.0;
    let y = (screen_h - height) * clamp_int(&get_or(&base_info, "yPosition", inum(0)), 0) as f64
        / 1000.0;
    Rect {
        x1: x,
        y1: y,
        x2: x + width,
        y2: y + height,
    }
}

pub fn rect_area(r: Rect) -> f64 {
    0.0f64.max(r.x2 - r.x1) * 0.0f64.max(r.y2 - r.y1)
}

pub fn screen_area(aspect: f64) -> f64 {
    10000.0 * 10000.0 * 0.1f64.max(clamp_float(&gof_num(aspect), 16.0 / 9.0))
}

pub fn fcl_button_area_ratio(button: &Value, aspect: f64) -> f64 {
    rect_area(fcl_button_rect(button, aspect)) / 1.0f64.max(screen_area(aspect))
}

fn rect_overlap_area(a: Rect, b: Rect) -> f64 {
    0.0f64.max(a.x2.min(b.x2) - a.x1.max(b.x1)) * 0.0f64.max(a.y2.min(b.y2) - a.y1.max(b.y1))
}

fn rect_center(r: Rect) -> (f64, f64) {
    ((r.x1 + r.x2) / 2.0, (r.y1 + r.y2) / 2.0)
}

fn rect_contains_point(r: Rect, px: f64, py: f64) -> bool {
    r.x1 <= px && px <= r.x2 && r.y1 <= py && py <= r.y2
}

fn rect_gap(a: Rect, b: Rect) -> (f64, f64) {
    let horizontal = 0.0f64.max(a.x1.max(b.x1) - a.x2.min(b.x2));
    let vertical = 0.0f64.max(a.y1.max(b.y1) - a.y2.min(b.y2));
    (horizontal, vertical)
}

fn rect_distance(a: Rect, b: Rect) -> f64 {
    let (horizontal, vertical) = rect_gap(a, b);
    horizontal.hypot(vertical)
}

pub fn same_visibility(a: &Value, b: &Value) -> bool {
    let a_base = a
        .get("baseInfo")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let b_base = b
        .get("baseInfo")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let mut a_vis = to_string_v(&get_or(&a_base, "visibilityType", json!("ALWAYS")));
    if a_vis.is_empty() {
        a_vis = "ALWAYS".to_string();
    }
    let mut b_vis = to_string_v(&get_or(&b_base, "visibilityType", json!("ALWAYS")));
    if b_vis.is_empty() {
        b_vis = "ALWAYS".to_string();
    }
    a_vis == b_vis
}

fn overlay_match_score(event_button: &Value, display_button: &Value, aspect: f64) -> f64 {
    let event_rect = fcl_button_rect(event_button, aspect);
    let display_rect = fcl_button_rect(display_button, aspect);
    let event_area = rect_area(event_rect);
    let display_area = rect_area(display_rect);
    if event_area <= 0.0 || display_area <= 0.0 {
        return 0.0;
    }

    let overlap = rect_overlap_area(event_rect, display_rect);
    let overlap_min = overlap / 1.0f64.max(event_area.min(display_area));
    let (display_center_x, display_center_y) = rect_center(display_rect);
    let display_center_in_event = rect_contains_point(event_rect, display_center_x, display_center_y);
    let (event_center_x, event_center_y) = rect_center(event_rect);
    let event_center_in_display = rect_contains_point(display_rect, event_center_x, event_center_y);
    let (horizontal_gap, vertical_gap) = rect_gap(event_rect, display_rect);
    let event_w = event_rect.x2 - event_rect.x1;
    let event_h = event_rect.y2 - event_rect.y1;
    let display_w = display_rect.x2 - display_rect.x1;
    let display_h = display_rect.y2 - display_rect.y1;
    let vertical_overlap = 0.0f64.max(event_rect.y2.min(display_rect.y2) - event_rect.y1.max(display_rect.y1))
        / 1.0f64.max(event_h.min(display_h));
    let horizontal_overlap = 0.0f64.max(event_rect.x2.min(display_rect.x2) - event_rect.x1.max(display_rect.x1))
        / 1.0f64.max(event_w.min(display_w));

    if overlap_min >= 0.25 || display_center_in_event || event_center_in_display {
        let mut score = 100.0 + overlap_min * 100.0;
        if display_center_in_event {
            score += 25.0;
        }
        if event_center_in_display {
            score += 10.0;
        }
        return score;
    }

    let max_w = event_w.max(display_w);
    let max_h = event_h.max(display_h);
    if vertical_overlap >= 0.65 && horizontal_gap <= 250.0f64.max(max_w * 0.25) {
        return 40.0 + vertical_overlap * 20.0 - horizontal_gap / 1.0f64.max(max_w);
    }
    if horizontal_overlap >= 0.65 && vertical_gap <= 250.0f64.max(max_h * 0.25) {
        return 40.0 + horizontal_overlap * 20.0 - vertical_gap / 1.0f64.max(max_h);
    }
    0.0
}

pub fn match_fcl_overlay_buttons(buttons: &[&Value], aspect: f64) -> (HashMap<usize, usize>, HashSet<usize>) {
    let mut display_indices: Vec<usize> = Vec::new();
    let mut event_indices: Vec<usize> = Vec::new();
    for (i, button) in buttons.iter().enumerate() {
        let text = to_string_v(&get_or(button, "text", json!("")));
        if !fcl_button_has_payload(button) && !text.trim().is_empty() {
            display_indices.push(i);
        }
        if fcl_button_has_payload(button) && text.trim().is_empty() {
            event_indices.push(i);
        }
    }
    let mut matches: HashMap<usize, usize> = HashMap::new();
    let mut consumed: HashSet<usize> = HashSet::new();

    for &event_index in &event_indices {
        let event_button = buttons[event_index];
        let mut best_index: isize = -1;
        let mut best_score = 0.0f64;
        for &display_index in &display_indices {
            if consumed.contains(&display_index) {
                continue;
            }
            let display_button = buttons[display_index];
            if !same_visibility(event_button, display_button) {
                continue;
            }
            let score = overlay_match_score(event_button, display_button, aspect);
            if score > best_score {
                best_score = score;
                best_index = display_index as isize;
            }
        }
        if best_index >= 0 && best_score >= 40.0 {
            matches.insert(event_index, best_index as usize);
            consumed.insert(best_index as usize);
        }
    }
    (matches, consumed)
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GridSignature {
    pub style: String,
    pub width: i64,
    pub height: i64,
    pub visibility: String,
}

fn fcl_button_grid_signature(button: &Value) -> GridSignature {
    let base_info = button
        .get("baseInfo")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let pw = base_info
        .get("percentageWidth")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    let ph = base_info
        .get("percentageHeight")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    GridSignature {
        style: to_string_v(&get_or(button, "style", json!(""))),
        width: clamp_int(&get_or(&pw, "size", inum(0)), 0),
        height: clamp_int(&get_or(&ph, "size", inum(0)), 0),
        visibility: to_string_v(&get_or(&base_info, "visibilityType", json!("ALWAYS"))),
    }
}

pub fn inferable_grid_indices(buttons: &[&Value]) -> HashSet<usize> {
    let mut buckets: HashMap<GridSignature, Vec<usize>> = HashMap::new();
    for (i, button) in buttons.iter().enumerate() {
        let text = to_string_v(&get_or(button, "text", json!("")));
        if fcl_button_has_payload(button) || text.trim().is_empty() {
            continue;
        }
        let sig = fcl_button_grid_signature(button);
        if sig.width <= 0 || sig.height <= 0 {
            continue;
        }
        buckets.entry(sig).or_default().push(i);
    }
    let mut result = HashSet::new();
    for indices in buckets.values() {
        if indices.len() >= 4 {
            for &idx in indices {
                result.insert(idx);
            }
        }
    }
    result
}

pub struct GroupMatch {
    pub prefix_score: i64,
    pub normalized_len: usize,
    pub candidate_len: usize,
    pub group_id: String,
}

pub fn infer_events_from_group_names(
    button: &Value,
    group_ids_by_name: &GroupIdsByName,
    group_name: &str,
) -> Vec<Value> {
    let text = to_string_v(&get_or(button, "text", json!("")));
    let text_words = normalized_control_words(&text);
    let normalized_text = normalized_control_text(&text);
    if text_words.is_empty() && normalized_text.is_empty() {
        return Vec::new();
    }

    let mut matches: Vec<GroupMatch> = Vec::new();
    let group_prefix = normalized_control_text(group_name);
    for (candidate_name, group_id) in group_ids_by_name.iter() {
        if group_id.is_empty() || candidate_name == group_name {
            continue;
        }
        let candidate_words = normalized_control_words(candidate_name);
        let normalized_candidate = normalized_control_text(candidate_name);
        if candidate_words.is_empty() && normalized_candidate.is_empty() {
            continue;
        }
        let mut candidate_subset = false;
        if !candidate_words.is_empty() {
            candidate_subset = true;
            for w in &candidate_words {
                if !text_words.contains(w) {
                    candidate_subset = false;
                    break;
                }
            }
        }
        if candidate_subset
            || (!normalized_candidate.is_empty() && normalized_text.contains(&normalized_candidate))
        {
            let mut prefix_score = 0;
            if !group_prefix.is_empty() && normalized_candidate.starts_with(&group_prefix) {
                prefix_score = 1;
            }
            matches.push(GroupMatch {
                prefix_score,
                normalized_len: normalized_candidate.chars().count(),
                candidate_len: candidate_name.chars().count(),
                group_id: group_id.clone(),
            });
        }
    }

    matches.sort_by(|a, b| {
        (a.prefix_score, a.normalized_len, a.candidate_len)
            .cmp(&(b.prefix_score, b.normalized_len, b.candidate_len))
            .reverse()
    });
    if matches.is_empty() {
        return Vec::new();
    }
    let result = vec![json!({
        "type": "switch_layer",
        "key": matches[0].group_id,
    })];
    dedupe_events(result)
}

pub fn event_bind_targets(button: &Value) -> HashSet<String> {
    let mut targets = HashSet::new();
    let event_root = button
        .get("event")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    for event_name in ["pressEvent", "clickEvent", "doubleClickEvent", "longPressEvent"] {
        let event = event_root
            .get(event_name)
            .filter(|v| v.is_object())
            .cloned()
            .unwrap_or_else(obj);
        for group_id in get_or_list(&event, "bindViewGroup") {
            targets.insert(to_string_v(group_id));
        }
    }
    targets
}

pub fn layer_event_targets(group: &Value) -> HashSet<String> {
    let mut targets = HashSet::new();
    let view_data = group
        .get("viewData")
        .filter(|v| v.is_object())
        .cloned()
        .unwrap_or_else(obj);
    if let Some(items) = view_data.get("buttonList").and_then(|v| v.as_array()) {
        for item in items {
            if item.is_object() {
                for t in event_bind_targets(item) {
                    targets.insert(t);
                }
            }
        }
    }
    targets
}

struct OpenerScore {
    score: f64,
    target_id: String,
}

pub fn infer_reciprocal_layer_openers(data: &Value, aspect: f64) -> HashMap<String, String> {
    let mut groups: Vec<&Value> = Vec::new();
    for item in get_or_list(data, "viewGroups") {
        if item.is_object() {
            groups.push(item);
        }
    }
    let mut opener_scores: HashMap<String, OpenerScore> = HashMap::new();

    let mut group_index: HashMap<String, usize> = HashMap::new();
    for (i, group) in groups.iter().enumerate() {
        group_index.insert(to_string_v(&get_or(group, "id", json!(""))), i);
    }
    let mut group_ids_by_name = GroupIdsByName::new();
    for group in &groups {
        let id = to_string_v(&get_or(group, "id", json!("")));
        if id.is_empty() {
            continue;
        }
        let name = to_string_v(&get_or(group, "name", json!("Layer")));
        group_ids_by_name.insert(name, id);
    }
    let mut targets_by_group_id: HashMap<String, HashSet<String>> = HashMap::new();
    for group in &groups {
        let id = to_string_v(&get_or(group, "id", json!("")));
        targets_by_group_id.insert(id.clone(), layer_event_targets(group));
    }

    for source_group in &groups {
        let source_id = to_string_v(&get_or(source_group, "id", json!("")));
        let view_data = source_group
            .get("viewData")
            .filter(|v| v.is_object())
            .cloned()
            .unwrap_or_else(obj);
        let mut source_buttons: Vec<&Value> = Vec::new();
        if let Some(items) = view_data.get("buttonList").and_then(|v| v.as_array()) {
            for item in items {
                if item.is_object() {
                    source_buttons.push(item);
                }
            }
        }
        let mut candidates: Vec<&Value> = Vec::new();
        for button in &source_buttons {
            let text = to_string_v(&get_or(button, "text", json!("")));
            if !fcl_button_has_payload(button) && !text.trim().is_empty() && fcl_button_area_ratio(button, aspect) < 0.05 {
                candidates.push(button);
            }
        }
        if candidates.is_empty() {
            continue;
        }

        for candidate in &candidates {
            let source_name = to_string_v(&get_or(source_group, "name", json!("")));
            let inferred_events = infer_events_from_group_names(candidate, &group_ids_by_name, &source_name);
            for event in &inferred_events {
                let target_id = to_string_v(&get_or(event, "key", json!("")));
                if !target_id.is_empty() && target_id != source_id {
                    let index_distance =
                        group_index.get(&target_id).copied().unwrap_or(0) as i64
                            - group_index.get(&source_id).copied().unwrap_or(0) as i64;
                    let index_distance = index_distance.abs();
                    let button_id = to_string_v(&get_or(candidate, "id", json!("")));
                    let score = index_distance as f64 * 10000.0 - 1.0;
                    let better = match opener_scores.get(&button_id) {
                        None => true,
                        Some(prev) => score < prev.score,
                    };
                    if better {
                        opener_scores.insert(
                            button_id,
                            OpenerScore {
                                score,
                                target_id: target_id.clone(),
                            },
                        );
                    }
                }
            }
        }

        for target_group in &groups {
            let target_id = to_string_v(&get_or(target_group, "id", json!("")));
            if target_id.is_empty() || target_id == source_id {
                continue;
            }
            if to_string_v(&get_or(target_group, "visibility", json!(""))) != "INVISIBLE" {
                continue;
            }
            let source_name = to_string_v(&get_or(source_group, "name", json!("")));
            let target_name = to_string_v(&get_or(target_group, "name", json!("")));
            let source_words = normalized_control_words(&source_name);
            let target_words = normalized_control_words(&target_name);
            let source_targets = targets_by_group_id.get(&source_id);
            let mut has_intersection = false;
            if !source_words.is_empty() && !target_words.is_empty() {
                for w in &source_words {
                    if target_words.contains(w) {
                        has_intersection = true;
                        break;
                    }
                }
            }
            if has_intersection {
                match source_targets {
                    Some(st) if !st.contains(&target_id) => continue,
                    None => continue,
                    _ => {}
                }
            }

            let view_data = target_group
                .get("viewData")
                .filter(|v| v.is_object())
                .cloned()
                .unwrap_or_else(obj);
            let mut target_buttons: Vec<&Value> = Vec::new();
            if let Some(items) = view_data.get("buttonList").and_then(|v| v.as_array()) {
                for item in items {
                    if item.is_object() {
                        target_buttons.push(item);
                    }
                }
            }
            let mut close_buttons: Vec<&Value> = Vec::new();
            for button in &target_buttons {
                let bind_targets = event_bind_targets(button);
                let has_source = bind_targets.contains(&source_id);
                let has_target = bind_targets.contains(&target_id);
                let ratio = fcl_button_area_ratio(button, aspect);
                if has_source && has_target && (0.08..=0.50).contains(&ratio) {
                    close_buttons.push(button);
                }
            }
            if close_buttons.is_empty() {
                continue;
            }

            let mut best_candidate: Option<&Value> = None;
            let mut best_distance = f64::INFINITY;
            for candidate in &candidates {
                let candidate_rect = fcl_button_rect(candidate, aspect);
                for close_button in &close_buttons {
                    let distance = rect_distance(candidate_rect, fcl_button_rect(close_button, aspect));
                    if distance < best_distance {
                        best_distance = distance;
                        best_candidate = Some(candidate);
                    }
                }
            }
            if let Some(best_candidate) = best_candidate {
                if best_distance <= 500.0 {
                    let button_id = to_string_v(&get_or(best_candidate, "id", json!("")));
                    let index_distance =
                        group_index.get(&target_id).copied().unwrap_or(0) as i64
                            - group_index.get(&source_id).copied().unwrap_or(0) as i64;
                    let index_distance = index_distance.abs();
                    let score = index_distance as f64 * 10000.0 + best_distance;
                    let better = match opener_scores.get(&button_id) {
                        None => true,
                        Some(prev) => score < prev.score,
                    };
                    if better {
                        opener_scores.insert(
                            button_id,
                            OpenerScore {
                                score,
                                target_id: target_id.clone(),
                            },
                        );
                    }
                }
            }
        }
    }

    let mut result = HashMap::new();
    for (button_id, v) in opener_scores {
        result.insert(button_id, v.target_id);
    }
    result
}

pub fn abs_int(x: i64) -> i64 {
    x.abs()
}

pub fn infer_builtin_menu_events(button: &Value) -> Vec<Value> {
    let text = normalized_control_text(&to_string_v(&get_or(button, "text", json!(""))));
    if text == "fcl菜单" || text == "菜单" {
        return vec![json!({
            "type": "launcher_event",
            "key": "launcher.event.switch_menu",
        })];
    }
    if text == "输入法" || text == "输入文字" {
        return vec![json!({
            "type": "launcher_event",
            "key": "launcher.event.switch_ime",
        })];
    }
    if text == "社交" {
        return vec![json!({
            "type": "key",
            "key": "GLFW_KEY_P",
        })];
    }
    if text == "聊天" {
        return vec![json!({
            "type": "key",
            "key": "GLFW_KEY_T",
        })];
    }
    Vec::new()
}
