use macroquad::prelude::*;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Wall,
    Start,
    End,
    Visited,
    Path,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pathfinder Visualizer".to_string(),
        window_width: 800,
        window_height: 800,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid_size: usize = 20;
    let mut grid = vec![vec![CellType::Empty; grid_size]; grid_size];
    let mut start_pos: Option<(usize, usize)> = None;
    let mut end_pos: Option<(usize, usize)> = None;

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut parent = vec![vec![None::<(usize, usize)>; grid_size]; grid_size];
    let mut searching = false;
    let mut path_found = false;

    loop {
        clear_background(BLACK);

        let grid_size_px = screen_width().min(screen_height());
        let cell_size = grid_size_px / grid_size as f32;

        let offset_x = (screen_width() - grid_size_px) / 2.0;
        let offset_y = (screen_height() - grid_size_px) / 2.0;

        let super_down = is_key_down(KeyCode::LeftSuper) || is_key_down(KeyCode::RightSuper);

        if super_down && (is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd)) {
            grid_size += 1;
            grid = vec![vec![CellType::Empty; grid_size]; grid_size];
            parent = vec![vec![None; grid_size]; grid_size];
            queue.clear();
            start_pos = None;
            end_pos = None;
            searching = false;
            path_found = false;
        }

        if super_down && (is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract)) {
            if grid_size > 20 {
                grid_size -= 1;
                grid = vec![vec![CellType::Empty; grid_size]; grid_size];
                parent = vec![vec![None; grid_size]; grid_size];
                queue.clear();
                start_pos = None;
                end_pos = None;
                searching = false;
                path_found = false;
            }
        }

        if is_key_pressed(KeyCode::Space) && start_pos.is_some() && end_pos.is_some() && !path_found {
            if !searching && queue.is_empty() {
                let start = start_pos.unwrap();
                queue.push_back(start);
            }
            searching = !searching;
        }

        if is_key_pressed(KeyCode::R) {
            searching = false;
            path_found = false;
            queue.clear();
            parent = vec![vec![None; grid_size]; grid_size];

            for x in 0..grid_size {
                for y in 0..grid_size {
                    if grid[x][y] == CellType::Visited || grid[x][y] == CellType::Path {
                        grid[x][y] = CellType::Empty;
                    }
                }
            }
        }

        if is_key_pressed(KeyCode::C) {
            searching = false;
            path_found = false;
            queue.clear();
            parent = vec![vec![None; grid_size]; grid_size];
            grid = vec![vec![CellType::Empty; grid_size]; grid_size];
            start_pos = None;
            end_pos = None;
        }

        if !searching {
            if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
                let (mouse_x, mouse_y) = mouse_position();

                let relative_x = mouse_x - offset_x;
                let relative_y = mouse_y - offset_y;

                if relative_x >= 0.0 && relative_y >= 0.0 {
                    let grid_x = (relative_x / cell_size) as usize;
                    let grid_y = (relative_y / cell_size) as usize;

                    if grid_x < grid_size && grid_y < grid_size {
                        if is_mouse_button_down(MouseButton::Left) {
                            if is_key_down(KeyCode::S) {
                                if let Some((old_x, old_y)) = start_pos {
                                    grid[old_x][old_y] = CellType::Empty;
                                }
                                grid[grid_x][grid_y] = CellType::Start;
                                start_pos = Some((grid_x, grid_y));
                            } else if is_key_down(KeyCode::E) {
                                if let Some((old_x, old_y)) = end_pos {
                                    grid[old_x][old_y] = CellType::Empty;
                                }
                                grid[grid_x][grid_y] = CellType::End;
                                end_pos = Some((grid_x, grid_y));
                            } else if grid[grid_x][grid_y] == CellType::Empty {
                                grid[grid_x][grid_y] = CellType::Wall;
                            }
                        } else if is_mouse_button_down(MouseButton::Right) {
                            if grid[grid_x][grid_y] == CellType::Start {
                                start_pos = None;
                            } else if grid[grid_x][grid_y] == CellType::End {
                                end_pos = None;
                            }
                            grid[grid_x][grid_y] = CellType::Empty;
                        }
                    }
                }
            }
        }

        if searching {
            let steps_per_frame = (grid_size * grid_size) / 200 + 1;

            for _ in 0..steps_per_frame {
                if queue.is_empty() {
                    break;
                }

                let current = queue.pop_front().unwrap();
                let (cx, cy) = current;

                if Some(current) == end_pos {
                    searching = false;
                    path_found = true;
                    let mut curr = parent[cx][cy];
                    while let Some((px, py)) = curr {
                        if Some((px, py)) == start_pos {
                            break;
                        }
                        grid[px][py] = CellType::Path;
                        curr = parent[px][py];
                    }
                    break;
                } else {
                    if grid[cx][cy] != CellType::Start {
                        grid[cx][cy] = CellType::Visited;
                    }

                    let neighbors = [
                        (cx as i32 + 1, cy as i32),
                        (cx as i32 - 1, cy as i32),
                        (cx as i32, cy as i32 + 1),
                        (cx as i32, cy as i32 - 1),
                    ];

                    for (nx, ny) in neighbors {
                        if nx >= 0 && nx < grid_size as i32 && ny >= 0 && ny < grid_size as i32 {
                            let nx = nx as usize;
                            let ny = ny as usize;

                            if (grid[nx][ny] == CellType::Empty || grid[nx][ny] == CellType::End)
                                && parent[nx][ny].is_none()
                                && Some((nx, ny)) != start_pos
                            {
                                parent[nx][ny] = Some((cx, cy));
                                queue.push_back((nx, ny));
                            }
                        }
                    }
                }
            }
        }

        for x in 0..grid_size {
            for y in 0..grid_size {
                let rect_x = offset_x + (x as f32 * cell_size);
                let rect_y = offset_y + (y as f32 * cell_size);

                match grid[x][y] {
                    CellType::Wall => draw_rectangle(rect_x, rect_y, cell_size, cell_size, GRAY),
                    CellType::Start => draw_rectangle(rect_x, rect_y, cell_size, cell_size, GREEN),
                    CellType::End => draw_rectangle(rect_x, rect_y, cell_size, cell_size, RED),
                    CellType::Visited => draw_rectangle(rect_x, rect_y, cell_size, cell_size, SKYBLUE),
                    CellType::Path => draw_rectangle(rect_x, rect_y, cell_size, cell_size, YELLOW),
                    CellType::Empty => {},
                }

                draw_rectangle_lines(rect_x, rect_y, cell_size, cell_size, 1.0, DARKGRAY);
            }
        }

        next_frame().await;
    }
}