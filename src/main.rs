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
    // 迷路を初期化する関数

    // 2D ベクトルで迷路を表現。各セルは char 型で、' ' は空白を、'■' は柱を表す。
    let mut maze = vec![vec![' '; WIDTH]; HEIGHT];

    // 奇数の座標に柱を配置
    for i in (1..HEIGHT).step_by(2) {
        for j in (1..WIDTH).step_by(2) {
            maze[i][j] = '■'; // '■' は柱を表す
        }
    }
    // 初期化された迷路を返す
    maze
}


fn generate_walls(maze: &mut Vec<Vec<char>>) {
    // 柱からランダムな方向に壁を生成する関数

    // 迷路の奇数の座標にある柱からランダムな方向に壁を伸ばす
    for i in (1..HEIGHT - 1).step_by(2) {
        for j in (1..WIDTH - 1).step_by(2) {
            // 伸ばす方向をランダムに選択するための方向ベクトル
            let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
            
            // ランダムに方向を選択
            if let Some((dx, dy)) = directions.choose(&mut rand::thread_rng()) {
                // 新しい座標を計算
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


fn print_maze(maze: &Vec<Vec<char>>) {
    // 迷路をコンソールに出力する関数

    // 迷路を行ごとに走査
    for row in maze {
        // 行内の各セルを出力
        for cell in row {
            print!("{} ", cell);
        }
        println!(); // 行ごとに改行
    }

    // コンソールのバッファをフラッシュして即座に表示
    io::stdout().flush().unwrap();
}

