// bin

use crate::Queue;
use std::collections::VecDeque;
use nfq::Message;


pub trait EnhanceQueue{
    fn new(queue: Queue) -> Self;
}

pub struct QueueOverride{
    queue: Queue,
}

impl EnhanceQueue for QueueOverride{
    fn new(queue: Queue) -> Self {
        QueueOverride {
            queue: queue, 
        }
    }
}



pub trait EnhanceMessage{
    fn new(packet: Message) -> Self;
    fn get_payload(self) -> &'static mut [u8];
}

pub struct MessageOverride{
    packet: Message,
}

impl EnhanceMessage for MessageOverride {
    fn new(message: Message) -> Self{
        MessageOverride { packet: message }
    }

    fn get_payload(self) -> &'static mut [u8] {
        self.get_payload()
    }
}