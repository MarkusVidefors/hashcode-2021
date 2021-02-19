use std::io::Read;

fn greedy(mut pizzas: Vec<Vec<&str>>, teams: [usize; 3]) {
    let mut deliveries: Vec<Vec<usize>> = Vec::new();

    for i in 0..=2 {
        let size = i + 2;
        for _ in 0..teams[i] {
            if pizzas.len() < size {
                break
            }

            let mut to_deliver = vec![pizzas.len()];
            let mut ingredients = pizzas.pop().unwrap();

            for _ in 0..size - 1 {
                let mut best: (usize, usize) = (0, 0);

                for (i, other) in pizzas.iter().enumerate() {
                    let mut collisions = 0;
                    for topping in other.iter() {
                        if ingredients.contains(topping) {
                            collisions += 1;
                        }
                    }

                    let total = other.len() + ingredients.len() - collisions;
                    if total > best.0 {
                        best = (total, i)
                    }
                }

                println!("debug A {:?}", pizzas);
                ingredients.append(&mut pizzas.remove(best.1));
                ingredients.sort();
                ingredients.dedup();
                println!("debug A {:?}", pizzas);

                println!("debug B {:?}", to_deliver);
                to_deliver.push(best.1);
                println!("debug B {:?}", to_deliver);
            }

            deliveries.push(to_deliver);
        }
    }

    println!("{}", deliveries.len());
    for d in deliveries.iter() {
        print!("{} ", d.len());
        for i in d.iter() {
            print!("{} ", i);
        }
        print!("\n")
    }
}

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let mut input = buf.split_whitespace();

    let m: usize = input.next().unwrap().parse().unwrap();
    let teams: [usize; 3] = [
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
        input.next().unwrap().parse().unwrap(),
    ];

    let mut pizzas: Vec<Vec<&str>> = Vec::with_capacity(m);
    for _ in 0..m {
        let i = input.next().unwrap().parse().unwrap();
        let mut pizza: Vec<&str> = Vec::with_capacity(i);

        for _ in 0..i {
            let ingredient = input.next().unwrap();
            pizza.push(ingredient)
        }

        pizzas.push(pizza);
    }

    greedy(pizzas, teams);
}
