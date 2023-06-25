use fugit::MicrosDurationU64;

pub struct AnimationStep {
    duration: MicrosDurationU64,
}

impl AnimationStep {
    pub fn new(duration: MicrosDurationU64) -> Self {
        Self {
            duration
        }
    }

    pub fn duration(&self) -> MicrosDurationU64 {
        self.duration
    }
}