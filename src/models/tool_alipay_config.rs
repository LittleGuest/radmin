use anyhow::Result;
use rbatis::crud::CRUD;
use rbatis::plugin::page::{Page, PageRequest};

use radmin_macro::RBatisModel;

use crate::models::RBatisModel;
use crate::RB;

/// 支付宝配置类
#[crud_enable]
#[derive(RBatisModel, Default, Clone, Debug)]
pub struct ToolAlipayConfig {
    /// ID
    pub id: Option<i64>,
    /// 应用ID
    pub app_id: Option<String>,
    /// 编码
    pub charset: Option<String>,
    /// 类型 固定格式json
    pub format: Option<String>,
    /// 网关地址
    pub gateway_url: Option<String>,
    /// 异步回调
    pub notify_url: Option<String>,
    /// 私钥
    pub private_key: Option<String>,
    /// 公钥
    pub public_key: Option<String>,
    /// 回调地址
    pub return_url: Option<String>,
    /// 签名方式
    pub sign_type: Option<String>,
    /// 商户号
    pub sys_service_provider_id: Option<String>,
}
