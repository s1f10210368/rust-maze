use std::io::{self, Write};
use rand::seq::SliceRandom; // rand クレートから SliceRandom トレイトを使用

const WIDTH: usize = 11;
const HEIGHT: usize = 11;

fn main() {
    // 迷路を柱で初期化
    let mut maze = initialize_maze();

    // 柱から壁を生成
    generate_walls(&mut maze);

    // 迷路を表示
    print_maze(&maze);
}

fn initialize_maze() -> Vec<Vec<char>> {
    let mut maze = vec![vec![' '; WIDTH]; HEIGHT];

    // 奇数の座標に柱を配置
    for i in (1..HEIGHT).step_by(2) {
        for j in (1..WIDTH).step_by(2) {
            maze[i][j] = '■'; // 'P' は柱を表す
        }
    }
    maze
}

fn generate_walls(maze: &mut Vec<Vec<char>>) {
    for i in (1..HEIGHT - 1).step_by(2) {
        for j in (1..WIDTH - 1).step_by(2) {
            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            if let Some((dx, dy)) = directions.choose(&mut rand::thread_rng()) {
                let ni = i as isize + dx;
                let nj = j as isize + dy;
                if ni >= 0 && nj >= 0 && ni < HEIGHT as isize && nj < WIDTH as isize {
                    maze[ni as usize][nj as usize] = '■'; // 'W' は壁を表す
                }
            }
        }
    }
}

fn print_maze(maze: &Vec<Vec<char>>) {
    for row in maze {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
    io::stdout().flush().unwrap();
}
