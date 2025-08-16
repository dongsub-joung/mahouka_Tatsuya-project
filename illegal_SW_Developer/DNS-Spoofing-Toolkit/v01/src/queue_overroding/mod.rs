use crate::Queue;
use std::collections::VecDeque;
use nfq::Message;


pub trait EnhanceQueue{
    fn new(packet: QueueOverride) -> Self;
    fn get_payload(self) -> [u8];
}

pub struct QueueOverride{
    packet: Message,
}

impl EnhanceQueue for QueueOverride{
    fn new(queue: QueueOverride) -> Self {
        QueueOverride {
            packet : queue.packet, 
        }
    }
    fn get_payload(self) -> [u8] {
        self.get_payload()
    }
}