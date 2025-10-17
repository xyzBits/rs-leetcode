use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Hash, Eq, PartialEq, Clone)]
struct Packet(i32, i32, i32);

struct Router {
    mem_limit: i32,
    length: i32,
    is_exist: HashSet<Packet>,
    same_dest_que: HashMap<i32, VecDeque<i32>>,
    que: VecDeque<Packet>,
}

impl Router {
    fn new(memory_limit: i32) -> Self {
        Router {
            mem_limit: memory_limit,
            length: 0,
            is_exist: HashSet::new(),
            same_dest_que: HashMap::new(),
            que: VecDeque::new(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet = Packet(source, destination, timestamp);
        if self.is_exist.contains(&packet) {
            return false;
        }

        if self.length == self.mem_limit {
            self.forward_packet();
        }

        self.length += 1;
        self.que.push_back(packet.clone());
        self.same_dest_que
            .entry(destination)
            .or_insert(VecDeque::new())
            .push_back(timestamp);
        self.is_exist.insert(packet);

        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        let mut data = Vec::new();
        if let Some(packet) = self.que.pop_front() {
            data = vec![packet.0, packet.1, packet.2];

            self.is_exist.remove(&packet);
            if let Some(dq) = self.same_dest_que.get_mut(&data[1]) {
                dq.pop_front();
            }
            self.length -= 1;
        }

        data
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some(dq) = self.same_dest_que.get(&destination) {
            dq.iter()
                .filter(|&&t| t >= start_time && t <= end_time)
                .count() as i32
        } else {
            0
        }
    }
}
