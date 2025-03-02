use std::fmt::Debug;

use crate::math::transform::TransformOffset;

use super::node_data::InoxData;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct InoxNodeUuid(pub(crate) u32);

#[derive(Clone, Debug)]
pub struct InoxNode<T = ()> {
    pub uuid: InoxNodeUuid,
    pub name: String,
    pub enabled: bool,
    pub zsort: f32,
    pub trans_offset: TransformOffset,
    pub lock_to_root: bool,
    pub data: InoxData<T>,
}

impl<T> InoxNode<T> {
    pub fn is_node(&self) -> bool {
        self.data.is_node()
    }

    pub fn is_part(&self) -> bool {
        self.data.is_part()
    }

    pub fn is_composite(&self) -> bool {
        self.data.is_composite()
    }

    pub fn is_simple_physics(&self) -> bool {
        self.data.is_simple_physics()
    }

    pub fn is_custom(&self) -> bool {
        self.data.is_custom()
    }

    pub fn node_type_name(&self) -> &'static str {
        self.data.data_type_name()
    }
}
