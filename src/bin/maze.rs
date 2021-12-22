#[derive(Debug, Copy, Clone)]
struct Node {
    value: u32,
    neighbors: &'static [(usize, char)],
}

static NODES: [Node; 8] = [
    Node {
        value: 22,
        neighbors: &[(1, '+'), (2, '+'), (2, '-'), (4, '-')],
    },
    Node {
        value: 4,
        neighbors: &[(2, '+'), (2, '*'), (5, '*'), (3, '*'), (3, '*')],
    },
    Node {
        value: 4,
        neighbors: &[
            (4, '-'),
            (6, '-'),
            (5, '-'),
            (5, '*'),
            (3, '*'),
            (1, '*'),
            (1, '+'),
        ],
    },
    Node {
        value: 8,
        neighbors: &[(1, '*'), (2, '*'), (5, '*'), (5, '-'), (7, '-')],
    },
    Node {
        value: 9,
        neighbors: &[(2, '-'), (5, '-'), (6, '-'), (6, '*')],
    },
    Node {
        value: 11,
        neighbors: &[
            (2, '*'),
            (2, '-'),
            (4, '-'),
            (6, '-'),
            (6, '*'),
            (7, '*'),
            (7, '-'),
            (3, '-'),
            (3, '*'),
            (1, '*'),
        ],
    },
    Node {
        value: 18,
        neighbors: &[(4, '-'), (4, '*'), (2, '-'), (5, '-'), (5, '*'), (7, '*')],
    },
    Node {
        value: 1,
        neighbors: &[(6, '*'), (5, '-'), (5, '*'), (3, '-')],
    },
];

fn pathsum(path: &Vec<(u32, char)>) -> u32 {
    path.iter().fold(0, |acc, (val, op)| match op {
        '+' => acc + val,
        '-' => acc - val,
        '*' => acc * val,
        _ => unreachable!(),
    })
}
fn bredth_first(
    currentpath: &Vec<(u32, char)>,
    pos: usize,
    end: usize,
    maxlen: usize,
) -> Option<Vec<(u32, char)>> {
    if currentpath.len() == maxlen {
        return None;
    }
    if (pos == end) & (pathsum(&currentpath) == 30) {
        return Some(currentpath.clone());
    }
    for &(ind, op) in NODES[pos].neighbors {
        let mut newpath = currentpath.clone();
        newpath.push((NODES[ind].value, op));
        if let Some(vec) = bredth_first(&newpath, ind, end, maxlen) {
            return Some(vec);
        }
    }
    None
}

fn main() {
    if let Some(ans) = bredth_first(&vec![(22, '+')], 0, 7, 8) {
        print!("0 ");
        for (val, op) in &ans {
            print!("{} {} ", op, val);
        }
        println!("= {}", pathsum(&ans));
    }
}
