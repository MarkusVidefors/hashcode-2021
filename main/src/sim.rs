use std::collections::VecDeque;

struct Street<'a> {
    from: usize,
    end: usize,
    name: &'a str,
    len: usize,
    cars: VecDeque<Car<'a>>,
}

impl<'a> Street<'a> {
    fn new(data: (usize, usize, &'a str, usize)) -> Self {
        Self {
            from: data.0,
            end: data.1,
            name: data.2,
            len: data.3,
            cars: VecDeque::new(),
        }
    }
}

struct Car<'a> {
    path: Vec<&'a str>,
    dist: usize,
}

impl<'a> Car<'a> {
    fn new(data: Vec<&'a str>) -> Self {
        Self {
            path: data,
            dist: 0,
        }
    }
}

pub struct Simulator<'a> {
    streets: Vec<Street<'a>>,
    light_sch: Vec<Vec<(&'a str, usize)>>, // street, time
    stop_time: usize,
    intersections: Vec<(usize, usize)>, // current street index in light_schedule, time left
    f: usize,
}

impl<'a> Simulator<'a> {
    pub fn new(
        street_data: &Vec<(usize, usize, &'a str, usize)>,
        car_data: &Vec<Vec<&'a str>>,
        n_inters: usize,
        light_sch: Vec<Vec<(&'a str, usize)>>,
        stop_time: usize,
        f: usize,
    ) -> Self {
        let mut streets = Vec::new();
        for sd in street_data {
            streets.push(Street::new(*sd));
        }
        for cd in car_data {
            let car = Car::new(cd.clone());
            for s in streets.iter_mut() {
                if s.name == car.path[0] {
                    s.cars.push_back(car);
                    break;
                }
            }
        }
        let mut intersections = Vec::with_capacity(n_inters);
        for i in 0..n_inters {
            if light_sch[i].is_empty() {
                intersections.push((0, 0));
            } else {
                intersections.push((0, light_sch[i][0].1));
            }
        }
        Self {
            streets,
            light_sch,
            stop_time,
            intersections,
            f,
        }
    }

    pub fn run(&mut self) -> usize {
        let mut score = 0;
        for t in 0..self.stop_time {
            for street_index in 0..self.streets.len() {
                if !self.light_sch[self.streets[street_index].end].is_empty()
                    && self.light_sch[self.streets[street_index].end]
                        [self.intersections[self.streets[street_index].end].0]
                        .0
                        == self.streets[street_index].name
                {
                    if let Some(c) = self.streets[street_index].cars.front() {
                        if c.dist == 0 {
                            let mut car = self.streets[street_index].cars.pop_front().unwrap();
                            car.path.remove(0);
                            if !car.path.len() == 0 {
                                for target in self.streets.iter_mut() {
                                    if target.name == car.path[0] {
                                        car.dist = target.len;
                                        target.cars.push_back(car);
                                        break;
                                    }
                                }
                            } else {
                                score += self.f + self.stop_time - t;
                            }
                        }
                    }
                }
                for c in self.streets[street_index].cars.iter_mut() {
                    if c.dist > 0 {
                        c.dist -= 1;
                    }
                }
            }

            for (n, i) in self.intersections.iter_mut().enumerate() {
                if !self.light_sch[n].is_empty() {
                    i.1 -= 1;
                    if i.1 == 0 {
                        let mut idx = i.0 + 1;
                        if idx == self.light_sch[n].len() {
                            idx = 0;
                        }
                        i.0 = idx;
                        i.1 = self.light_sch[n][idx].1;
                    }
                }
            }
        }
        score
    }
}
