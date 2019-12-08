use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{BoxView, LinearLayout, Panel, TextView};
use cursive::Cursive;
use std::io::Write;
use crate::machine::*;

pub struct TuiDebugger<'a> {
    machine: &'a mut Machine
}

impl TuiDebugger<'_> {
    pub fn new<'a>(machine: &'a mut Machine) -> TuiDebugger<'a> {
        TuiDebugger { machine }
    }

    fn update_tui(&mut self, app: &mut Cursive) {
        let mut prg = Vec::new();
        let mut addr = self.machine.ip();
        loop {
            addr = match self.machine.get_disassembly(addr) {
                Some(Disassembly::Instruction(x)) => {
                    writeln!(prg, "{}", x).expect("err");
                    addr + x.increment()
                },
                Some(Disassembly::MemoryValue(x)) => {
                    writeln!(prg, "{}", x).expect("err");
                    addr + 1
                }
                _ => {
                    break;
                }
            };
        }

        let s = String::from_utf8(prg).unwrap();
        let mut p = app.find_id::<TextView>("program").unwrap();
        p.set_content(s);
        let mut ms = Vec::new();
        self.machine.memory().iter().enumerate().for_each(|(a, &v)| {
            if (a % 8) == 0 {
                write!(&mut ms, "{:>8}: ", a).expect("err");
            }
            write!(&mut ms, "{:>8},", v).expect("err");
            if (a % 8) == 7 {
                writeln!(&mut ms).expect("err");
            }
        });
        let mut m = app.find_id::<TextView>("mem").unwrap();
        m.set_content(String::from_utf8(ms).unwrap());
    }

    pub fn debug(&mut self) {
        let mut app = Cursive::default();
        app.add_fullscreen_layer(
            LinearLayout::vertical()
                .child(BoxView::with_full_screen(
                    LinearLayout::horizontal()
                        .child(BoxView::with_full_width(
                            Panel::new(TextView::new("").with_id("program").scrollable())
                                .title("Program")
                                .title_position(HAlign::Left),
                        ))
                        .child(BoxView::with_fixed_width(
                            12,
                            Panel::new(TextView::new("").scrollable())
                                .title("Bp")
                                .title_position(HAlign::Left),
                        )),
                ))
                .child(BoxView::with_full_screen(
                    Panel::new(TextView::new("").with_id("mem").scrollable())
                        .title("Mem")
                        .title_position(HAlign::Left),
                )),
        );
        app.add_global_callback('q', |a| a.quit());
        self.update_tui(&mut app);
        app.run();
    }
}
