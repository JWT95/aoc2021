use crate::common::read_input;
use anyhow::Result;
use recap::Recap;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct Packet {
    version: usize,
    operator: usize,
    value: usize,
    sub_packets: Vec<Packet>,
}

fn parse_packet_from_string(input: String) -> (Packet, usize) {
    // First parse the headers
    let version = String::from(&input[0..3]);
    let operator = String::from(&input[3..6]);

    // Now parse the value
    let (value, sub_packets, offset) = match operator.as_str() {
        "100" => {
            let (value, offset) = parse_value(&input[6..]);
            (value, vec![], offset)
        }
        _ => {
            let (packets, offset) = parse_subpackets(&input[6..]);
            (String::from("0"), packets, offset)
        }
    };

    let packet = Packet {
        version: usize::from_str_radix(&version, 2).unwrap(),
        operator: usize::from_str_radix(&operator, 2).unwrap(),
        value: usize::from_str_radix(&value, 2).unwrap(),
        sub_packets,
    };

    (packet, offset + 6)
}

fn parse_value(input: &str) -> (String, usize) {
    // Go through the string until we get to the end
    let mut processed_string = String::from("");
    let mut head = 0;
    let mut finished = false;
    while !finished {
        let slice = String::from(&input[head..(head + 5)]);
        if slice.chars().nth(0).unwrap() == '0' {
            finished = true;
        }
        head += 5;
        processed_string.push_str(&slice[1..5]);
    }

    (processed_string, head)
}

fn parse_subpackets(input: &str) -> (Vec<Packet>, usize) {
    // First parse the length packet
    let length_type = input.chars().nth(0).unwrap();
    let mut packets = vec![];
    let mut index = 0;

    match length_type {
        '0' => {
            // Take the next 15 bits
            let length = &input[1..16];
            let packet = &input[16..];
            let n = usize::from_str_radix(length, 2).unwrap();
            while index < n {
                let packet_sub = &packet[index..];
                let (packet, sub_index) = parse_packet_from_string(String::from(packet_sub));
                index += sub_index;
                packets.push(packet)
            }
            // Add the index caused by the length header to this packet
            index += 16;
        }
        _ => {
            // Take the next 11 bits
            let length = &input[1..12];
            let packet = &input[12..];
            let n = usize::from_str_radix(length, 2).unwrap();
            for i in 0..n {
                let packet_sub = &packet[index..];
                let (packet, sub_index) = parse_packet_from_string(String::from(packet_sub));
                index += sub_index;
                packets.push(packet)
            }
            // Add the index caused by the length header to this packet
            index += 12;
        }
    }

    (packets, index)
}

fn part_one(packet: &Packet) -> usize {
    packet
        .sub_packets
        .iter()
        .map(|x| part_one(x))
        .sum::<usize>()
        + packet.version
}

fn evaluate_packet(packet: &Packet) -> usize {
    match packet.operator {
        0 => packet
            .sub_packets
            .iter()
            .map(|x| evaluate_packet(x))
            .sum::<usize>(),
        1 => packet
            .sub_packets
            .iter()
            .map(|x| evaluate_packet(x))
            .product::<usize>(),
        2 => packet
            .sub_packets
            .iter()
            .map(|x| evaluate_packet(x))
            .min()
            .unwrap(),
        3 => packet
            .sub_packets
            .iter()
            .map(|x| evaluate_packet(x))
            .max()
            .unwrap(),
        4 => packet.value,
        5 => {
            if evaluate_packet(&packet.sub_packets[0]) > evaluate_packet(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        6 => {
            if evaluate_packet(&packet.sub_packets[0]) < evaluate_packet(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        7 => {
            if evaluate_packet(&packet.sub_packets[0]) == evaluate_packet(&packet.sub_packets[1]) {
                1
            } else {
                0
            }
        }
        _ => unimplemented!(),
    }
}

pub fn day_16() -> Result<()> {
    let input: String = read_input("input/day_16.txt")?.nth(0).unwrap();

    let mut processed_input = String::from("");

    // Take each hexadecimal pair and convert to decimal
    for i in 0..(input.len() / 2) {
        let slice = &input[2 * i..2 * i + 2];

        let n = u8::from_str_radix(slice, 16).unwrap();
        processed_input.push_str(&format!("{:08b}", n));
    }

    let (packet, offset) = parse_packet_from_string(processed_input);

    println!("{:?}", packet);
    println!("{:?}", part_one(&packet));
    println!("{:?}", evaluate_packet(&packet));

    Ok(())
}
