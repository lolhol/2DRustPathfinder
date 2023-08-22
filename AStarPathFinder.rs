use rand::Rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{SystemTime, UNIX_EPOCH};

const IS_AUTO_GEN: bool = false;

// WARNING -> AUTO GEN WILL NOT MAKE SURE THAT YOU HAVE A PATH BETWEEN TWO POINTS!
// IT WILL ONLY RANDOMLY GEN THE GRID!

// Auto gen options
const MAX_MIN_COL: Vec2 = Vec2 { x: 9, y: 9 };
const MAX_MIN_ROW: Vec2 = Vec2 { x: 2, y: 9 };
const OBSTRUCTED_PERCENT: f64 = 40.0;

// might slow down time (idk tho depends)
const IS_PRINT_GRAPH: bool = true;
const IS_PRINT_GRAPH_END: bool = true;

////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy)]
struct Node {
    is_open: bool,
    is_closed: bool,

    coord: Vec2,
    parent: Vec2,
}

////////////////////////////////////

#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        return self.is_same(*other);
    }
}

impl Vec2 {
    fn is_same(&self, vec2: Vec2) -> bool {
        return self.x == vec2.x && self.y == vec2.y;
    }
}

////////////////////////////////////

#[derive(Debug, Clone, Copy)]
struct Vec2Weighted {
    x: i32,
    y: i32,
    h: f64,
}

// These next thingys is like the mumbo jumbo for the auto sorted array thing (i forgor name :/)

impl Vec2Weighted {
    fn to_vec2(&self) -> Vec2 {
        return Vec2 {
            x: self.x,
            y: self.y,
        };
    }
}

impl PartialEq for Vec2Weighted {
    fn eq(&self, other: &Self) -> bool {
        other.h == self.h
    }
}

impl Eq for Vec2Weighted {}

impl PartialOrd for Vec2Weighted {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vec2Weighted {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for max-heap behavior
        other.h.total_cmp(&self.h)
    }
}

////////////////////////////////////////

fn is_valid(vec: Vec2, row_cal: Vec2) -> bool {
    return (vec.x >= 0) && (vec.x < row_cal.x) && (vec.y >= 0) && (vec.y < row_cal.y);
}

fn get_distance(vec1: Vec2, vec2: Vec2) -> f64 {
    return f64::sqrt(
        ((vec1.x - vec2.x) * (vec1.x - vec2.x) + (vec1.y - vec2.y) * (vec1.y - vec2.y)) as f64,
    );
}

fn make_new_vec(reference: Vec2, add: Vec2) -> Vec2 {
    return Vec2 {
        x: reference.x + add.x,
        y: reference.y + add.y,
    };
}

fn get_children(vec: Vec2, row_cal: Vec2) -> Vec<Vec2> {
    let mut return_list: Vec<Vec2> = Vec::new();

    for i in -1..2 {
        for j in -1..2 {
            let new_vec: Vec2 = make_new_vec(vec.clone(), Vec2 { x: i, y: j });

            if !is_valid(new_vec, row_cal) {
                continue;
            }

            return_list.push(new_vec)
        }
    }

    return return_list;
}

fn is_obstructed(vec: Vec2, grid: &Vec<Vec<i32>>) -> bool {
    return grid[vec.x as usize][vec.y as usize] == 1;
}

fn get_ms_time() -> u64 {
    let start_time = SystemTime::now();
    let since_the_epoch = start_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let milliseconds =
        since_the_epoch.as_secs() * 1000 + u64::from(since_the_epoch.subsec_millis());
    return milliseconds;
}

fn get_closed(cur: Vec2, parent: Vec2) -> Node {
    return Node {
        is_closed: true,
        is_open: false,
        coord: cur,
        parent,
    };
}

fn retrace_path(end_vec: Vec2, start_vec: Vec2, node_info: Vec<Vec<Node>>) -> Vec<Vec2> {
    let mut cur_node: Node = node_info[end_vec.x as usize][end_vec.y as usize];
    let mut vectr: Vec<Vec2> = Vec::new();
    vectr.push(cur_node.coord.clone());
    let mut iter = 0;

    while !cur_node.parent.is_same(start_vec) && iter < 50 {
        //print!(" !!! {:?}", cur_node);

        cur_node = node_info[cur_node.parent.x as usize][cur_node.parent.y as usize];
        vectr.push(cur_node.coord.clone());

        iter += 1;
    }

    vectr.push(start_vec);

    return vectr;
}

