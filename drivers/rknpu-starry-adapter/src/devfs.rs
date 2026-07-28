use alloc::sync::Arc;

use axfs_ng_vfs::NodeType;
use starry_core::vfs::{Device, DirMapping, SimpleDir, SimpleFs};

use crate::card0::{CARD0_SYSTEM_DEVICE_ID, Card0};
use crate::card1::{CARD1_SYSTEM_DEVICE_ID, RKNPU_DEVICE_ID, Card1};

/// Register all RKNPU-related `/dev` nodes into the provided devfs root.
pub fn register_rknpu_devices(fs: Arc<SimpleFs>, root: &mut DirMapping) {
    root.add(
        "rknpu",
        Device::new(
            fs.clone(),
            NodeType::CharacterDevice,
            RKNPU_DEVICE_ID,
            Arc::new(Card1::new()),
        ),
    );

    let mut dri_dir = DirMapping::new();
    dri_dir.add(
        "card0",
        Device::new(
            fs.clone(),
            NodeType::CharacterDevice,
            CARD0_SYSTEM_DEVICE_ID,
            Arc::new(Card0::new()),
        ),
    );
    dri_dir.add(
        "card1",
        Device::new(
            fs.clone(),
            NodeType::CharacterDevice,
            CARD1_SYSTEM_DEVICE_ID,
            Arc::new(Card1::new()),
        ),
    );
    root.add("dri", SimpleDir::new_maker(fs, Arc::new(dri_dir)));
}
