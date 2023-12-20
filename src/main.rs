use std::{thread, time};
use std::io;
use std::io::Write;
use rand::prelude::SliceRandom;

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

    let mut maze = initialize_maze();

    generate_walls(&mut maze);
    
    print_maze(&maze);
    
    let start_i = 0;
    let start_j = 0;
    
    if search_maze(&mut maze, start_i, start_j) {
        println!("ゴールに到達!");
    } else {
        println!("ゴールに到達できず..."); 
    }

}

// 迷路の初期化
fn initialize_maze() -> Vec<Vec<char>> {
    // 2Dベクトルで迷路を初期化。各セルは ' '（空白）または '■'（柱）で初期化される。
    let mut maze = vec![vec![' '; WIDTH]; HEIGHT];

    // 奇数の座標に柱を配置
    for i in (1..HEIGHT).step_by(2) {
        for j in (1..WIDTH).step_by(2) {
            maze[i][j] = '◆'; // '■' は柱を表す
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


fn search_maze(maze: &mut Vec<Vec<char>>, i: usize, j: usize) -> bool {
    if i < 0 || i >= maze.len() || j < 0 || j >= maze[0].len() {
        return false; 
    }

    if maze[i][j] == GOAL {
        maze[i][j] = PATH;
        return true; 
    }

    if maze[i][j] != EMPTY {
        return false;
    }

    maze[i][j] = PATH;

    print_maze(maze);
    thread::sleep(time::Duration::from_millis(100));

    if search_maze(maze, i - 1, j) 
        || search_maze(maze, i + 1, j)
        || search_maze(maze, i, j - 1) 
        || search_maze(maze, i , j + 1) {
        
        return true;
    }

    maze[i][j] = EMPTY;
    return false;
}