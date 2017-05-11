use chord::{RingKey};

pub struct IdsChain {
    cur_id: RingKey, // Current id
    dst_id: RingKey, // Destination id
}

/// Find the msb bit index of a given number.
fn get_msb(mut x: RingKey) -> Option<usize> {
    match x {
        0 => None,
        _ => {
            let mut index: usize = 0;
            while x > 0 {
                x >>= 1;
                index += 1;
            }
            Some(index)
        }
    }
}

///
/// Iterator for a chain of ids between some source id and a destination id.
/// Every two adjacent produced ids have a difference which is an exact
/// power of 2.
/// This iterator is guaranteed to be deterministic. (It will return the same
/// chain for the same source and destination ids every time).
impl Iterator for IdsChain {
    type Item = RingKey;
    fn next(&mut self) -> Option<RingKey> {
        if self.cur_id == self.dst_id {
            // We have already arrived:
            return None
        }

        // Find the most significant different bit between cur_id and dst_id:
        let msb_diff: usize = get_msb(self.cur_id ^ self.dst_id).unwrap();

        // Check if we need to add or to subtract:
        let pow_diff: RingKey = 2_u64.pow(msb_diff as u32);
        if (self.cur_id >> msb_diff) & 1 == 0 {
            self.cur_id += pow_diff;
        } else {
            self.cur_id -= pow_diff;
        }
        Some(self.cur_id)
    }
}

fn ids_chain(src_id: RingKey, dst_id: RingKey) -> IdsChain {
    IdsChain {
        cur_id: src_id,
        dst_id: dst_id,
    }
}