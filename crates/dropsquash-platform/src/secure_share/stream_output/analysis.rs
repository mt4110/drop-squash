use std::sync::Mutex;

use dropsquash_core::FrameSize;

const MAX_IN_FLIGHT: usize = 3;
const MAX_IN_FLIGHT_BYTES: usize = 96 * 1024 * 1024;

#[derive(Default)]
pub(super) struct AnalysisGate {
    budget: Mutex<Budget>,
}

impl AnalysisGate {
    pub(super) fn try_acquire(&self, size: FrameSize) -> Option<AnalysisSlot<'_>> {
        let bytes = frame_bytes(size)?;
        let mut budget = self.budget.lock().ok()?;
        if budget.frames >= MAX_IN_FLIGHT
            || budget.bytes.saturating_add(bytes) > MAX_IN_FLIGHT_BYTES
        {
            return None;
        }
        budget.frames += 1;
        budget.bytes += bytes;
        Some(AnalysisSlot { gate: self, bytes })
    }
}

pub(super) struct AnalysisSlot<'a> {
    gate: &'a AnalysisGate,
    bytes: usize,
}

impl Drop for AnalysisSlot<'_> {
    fn drop(&mut self) {
        if let Ok(mut budget) = self.gate.budget.lock() {
            budget.frames -= 1;
            budget.bytes -= self.bytes;
        }
    }
}

#[derive(Default)]
struct Budget {
    frames: usize,
    bytes: usize,
}

fn frame_bytes(size: FrameSize) -> Option<usize> {
    (size.width as usize)
        .checked_mul(size.height as usize)?
        .checked_mul(4)
}

#[cfg(test)]
mod tests {
    use super::AnalysisGate;
    use dropsquash_core::FrameSize;

    #[test]
    fn rejects_the_fourth_concurrent_analysis() {
        let gate = AnalysisGate::default();
        let size = FrameSize {
            width: 640,
            height: 480,
        };
        let first = gate.try_acquire(size);
        let second = gate.try_acquire(size);
        let third = gate.try_acquire(size);
        assert!(first.is_some() && second.is_some() && third.is_some());
        assert!(gate.try_acquire(size).is_none());
    }

    #[test]
    fn rejects_a_frame_larger_than_the_byte_budget() {
        let gate = AnalysisGate::default();
        assert!(gate
            .try_acquire(FrameSize {
                width: 8_000,
                height: 4_000
            })
            .is_none());
    }

    #[test]
    fn returns_capacity_when_analysis_finishes() {
        let gate = AnalysisGate::default();
        let size = FrameSize {
            width: 640,
            height: 480,
        };
        let slot = gate.try_acquire(size).expect("first analysis is admitted");
        drop(slot);
        assert!(gate.try_acquire(size).is_some());
    }
}
