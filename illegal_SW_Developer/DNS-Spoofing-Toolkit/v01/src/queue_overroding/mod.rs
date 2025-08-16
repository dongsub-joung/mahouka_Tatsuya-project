use crate::Queue;

pub trait EnhanceQueue{
    fn new(packet: QueueOverride) -> Self;

}

pub struct QueueOverride{
    payload: Queue::Payload,
}

impl EnhanceQueue for QueueOverride{
    fn new(packet: QueueOverride) -> Self {
        QueueOverride { payload : packet.payload }
    }
}