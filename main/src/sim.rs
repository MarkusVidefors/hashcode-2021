use std::collections::VecDeque;

struct Street {
    from: usize,
    to: usize,
    name: String,
    len: usize,
    cars: VecDeque<Car>,
}

impl Street {
    fn new(data: (usize, usize, String, usize)) -> Self {
        Self {
            from: data.0,
            to: data.1,
            name: data.2,
            len: data.3,
            cars: Vec::new(),
        }
    }
}

struct Car {
    path: Vec<String>,
    dist: i32,
}

impl Car {
    fn new(data: Vec<String>) -> Self {
        Self {
            path: data,
            dist: 0,
        }
    }
}

struct Simulator {
    cars: Vec<Car>,
    streets: Vec<Street>,
    light_sch: Vec<Vec<(String, usize)>>, // street, time
    stop_time: usize,
    intersections: Vec<(usize, usize)>, // current street index in light_schedual, time left
    f: usize,
}

impl Simulator {
    fn new(
        street_data: Vec<(usize, usize, String, usize)>,
        car_data: Vec<Vec<String>>,
        n_inters: usize,
        light_sch: Vec<Vec<(String, usize)>>,
        stop_time: usize,
        f: usize,
    ) -> Self {
        let mut streets = Vec::new();
        for sd in street_data {
            streets.push(Street::new(sd));
        }
        let mut cars = Vec::new();
        for cd in car_data {
            let car = Car::new(cd);
            for s in streets.iter() {
                if s.name == car.path[0] {
                    s.cars.push(car)
                }
            }
        }
        let mut intersections = Vec::with_capacity(n_inters);
        for i in 0..n_inters {
            intersections.push((0, light_sch[i][0].1));
        }
        Self {
            cars,
            streets,
            light_sch,
            stop_time,
            intersections,
            f,
        }
    }
    fn run() -> usize {
        let mut score = 0;
        for t in 0..self.stop_time {
            for s in self.streets.iter() {
                for c in s.cars.iter() {
                    if c.dist > 0 {
                        c.dist -= 1;
                    }
                }
                if self.light_sch[s.end][self.intersections[s.end].0].0 == s.name {
                    if let Some(c) = s.cars.front() {
                        if c.dist == 0 {
                            let car = s.cars.pop_front().unwrap();
                            car.path.remove[0];
                            if !car.path.len() == 0 {
                                for target in streets.iter() {
                                    if target.name == car.path[0] {
                                        car.dist = target.len;
                                        target.cars.push_back(car)
                                    }
                                }
                            } else {
                                //POINTS!
                                score += self.f + self.stop_time - t;
                            }
                        }
                    }
                }
            }
            for (n, i) in self.intersections.iter().enumerate() {
                i.1 -= 1;
                if i.1 == 0 {
                    let mut idx = i.0 + 1;
                    if idx == self.light_sch[n].len() {
                        idx = 0;
                    }
                    i.0 = idx;
                    i.1 = light_sch[n][idx].1;
                }
            }
        }
        score
    }
}