fn path_finder(
    start: Vec2,
    end: Vec2,
    grid: &Vec<Vec<i32>>,
    iterations: i32,
    row_cal: Vec2,
) -> Option<Vec<Vec2>> {
    let mut node_data: Vec<Vec<Node>> = Vec::new();
    let mut cur_iter: i32 = 0;
    let mut binary_heap: BinaryHeap<Vec2Weighted> = BinaryHeap::new();

    binary_heap.push(Vec2Weighted {
        x: start.x,
        y: start.y,
        h: get_distance(start, end),
    });

    for _ in 0..row_cal.x {
        let mut row: Vec<Node> = Vec::new();
        for _ in 0..row_cal.y {
            let node: Node = Node {
                is_closed: false,
                is_open: false,
                coord: Vec2 { x: -1, y: -1 },
                parent: Vec2 { x: -1, y: -1 },
            };

            row.push(node);
        }

        node_data.push(row);
    }

    node_data[start.x as usize][start.y as usize] = Node {
        is_closed: false,
        is_open: true,
        coord: start,
        parent: Vec2 { x: -1, y: -1 },
    };

    while !binary_heap.is_empty() && cur_iter < iterations {
        let cur_list_vec: Vec2Weighted = binary_heap.pop().unwrap();
        let cur_vec: Vec2 = cur_list_vec.to_vec2();

        if cur_vec.is_same(end) {
            return Some(retrace_path(cur_vec, start, node_data));
        }

        node_data[cur_vec.x as usize][cur_vec.y as usize] = get_closed(
            cur_vec,
            node_data[cur_vec.x as usize][cur_vec.y as usize].parent,
        );

        let children: Vec<Vec2> = get_children(cur_vec, row_cal);
        for i in children.iter() {
            let vec: Vec2 = *i;
            let child_node: &Node = &mut node_data[vec.x as usize][vec.y as usize];

            if is_obstructed(vec, grid) || child_node.is_closed {
                node_data[vec.x as usize][vec.y as usize] = Node {
                    is_closed: true,
                    is_open: false,
                    coord: child_node.coord,
                    parent: child_node.parent,
                };

                continue;
            }

            if !child_node.is_open {
                let h_cost: f64 = get_distance(end, vec);

                node_data[vec.x as usize][vec.y as usize] = Node {
                    is_closed: child_node.is_closed,
                    is_open: true,
                    coord: vec,
                    parent: cur_vec,
                };

                binary_heap.push(Vec2Weighted {
                    x: vec.x,
                    y: vec.y,
                    h: h_cost,
                })
            }
        }

        cur_iter += 1;
    }

    None
}

fn auto_generate_random(
    max_min_col_vec: Vec2,
    max_min_row_vec: Vec2,
    obstructed_percent: f64,
) -> Vec<Vec<i32>> {
    // The obstructed percent thingy isnt what u think it is basically the % chance that a specific node will be obstructed.
    let mut grid: Vec<Vec<i32>> = Vec::new();

    let mut rng = rand::thread_rng();
    let random_int_col: i32 = rng.gen_range(max_min_col_vec.x..=max_min_col_vec.y);
    let random_int_row: i32 = rng.gen_range(max_min_row_vec.x..=max_min_row_vec.y);

    for _ in 0..random_int_col {
        let mut temp_vec: Vec<i32> = Vec::new();

        for _ in 0..random_int_row {
            let gened_perc = rng.gen_range(0..=100) as f64;
            temp_vec.push(get_val(is_will_be_obstructed(
                obstructed_percent,
                gened_perc,
            )));
        }

        grid.push(temp_vec);
    }

    return grid;
}

fn auto_gen_random_point(point_range: Vec2, grid: &Vec<Vec<i32>>) -> Vec2 {
    let mut rng = rand::thread_rng();
    let mut random_int_col: i32 = rng.gen_range(0..point_range.x);
    let mut random_int_row: i32 = rng.gen_range(0..point_range.y);

    let mut point_gen = Vec2 {
        x: random_int_col,
        y: random_int_row,
    };

    while is_valid(point_gen, point_range) && !is_obstructed(point_gen, grid) {
        random_int_col = rng.gen_range(0..point_range.x);
        random_int_row = rng.gen_range(0..point_range.y);

        point_gen = Vec2 {
            x: random_int_col,
            y: random_int_row,
        };
    }

    return point_gen;
}

