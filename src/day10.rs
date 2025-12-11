use std::usize;

use failure::Error;
use parse::parse_input;

mod parse {
    use crate::parsers::unsigned;
    use failure::{Error, err_msg};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::newline;
    use nom::combinator::{all_consuming, map, value};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{delimited, terminated};
    use nom::{IResult, Parser};

    use super::{Machine, Light};


    fn light(input: &str) -> IResult<&str, Light> {
        alt((
            value(Light::Off, tag(".")),
            value(Light::On, tag("#"))
        )).parse(input)
    }

    fn lights(input: &str) -> IResult<&str, Box<[Light]>> {
        delimited(tag("["), map(many1(light), Vec::into_boxed_slice), tag("]")).parse(input)
    }

    fn button(input: &str) -> IResult<&str, Box<[usize]>> {
        delimited(tag("("), map(separated_list1(tag(","), unsigned), Vec::into_boxed_slice), tag(")")).parse(input)
    }


    fn buttons(input: &str) -> IResult<&str, Box<[Box<[usize]>]>> {
        map(separated_list1(tag(" "), button), Vec::into_boxed_slice).parse(input)
    }

    fn joltage(input: &str) -> IResult<&str, Box<[u64]>> {
        delimited(tag("{"), map(separated_list1(tag(","), unsigned), Vec::into_boxed_slice), tag("}")).parse(input)
    }

    fn machine(input: &str) -> IResult<&str, Machine> {
        map((lights, tag(" "), buttons, tag(" "), joltage), |(lights, _, buttons, _, _joltage)| Machine { lights, buttons, _joltage }).parse(input)
    }


    pub(super) fn parse_input(s: &str) -> Result<Box<[Machine]>, Error> {
        all_consuming(map(
            many1(terminated(machine, newline)),
            Vec::into_boxed_slice,
        ))
        .parse(s)
        .map_err(|err| err_msg(format!("Failed to parse input: {}", err)))
        .map(|(_, machines)| machines)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Light {
    On,
    Off
}

impl Light {
    fn flip(&mut self) {
        *self = match self {
            Light::On => Light::Off,
            Light::Off => Light::On
        }
    }
}

pub struct Machine {
    lights: Box<[Light]>,
    buttons: Box<[Box<[usize]>]>,
    _joltage: Box<[u64]>,
}

impl Machine {

    fn press_button(&self, lights: &[Light], buttons: &[usize]) -> Vec<Light> {
        let mut lights = lights.to_vec();
        for &index in buttons {
            lights[index].flip()
        }
        lights
    }

    fn num_matching_lights(&self, lights: &[Light]) -> usize {
        self.lights.iter().zip(lights.iter()).filter(|(l1, l2)| l1 == l2).count()
    }

    fn min_presses(&self) -> usize {
        let mut best = usize::MAX;
        let mut stack = vec![(self.lights.iter().map(|_| Light::Off).collect::<Vec<_>>(), &self.buttons[..], 0)];

        while let Some((lights, buttons, count)) = stack.pop() {
            if count >= best {
                continue;
            }
            if &lights[..] == &self.lights[..] {
                best = count;
                continue;
            }
            if buttons.is_empty() {
                continue;
            }

            let new_lights = self.press_button(&lights, &buttons[0]);

            let current_match = self.num_matching_lights(&lights);
            let new_match = self.num_matching_lights(&new_lights);

            let dont_push = (lights, &buttons[1..], count);
            let do_push = (new_lights, &buttons[1..], count + 1);

            if new_match > current_match {
                stack.push(dont_push);
                stack.push(do_push);
            } else {
                stack.push(do_push);
                stack.push(dont_push);
            }
        }

        best
    }
}

pub struct Solver {}

impl super::Solver for Solver {
    type Problem = Box<[Machine]>;

    fn parse_input(data: String) -> Result<Self::Problem, Error> {
        parse_input(&data)
    }

    fn solve(machines: Self::Problem) -> (Option<String>, Option<String>) {
        let part1: usize = machines.iter().map(|machine| machine.min_presses()).sum();
        (Some(part1.to_string()), None)
    }
}
