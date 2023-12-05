use std::collections::VecDeque;
use std::io::{self, Write};
use rand::seq::SliceRandom;

const WIDTH: usize = 11;
const HEIGHT: usize = 11;
const START: char = 'S';
const GOAL: char = 'G';
const EMPTY: char = ' ';
const WALL: char = '■';
const PATH: char = 'o';

// 方向を表す構造体
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    // 迷路を柱と壁で初期化
    let mut maze = initialize_maze();

    // 柱から壁を生成
    generate_walls(&mut maze);

    // 迷路を表示
    print_maze(&maze);

    // 解を見つけて表示
    find_and_print_solution(&mut maze);
}

// 迷路の初期化
fn initialize_maze() -> Vec<Vec<char>> {
    // 2Dベクトルで迷路を初期化。各セルは ' '（空白）または '■'（柱）で初期化される。
    let mut maze = vec![vec![' '; WIDTH]; HEIGHT];

    // 奇数の座標に柱を配置
    for i in (1..HEIGHT).step_by(2) {
        for j in (1..WIDTH).step_by(2) {
            maze[i][j] = '■'; // '■' は柱を表す
        }
    }

    // スタートとゴールの位置を設定
    maze[0][0] = 'S';
    maze[HEIGHT - 1][WIDTH - 1] = 'G';

    maze
}

// 柱からランダムに壁を生成
fn generate_walls(maze: &mut Vec<Vec<char>>) {
    for i in (1..HEIGHT - 1).step_by(2) {
        for j in (1..WIDTH - 1).step_by(2) {
            // 壁を伸ばす方向の選択肢
            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

            // ランダムに方向を選択
            if let Some((dx, dy)) = directions.choose(&mut rand::thread_rng()) {
                let ni = i as isize + dx;
                let nj = j as isize + dy;

                // 新しい座標が範囲内であれば、そこに壁を設定
                if ni >= 0 && nj >= 0 && ni < HEIGHT as isize && nj < WIDTH as isize {
                    maze[ni as usize][nj as usize] = '■'; // '■' は壁を表す
                }
            }
        }
    }
}

// 解を見つけて表示
fn find_and_print_solution(maze: &mut Vec<Vec<char>>) {
    // スタート地点
    let start = (0, 0);
    // ゴール地点
    let goal = (HEIGHT - 1, WIDTH - 1);

    // スタート地点から開始
    let mut current_position = start;

    // スタートからゴールまでの最終経路を表示
    print_maze(maze);
    if current_position == goal {
        println!("Solution found!");
    }
}

// 次の座標を取得するヘルパー関数
fn get_next_position((i, j): (usize, usize), direction: Direction) -> (usize, usize) {
    match direction {
        Direction::Up => if i > 0 { (i - 1, j) } else { (i, j) },
        Direction::Down => (i + 1, j),
        Direction::Left => if j > 0 { (i, j - 1) } else { (i, j) },
        Direction::Right => (i, j + 1),
    }
}

// 移動が有効かどうかを判定するヘルパー関数
fn is_valid_move((i, j): (usize, usize), maze: &Vec<Vec<char>>) -> bool {
    i < HEIGHT && j < WIDTH && maze[i][j] != '■' && maze[i][j] != 'o'
}

// 迷路を表示
fn print_maze(maze: &Vec<Vec<char>>) {
    // 上の境界を表示
    println!("{}", "=".repeat((WIDTH + 2) * 2));

    for row in maze {
        print!("||"); // 左の境界

        for cell in row {
            print!("{} ", cell);
        }

        println!("||"); // 右の境界
    }

    // 下の境界を表示
    println!("{}", "=".repeat((WIDTH + 2) * 2));

    // コンソールのバッファをフラッシュして即座に表示
    io::stdout().flush().unwrap();
}
