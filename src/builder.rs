/// Layer in OSI Model
pub trait Layer {
    fn wrap(&self, data: &[u8]) -> Vec<u8>;
}

pub struct PacketBuilder {
    layers: Vec<Box<dyn Layer>>,
}

impl PacketBuilder {
    pub fn new() -> Self {
        PacketBuilder { layers: Vec::new() }
    }

    pub fn layer<L: Layer + 'static>(mut self, layer: L) -> Self {
        self.layers.push(Box::new(layer));
        self
    }

    pub fn build(self, data: Vec<u8>) -> Vec<u8> {
        self.layers
            .into_iter()
            .rev()
            .fold(data, |payload, layer| layer.wrap(&payload))
    }
}
