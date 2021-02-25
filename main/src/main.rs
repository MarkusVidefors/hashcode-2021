use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_ascii_whitespace();
    let d: usize = input.next().unwrap().parse().unwrap();
    let i: usize = input.next().unwrap().parse().unwrap();
    let s: usize = input.next().unwrap().parse().unwrap();
    let v: usize = input.next().unwrap().parse().unwrap();
    let f: usize = input.next().unwrap().parse().unwrap();

    let mut bloo = vec![Vec::new(); i];

    let mut streets = Vec::with_capacity(s);
    for _ in 0..s {
        let b: usize = input.next().unwrap().parse().unwrap();
        let e: usize = input.next().unwrap().parse().unwrap();
        let name = input.next().unwrap();
        let l: usize = input.next().unwrap().parse().unwrap();
        streets.push((b, e, name, l));
        bloo[e].push(name);
    }

    let mut cars = Vec::with_capacity(v);
    for _ in 0..v {
        let p: usize = input.next().unwrap().parse().unwrap();
        let mut car = Vec::with_capacity(p);
        for _ in 0..p {
            let name = input.next().unwrap();
            car.push(name);
        }
        cars.push(car);
    }

    println!("{}", i);
    for id in 0..i {
        println!("{}", id);
        println!("{}", bloo[id].len());
        for n in bloo[id].iter() {
            println!("{} 1", n);
        }
    }
}
