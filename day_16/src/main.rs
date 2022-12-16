use std::{collections::HashMap, str::FromStr};

type ValveId = u8;

#[derive(Debug)]
struct Valve {
    id: ValveId,
    opened: bool,
    flow_rate: u8,
    leads_to: Vec<ValveId>,
}

#[derive(Debug)]
struct Tunnel {
    valves: HashMap<ValveId, Valve>,
    current_valve: ValveId,
    remaining_time: u8,
    current_rate: u8,
}

impl FromStr for Valve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || -> Self::Err { format!("`{s}` is invalid") };
        let s = s.strip_prefix("Valve ").ok_or_else(err)?;
        let id = s[..=2].bytes().sum();
        let s = s[2..].strip_prefix(" has flow rate=").ok_or_else(err)?;
        let (flow_rate, valves) = s.split_once(';').ok_or_else(err)?;
        let flow_rate = flow_rate
            .parse()
            .map_err(|_| format!("`{flow_rate}` is not a valid number"))?;
        let valves = valves
            .replace("tunnel ", "tunnels ")
            .replace("leads ", "lead ")
            .replace("valve ", "valves ");
        let valves = valves
            .strip_prefix(" tunnels lead to valves ")
            .ok_or_else(err)?;
        let leads_to = valves.split(", ").map(|str| str.bytes().sum()).collect();
        Ok(Self {
            id,
            opened: false,
            flow_rate,
            leads_to,
        })
    }
}

impl FromStr for Tunnel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valves = s
            .lines()
            .map(|l| Valve::from_str(l).map(|v| (v.id, v)))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            valves,
            current_valve: b'a' * 2,
            current_rate: 0,
            remaining_time: 30,
        })
    }
}

fn main() {
    let tunnel = Tunnel::from_str(include_str!("../test_input.txt")).unwrap();
    println!("{tunnel:#?}");
}
