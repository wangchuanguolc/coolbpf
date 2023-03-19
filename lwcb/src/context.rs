use crate::{ast::*, event::Event, bpf::map::{Map, PerfMap}};
use byteorder::{ByteOrder, NativeEndian};

pub struct Context {
    pub tb: TypeBinding,
    pub events: Vec<Event>,
    pub python: bool,

    pub perf: PerfMap,
}


impl Context {

    pub fn new(tb: TypeBinding) -> Self {
        Self {
            tb,
            events: vec![],
            python: false,
            perf: PerfMap::new(),
        }
    }

    // get last event id
    pub fn event_id(&self) -> usize {
        self.events.len()
    }


    pub fn add_event(&mut self, event: Event) {
        self.events.push(event)
    }

    pub fn perf_mapfd(&self) -> i64{
        self.perf.fd()
    }

    pub fn handle_data(&self, data: Vec<u8>) {
        log::debug!("raw data: {:?}", data);
        let id = NativeEndian::read_u64(&data[..8]);
        self.events[id as usize].print(&data);
    }

}