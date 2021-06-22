use crate::controller::Controller;
use crate::models::ToolQiniuContent;

pub struct QiniuController;

impl Controller for QiniuController {
    type M = ToolQiniuContent;
}