fn get_val(result_o_gen: bool) -> i32 {
    if !result_o_gen {
        return 1;
    } else {
        return 0;
    }
}

fn is_will_be_obstructed(percent: f64, generated: f64) -> bool {
    return generated > percent;
}

fn print_vectrs(vectrs: &Vec<Vec<i32>>) {
    for j in vectrs.iter() {
        let init_string = "[".to_string();
        let end_string = "]".to_string();
        let mut inner_str = "".to_string();
        inner_str = init_string + &inner_str;

        for i in j.iter() {
            let numb_str = i.clone().to_string();
            let main_str = ", ".to_string();
            let combined = " ".to_string() + &numb_str + &main_str;

            inner_str = inner_str + &combined;
        }

        inner_str = inner_str + &end_string;

        println!("{}", inner_str)
    }
}

fn print_vectrs_with_markers(vectrs: &Vec<Vec<i32>>, markers: &Vec<Vec2>) {
    for i in 0..vectrs.len() {
        let init_string = "[".to_string();
        let end_string = "]".to_string();
        let mut inner_str = "".to_string();
        inner_str = init_string + &inner_str;

        for j in 0..vectrs[i].len() {
            let i_j_vec: Vec2 = Vec2 {
                x: i as i32,
                y: j as i32,
            };

            let mut numb_str = vectrs[i][j].clone().to_string();

            if markers.contains(&i_j_vec) {
                numb_str = "🔴".to_string();
            } else {
                numb_str = " ".to_string() + &numb_str;
            }

            let main_str = ", ".to_string();
            let combined = numb_str + &main_str;

            inner_str = inner_str + &combined;
        }

        inner_str = inner_str + &end_string;

        println!("{}", inner_str)
    }
}

fn print_end() {
    println!(" ");
    println!(" ");
    println!("---------------------------------------------------------");
}

fn main() {
    println!("---------------------------------------------------------");
    let mut start: Vec2 = Vec2 { x: 0, y: 0 };
    let mut end: Vec2 = Vec2 { x: 0, y: 9 };
    let mut grid: Vec<Vec<i32>> = vec![
        vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 1, 0, 1, 0],
        vec![1, 1, 0, 1, 0, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 1, 0, 1],
        vec![0, 1, 0, 0, 0, 0, 1, 0, 1, 1],
        vec![0, 1, 1, 1, 1, 0, 1, 1, 1, 0],
        vec![0, 1, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 1, 1, 0, 1, 1, 0],
    ];

    println!(" ");

    let row_cal: Vec2;

    if IS_AUTO_GEN {
        grid.clear();
        grid = auto_generate_random(MAX_MIN_COL, MAX_MIN_ROW, OBSTRUCTED_PERCENT);
        row_cal = Vec2 {
            x: grid.len() as i32,
            y: grid[0].len() as i32,
        };

        start = auto_gen_random_point(row_cal, &grid);
        end = auto_gen_random_point(row_cal, &grid);
    } else {
        row_cal = Vec2 {
            x: grid.len() as i32,
            y: grid[0].len() as i32,
        };
    }

    if IS_PRINT_GRAPH {
        print_vectrs(&grid);
    }

    println!(" ");
    //println!(" {}", (row_cal.x * row_cal.y));

    let start_ms = get_ms_time(); // modify @this location to see speeeeed

    let result: Option<Vec<Vec2>> = path_finder(start, end, &grid, row_cal.x * row_cal.y, row_cal);

    if result.is_none() {
        print!("Found no path!");
        print_end();
        return;
    }

    print!("FOUND! Took -> {:?} ms.", get_ms_time() - start_ms);
    println!(" ");

    let unwrapped = result.unwrap();

    for i in unwrapped.iter().rev() {
        print!("({}, {}) ", i.x, i.y);
    }

    if IS_PRINT_GRAPH_END {
        println!(" ");
        println!(" ");
        print_vectrs_with_markers(&grid, &unwrapped);
    }

    print_end();
}


// RRRRAAAAWWWWWWRRRR
// Was watching to Ryan's only-fans the whole time
