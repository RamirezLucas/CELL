// External libraries
use vulkano::instance::{Instance, InstanceExtensions};

// CELL
mod commands;
mod game_of_life;
mod grid;
mod simulator;
mod terminal_ui;
use game_of_life::GameOfLife;
use simulator::Simulator;
use terminal_ui::TerminalUI;

fn main() -> () {
    let instance = Instance::new(None, &InstanceExtensions::none(), None).unwrap();
    let mut sim = Simulator::new_cpu_sim(
        "Conway GPU",
        GameOfLife::new(),
        &game_of_life::gosper_glider_gun(),
    );
    let mut term_ui = TerminalUI::new(sim);
    term_ui.cmd_interpreter().unwrap();
}
