use std::{collections::HashSet, str::FromStr};

type Coord = [i64; 2];

const fn dist([x, y]: Coord, [x2, y2]: Coord) -> u64 {
    x.abs_diff(x2) + y.abs_diff(y2)
}

#[derive(Debug)]
struct Sensor {
    pos: Coord,
    closest_beacon: Coord,
    dist: u64,
}

#[derive(Debug)]
struct Sensors {
    sensors: Vec<Sensor>,
    beacons: HashSet<Coord>,
    min: Coord,
    max: Coord,
}

impl Sensor {
    const fn is_in_range(&self, [x, y]: Coord) -> bool {
        self.pos[0].abs_diff(x) + self.pos[1].abs_diff(y) <= self.dist
    }

    fn border_positions(&self) -> impl Iterator<Item = Coord> {
        let dist = self.dist as i64 + 1;
        let [x, y] = self.pos;
        (0..=dist).flat_map(move |i| {
            [
                [x - dist - i, y - i],
                [x + dist - i, y - i],
                [x - dist - i, y + i],
                [x + dist - i, y + i],
            ]
        })
    }
}

impl Sensors {
    fn no_beacons_in_row(&self, row: i64) -> usize {
        (self.min[0]..=self.max[0])
            .filter(|x| {
                let coord = [*x, row];
                !self.beacons.contains(&coord) && self.sensors.iter().any(|s| s.is_in_range(coord))
            })
            .count()
    }

    fn find_distress_beacon(&self) -> Option<Coord> {
        for sensor in &self.sensors {
            for p in sensor.border_positions() {
                if p[0] < 0 || p[1] < 0 || p[0] > 4_000_000 || p[1] > 4_000_000 {
                    continue;
                }
                if self.sensors.iter().all(|s| !s.is_in_range(p)) {
                    return Some(p);
                }
            }
        }
        None
    }
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(':', ",");
        let elems: Vec<i64> = s
            .split(',')
            .map(|elem| {
                elem.trim_matches(|c: char| c != '-' && !c.is_numeric())
                    .parse()
            })
            .collect::<Result<_, _>>()
            .map_err(|_| format!("`{s}` should contain 4 numeric values"))?;
        let [x, y, bx, by]: [_; 4] = elems
            .try_into()
            .map_err(|_| format!("Expected 4 numeric values in `{s}`"))?;
        let pos = [x, y];
        let closest_beacon = [bx, by];
        Ok(Self {
            dist: dist(pos, closest_beacon),
            pos,
            closest_beacon,
        })
    }
}

impl FromStr for Sensors {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut min = [i64::MAX; 2];
        let mut max = [i64::MIN; 2];
        let mut beacons = HashSet::new();

        let sensors: Vec<_> = s.lines().map(Sensor::from_str).collect::<Result<_, _>>()?;

        for sensor in &sensors {
            beacons.insert(sensor.closest_beacon);
            let dist = sensor.dist as i64;
            let [x_min, x_max] = [sensor.pos[0] - dist, sensor.pos[0] + dist];
            let [y_min, y_max] = [sensor.pos[1] - dist, sensor.pos[1] + dist];
            min[0] = min[0].min(x_min);
            max[0] = max[0].max(x_max);
            min[1] = min[1].min(y_min);
            max[1] = max[1].max(y_max);
        }
        Ok(Self {
            sensors,
            beacons,
            min,
            max,
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let sensors = Sensors::from_str(input).unwrap();
    // println!("{sensors:#?}");
    // Part 1
    println!("Part 1: {} positions", sensors.no_beacons_in_row(2_000_000));
    // Part 2
    let [x, y] = sensors.find_distress_beacon().unwrap();
    println!(
        "Part : Beacon at [{x},{y}]: frequency = {}",
        x * 4_000_000 + y
    );
}
