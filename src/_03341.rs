use crate::Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_time_to_reach_3341(move_time: Vec<Vec<i32>>) -> i32 {
        let n = move_time.len();
        let m = move_time[0].len();

        let inf = i32::MAX / 2;
        let mut d = vec![vec![inf; m]; n];
        let mut v = vec![vec![false; m]; n];
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        d[0][0] = 0;
        let mut queue = BinaryHeap::new();
        queue.push(Reverse((0, 0usize, 0usize)));

        while let Some(Reverse((dis, x, y))) = queue.pop() {
            if v[x][y] {
                continue;
            }

            v[x][y] = true;
            for &(dx, dy) in dirs.iter() {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx < 0 || nx >= n as isize || ny < 0 || ny >= m as isize {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;
                let dist = d[x][y].max(move_time[nx][ny]) + 1;
                if d[nx][ny] > dist {
                    d[nx][ny] = dist;
                    queue.push(Reverse((dist, nx, ny)));
                }
            }
        }

        d[n - 1][m - 1]
    }
}

#[test]
fn test_01() {
    let mut max_heap = BinaryHeap::new();

    // push 将元素插入堆中，堆会自动调整以保持最大堆的性质
    max_heap.push(1);
    max_heap.push(2);
    max_heap.push(5);
    max_heap.push(99);
    max_heap.push(3);
    max_heap.push(4);
    max_heap.push(5);

    // peek 返回堆顶的元素，
    println!("peek top: {:?}", max_heap.peek());

    // pop 移除并返回堆顶元素，元素按从大到小的顺序被移除
    while let Some(top) = max_heap.pop() {
        println!("pop top: {:?}", top);
    }
}

#[test]
fn test_02() {
    let mut min_heap = BinaryHeap::new();

    min_heap.push(Reverse(1));
    min_heap.push(Reverse(200));
    min_heap.push(Reverse(-91));
    min_heap.push(Reverse(1));
    min_heap.push(Reverse(32));

    println!("peek top: {:?}", min_heap.peek());

    while let Some(top) = min_heap.pop() {
        println!("pop top: {:?}", top);
    }
}

#[test]
fn test_03() {
    let mut task_queue = BinaryHeap::new();

    // 优先级队列，按优先级执行任务，数值越小，优先级越高
    task_queue.push(Reverse((2, "write report")));
    task_queue.push(Reverse((1, "handle email")));
    task_queue.push(Reverse((3, "open meeting")));

    while let Some(top) = task_queue.pop() {
        println!("pop top: {:?}", top);
    }
}
