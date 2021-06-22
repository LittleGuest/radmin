use crate::controller::Controller;
use crate::models::ToolAlipayConfig;

pub struct AlipayController;

impl Controller for AlipayController {
    type M = ToolAlipayConfig;
}
