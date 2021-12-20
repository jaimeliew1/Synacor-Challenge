use std::collections::HashMap;
// fn func_6027(mut x: &mut u16, mut y: &mut u16, mut stack: &mut Vec<u16>, target: u16) {
//     // println!("a");
//     while *x != 0 {
//         if *y != 0 {
//             let temp = *x;
//             *y -= 1;
//             func_6027(&mut x, &mut y, &mut stack, target);
//             // 6056
//             *y = *x;
//             *x = temp;
//             *x -= 1;
//             func_6027(&mut x, &mut y, &mut stack, target);
//             // 6067
//             return;
//         } else {
//             *x -= 1;
//             *y = target;
//             func_6027(&mut x, &mut y, &mut stack, target);
//             // 6047
//             return;
//         }
//     }
//     //6030
//     *x = (*y + 1) % 32768;
// }

fn func(x: u16, y: u16, target: u16, cache: &mut HashMap<(u16, u16), u16>) -> u16 {
    if let Some(out) = cache.get(&(x, y)) {
        return *out;
    } else {
        let out = match (x, y) {
            (0, y) => (y + 1) % 32768,
            (x, 0) => func(x - 1, target, target, cache),
            (x, y) => func(x - 1, func(x, y - 1, target, cache), target, cache),
        };
        cache.insert((x, y), out);
        return out;
    }
}

fn main() {
    for target in 1..32768 {
        let x = func(4, 1, target, &mut HashMap::new());
        if x == 6 {
            println!("target: {} (x, y): {:?}", target, x);
        }
    }

}
