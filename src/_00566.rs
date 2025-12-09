use crate::Solution;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();

        if (m * n) as i32 != r * c {
            return mat;
        }

        let mut ans = vec![vec![0i32; c as usize]; r as usize];
        for x in 0..m * n {
            ans[x / c as usize][x % c as usize] = mat[x / n][x % n];
        }

        ans
    }
}

fn print_matrix_aligned<T: std::fmt::Display>(mat: &Vec<Vec<T>>) {
    if mat.is_empty() || mat[0].is_empty() {
        println!("[[]]");
        return;
    }

    let rows = mat.len();
    let cols = mat[0].len();

    // 1. 计算每列的最大宽度，并计算总的打印宽度
    let mut col_widths: Vec<usize> = vec![0; cols];
    for j in 0..cols {
        for i in 0..rows {
            let width = mat[i][j].to_string().len();
            col_widths[j] = std::cmp::max(col_widths[j], width);
        }
    }

    // 计算元素打印的总字符宽度：所有列宽度之和 + (列数 - 1) 个元素间距
    let total_content_width = col_widths.iter().sum::<usize>() + (cols - 1);

    // 2. 打印矩阵上边界
    // 边界线的长度需要覆盖内容和两侧的空格（如果需要）
    println!("┌{}┐", "─".repeat(total_content_width + 2));

    // 3. 格式化并打印输出
    for row in mat.iter() {
        print!("| "); // 左边界 + 1 个空格

        for (j, element) in row.iter().enumerate() {
            let width = col_widths[j];
            let s = element.to_string();

            // 打印元素，使用 {:>width$} 进行右对齐
            print!("{:>width$}", s, width = width);

            // 只有不是最后一列时，才添加元素分隔空格
            if j < cols - 1 {
                print!(" ");
            }
        }

        // 打印右侧空格和右边界
        println!(" |");
    }

    // 4. 打印矩阵下边界
    println!("└{}┘", "─".repeat(total_content_width + 2));
}
#[test]
fn test_matrix_reshape() {
    let m = 5;
    let n = 2;

    let matrix = vec![vec![0; n]; m];

    print_matrix_aligned(&matrix);
}


use std::f64::consts::PI;

fn plot_sin_x_horizontal() {
    // 1. 定义画布参数
    let width = 80;  // 终端绘图区域的宽度 (x 轴)
    let height = 20; // 终端绘图区域的高度 (y 轴)

    // Y 轴的中心线（约第 10 行）
    let y_center = height / 2;

    // 振幅的缩放因子：将 [-1, 1] 映射到 [0, height/2]
    let y_scale = (height / 2) as f64;

    // X 轴的步长，使曲线覆盖约 3 个周期 (3 * 2 * PI)
    let x_max = 3.0 * 2.0 * PI;
    let x_step = x_max / (width as f64);

    // 2. 初始化画布（字符矩阵）
    // Vec<Vec<char>>: height 行, width 列, 初始填充空格
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; width]; height];

    // 3. 计算并标记所有点
    for col in 0..width {
        // 计算 x 值
        let x = col as f64 * x_step;

        // 计算 y 值
        let y = x.sin();

        // 将 y 值映射到行索引 (row_index)。注意：行索引 0 是顶部。
        // 映射: y_center - (y * y_scale)
        // 当 y=1 时，row_index = y_center - y_scale (顶部)
        // 当 y=-1 时，row_index = y_center + y_scale (底部)
        let mut row_index = (y_center as f64 - y * y_scale).round() as usize;

        // 确保行索引在边界内 [0, height - 1]
        row_index = row_index.min(height - 1).max(0);

        // 标记点
        canvas[row_index][col] = '*';

        // 绘制 X 轴中心线 (可选)
        if y_center < height && col % 5 == 0 {
            canvas[y_center][col] = '-';
        }
    }

    // 4. 打印画布
    println!("\n--- 绘制 sin(x) 水平图表 ({rows}x{cols}) ---", rows=height, cols=width);

    // 打印上边界
    println!(" {}", "═".repeat(width + 2));

    for row in canvas {
        // 将 Vec<char> 转换为 String 并打印
        let line: String = row.into_iter().collect();
        println!("║ {} ║", line);
    }

    // 打印下边界
    println!(" {}", "═".repeat(width + 2));
}

#[test]
fn test_plot_sin_x() {
    plot_sin_x_horizontal();
}

