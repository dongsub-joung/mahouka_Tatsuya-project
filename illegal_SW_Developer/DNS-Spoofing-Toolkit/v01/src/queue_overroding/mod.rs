use crate::Queue;
use std::collections::VecDeque;
use nfq::Message;


pub trait EnhanceQueue{
    fn new(packet: QueueOverride) -> Self;
}

#[derive(Copy)]
pub struct QueueOverride<'a>{
    packet: Message,
    payload: &'a [u8],
}

impl EnhanceQueue for QueueOverride<'_>{
    fn new(queue: QueueOverride) -> Self {
        QueueOverride {
            packet : queue.packet, 
            payload : queue.packet.get_payload(),
        }
    }
}