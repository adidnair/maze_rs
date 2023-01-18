ue image::GenericImageView;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

struct Maze {
    width: usize,
    height: usize,
    start: usize,
    end: usize,
    cell_map: Vec<Cell>,
}

struct Cell {
    wall: bool,
    node: Option<Node>,
}

struct Node {
    lrtb: [usize; 4],
    // visited: bool,
    distance: usize,
    prev: Option<usize>,
}

impl Maze {
    pub fn new(image: image::DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        let width = usize::try_from(width).unwrap();
        let height = usize::try_from(height).unwrap();
        let pixel_map = image.into_rgb8().into_raw();
        let cell_map: Vec<Cell> = pixel_map.iter().step_by(3).map(|r|
            Cell {
                wall: (*r == 0u8), node: None,
            }
        ).collect();

        let start = cell_map[..width].iter().position(|x| x.wall == false).unwrap_or_else(|| {
            width*cell_map.iter().step_by(width).position(|x| x.wall == false).unwrap()
        });

        let end = cell_map[width*(height-1)..].iter().position(|x| x.wall == false).unwrap_or_else(|| {
            width - 1 + width*cell_map[width-1..].iter().step_by(width).position(|x| x.wall == false).unwrap()
        });

        Maze {width, height, cell_map, start, end}
    }

    fn get_node(&self, i: usize) -> Option<Node> {
        if  self.cell_map[i-1].wall == self.cell_map[i+1].wall &&
            self.cell_map[i-self.width].wall == self.cell_map[i+self.width].wall &&
            self.cell_map[i+1].wall != self.cell_map[i+self.width].wall {
            None
        } else {
            let mut left = i-1;
            let mut top = i-self.width;
            if self.cell_map[left].wall == true {
                left = 0;
            } else {
                while self.cell_map[left].wall == false && self.cell_map[left].node.is_none() {
                    left -= 1;
                }
            }
            if self.cell_map[top].wall == true {
                top = 0;
            } else {
                while self.cell_map[top].wall == false && self.cell_map[top].node.is_none() {
                    top -= self.width;
                }
            }
            Some(Node {
                lrtb: [left, 0, top, 0],
                // visited: false,
                distance: usize::MAX,
                prev: None,
            })
        }
    }

    pub fn parse(&mut self){
        self.cell_map[self.start].node = Some(Node {
            lrtb: [0, 0, 0, 0],
            // visited: false,
            distance: 0usize,
            prev: None,
        });
        for i in 1..self.height-1 {
            for j in 1..self.width-1 {
                let curr = self.width*i+j;
                self.cell_map[curr].node = self.get_node(curr);
                if let Some(left) = self.cell_map[curr].node.as_ref().map(|n| n.lrtb[0]) {
                    if left != 0 {
                        self.cell_map[left].node.as_mut().map(|n| n.lrtb[1] = curr);
                    }
                }
                if let Some(top) = self.cell_map[curr].node.as_ref().map(|n| n.lrtb[2]) {
                    if top != 0 {
                        self.cell_map[top].node.as_mut().map(|n| n.lrtb[3] = curr);
                    }
                }
            }
        }
        if self.end > self.width*(self.height-1) {
            let mut top = self.end - self.width;
            while self.cell_map[top].node.is_none() {
                top -= self.width;
            }
            self.cell_map[self.end].node = Some(Node {
                lrtb: [0, 0, top, 0],
                // visited: false,
                distance: usize::MAX,
                prev: None,
            });
        } else {
            let mut left = self.end - 1;
            while self.cell_map[left].node.is_none() {
                left -= 1;
            }
            self.cell_map[self.end].node = Some(Node {
                lrtb: [left, 0, 0, 0],
                // visited: false,
                distance: usize::MAX,
                prev: None,
            });
        }
    }

    pub fn only_nodes(&self) -> Vec<&Node> {
        self.cell_map
            .iter()
            .filter(|cell| cell.node.is_some())
            .collect()
    }

    pub fn solve(&mut self) {
        let mut p_queue = BinaryHeap::from(self.only_nodes());
    }

    pub fn print_with_nodes(&self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if self.cell_map[self.width*i + j].wall == true {print!("██")} else {
                    if self.cell_map[self.width*i + j].node.is_some() {
                        print!("()")
                    } else {
                        print!("  ")
                    }
                }
            }
            print!("\n")
        }
    }

    // pub fn print(&self) {
    //     for i in 0..self.height {
    //         for j in 0..self.width {
    //             if self.cell_map[self.width*i + j].wall == true {print!("██")} else {print!("  ")}
    //         }
    //         print!("\n")
    //     }
    // }
}

fn main() {
    let maze_img = image::open("./mazes/maze.png").unwrap();
    let mut maze = Maze::new(maze_img);
    maze.parse();
    maze.print_with_nodes();
    println!("{:?}", maze.only_nodes())
}
