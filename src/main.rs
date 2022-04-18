use utils::timer::Timer;

mod day1_sonar_sweep;
mod day2_dive;
mod day3_binary_diagnostic;
mod day4_bingo;
mod day5_hydrothermal_lines;
mod day6_lanternfish;
mod day7_crabs;
mod day8_seven_segment_search;
mod day9_low_points;
mod day10_syntax_scoring;
mod day11_dumbo_octopus;
mod day12_passage_pathing;
mod day13_transparent_origami;
mod day14_polymerization;
mod day15_chiton;
mod day16_packet_decoder;
mod day17_trick_shot;
mod day18_snailfish;
mod day19_beacon_scanner;
mod day20_trench_map;
mod day21_dirac_dice;
mod day22_reactor_reboot;
mod day23_amphipod;
mod day24_arithmetic_logic_unit;
mod day25_sea_cucumber;

fn main() {
    let _timer = Timer::start(|elapsed| println!("main took {} ms.", elapsed.as_millis()));
    let day: usize = if let Some(arg1) = std::env::args().nth(1) {
        arg1.parse().expect("argument should be an integer")
    } else {
        25
    };
    println!("running day {}\n", day);
    match day {
        1 => day1_sonar_sweep::run(),
        2 => day2_dive::run(),
        3 => day3_binary_diagnostic::run(),
        4 => day4_bingo::run(),
        5 => day5_hydrothermal_lines::run(),
        6 => day6_lanternfish::run(),
        7 => day7_crabs::run(),
        8 => day8_seven_segment_search::run(),
        9 => day9_low_points::run(),
        10 => day10_syntax_scoring::run(),
        11 => day11_dumbo_octopus::run(),
        12 => day12_passage_pathing::run(),
        13 => day13_transparent_origami::run(),
        14 => day14_polymerization::run(),
        15 => day15_chiton::run(),
        16 => day16_packet_decoder::run(),
        17 => day17_trick_shot::run(),
        18 => day18_snailfish::run(),
        19 => day19_beacon_scanner::run(),
        20 => day20_trench_map::run(),
        21 => day21_dirac_dice::run(),
        22 => day22_reactor_reboot::run(),
        23 => day23_amphipod::run(),
        24 => day24_arithmetic_logic_unit::run(),
        25 => day25_sea_cucumber::run(),
        _ => panic!("day {} not found", day)
    }
}
