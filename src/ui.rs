use crate::harvest::cpu::Cpu;
use crate::harvest::process::Process;

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge, Row, Table, TableState},
    Frame,
};

struct ProcessTable {
    state: TableState,
    items: Vec<Vec<String>>,
}

impl ProcessTable {
    fn new(processes: Vec<&Process>) -> ProcessTable {
        let mut items = Vec::new();
        for process in processes {
            if process.status == "Unknown" {
                continue;
            }
            let row: Vec<String> = vec![
                process.pid.to_string(),
                process.name.clone(),
                process.status.clone(),
                process.cpu_usage.to_string(),
                process.memory.to_string(),
            ];
            items.push(row);
        }

        ProcessTable {
            state: TableState::default(),
            items,
        }
    }
}

pub fn draw_cpus<B>(f: &mut Frame<B>, cpus: &[Cpu], area: Rect)
where
    B: Backend,
{
    let (half0, half1) = cpus.split_at(cpus.len() / 2);
    let block = Block::default().borders(Borders::ALL).title("CPU");
    f.render_widget(block, area);

    let chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .direction(Direction::Horizontal)
        .split(area);

    draw_cpu_block(f, half0, chunks[0]);
    draw_cpu_block(f, half1, chunks[1]);
}

fn draw_cpu_block<B>(f: &mut Frame<B>, cpus: &[Cpu], area: Rect)
where
    B: Backend,
{
    let mut constraints = Vec::new();
    for _ in 0..cpus.len() {
        constraints.push(Constraint::Length(2));
    }

    let chunks = Layout::default()
        .constraints(constraints.as_ref())
        .margin(1)
        .split(area);

    for (i, cpu) in cpus.iter().enumerate() {
        let gauge = Gauge::default()
            .block(Block::default().title(&cpu.name).borders(Borders::NONE))
            .style(Style::default().fg(Color::Yellow))
            .percent(cpu.usage as u16);
        f.render_widget(gauge, chunks[i]);
    }
}

pub fn draw_processes<B>(f: &mut Frame<B>, processes: Vec<&Process>, area: Rect)
where
    B: Backend,
{
    let table = ProcessTable::new(processes);
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(0)
        .split(area);

    let normal_syle = Style::default().fg(Color::White);
    let header = ["PID", "Name", "Status", "CPU", "Memory"];
    let rows = table
        .items
        .iter()
        .map(|i| Row::StyledData(i.iter(), normal_syle));

    let t = Table::new(header.iter(), rows)
        .block(Block::default().title("Process").borders(Borders::ALL))
        .widths(&[
            Constraint::Percentage(5),
            Constraint::Percentage(40),
            Constraint::Percentage(5),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ]);
    f.render_widget(t, chunks[0]);
}
