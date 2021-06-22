#![allow(unused_must_use)]
#![allow(unused)]

extern crate derive_more;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis_macro_driver;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
// #[macro_use]
extern crate simple_excel_writer as excel;
extern crate tera;
#[macro_use]
extern crate validator;

use std::collections::HashMap;
use std::path::PathBuf;

use actix_files::NamedFile;
use actix_session::{CookieSession, Session};
use actix_web::{App, FromRequest, HttpRequest, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use anyhow::Result;
use log::Level;
use rbatis::rbatis::Rbatis;
use rbatis_core::db::DBPoolOptions;
use redis::ConnectionLike;
use tera::Tera;

use controller::*;
use radmin_conf::Config;

use crate::dtos::user::CurrentUser;
use crate::models::ToolAlipayConfig;

mod commons;
mod controller;
mod dtos;
mod middleware;
mod models;
mod utils;

// fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
//     println!("======================");
//     res.response_mut().headers_mut().insert(
//         http::header::CONTENT_TYPE,
//         http::HeaderValue::from_static("Error"),
//     );
//     Ok(ErrorHandlerResponse::Response(res))
// }

lazy_static! {
    static ref RB: Rbatis = Rbatis::new();
    static ref REDIS_CLI: redis::Client = {
        let redis_config = Config::get_redis_config();

        // redis://[<username>][:<passwd>@]<hostname>[:port][/<db>]
        let url = format!("redis://{}:{}/",redis_config.host,redis_config.port);

        redis::Client::open(url).expect("open redis failed")
    };
    // 加载全部模板文件
     static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };

     static ref MYSQL_TO_JAVA: HashMap<&'static str, &'static str> = {
        let mut map: HashMap<&'static str, &'static str> = HashMap::new();
        map.insert("tinyint", "Integer");
        map.insert("smallint", "Integer");
        map.insert("mediumint", "Integer");
        map.insert("int", "Integer");
        map.insert("integer", "Integer");

        map.insert("bigint", "Long");

        map.insert("float", "Float");

        map.insert("double", "Double");

        map.insert("decimal", "BigDecimal");

        map.insert("bit", "Boolean");

        map.insert("char", "String");
        map.insert("varchar", "String");
        map.insert("tinytext", "String");
        map.insert("text", "String");
        map.insert("mediumtext", "String");
        map.insert("longtext", "String");

        map.insert("date", "Timestamp");
        map.insert("datetime", "Timestamp");
        map.insert("timestamp", "Timestamp");
        map
    };
     static ref TEMPLATE_NAMES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

        let mut rts: Vec<&'static str> = Vec::new();
        rts.push("model");
        rts.push("dto");
        rts.push("controller");

        map.insert("rust", rts);

        let mut jts: Vec<&'static str> = Vec::new();
        jts.push("model");
        jts.push("dto");
        jts.push("controller");
        jts.push("mapper");
        jts.push("repository");
        jts.push("service");
        jts.push("service_impl");
        jts.push("query_criteria");

        map.insert("java", jts);

        let mut wts: Vec<&'static str> = Vec::new();
        wts.push("index");
        wts.push("api");

        map.insert("web", wts);

        map
    };
     static ref TEMPLATE_FILE_SUFFIX: HashMap<&'static str, &'static str> = {
        let mut map: HashMap<&'static str, &'static str> = HashMap::new();
        map.insert("rust", "rs");
        map.insert("java", "java");
        map
    };
}

async fn local_file(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?)
}

async fn test(req: HttpRequest) -> actix_web::Result<impl Responder, crate::commons::RespErr> {
    let data = test_data()?;
    Ok(crate::commons::RespBuilder::ok().with_data(data).build())
}

fn test_data() -> anyhow::Result<Vec<u8>> {
    Err(anyhow::Error::msg("anyhow::Error"))
}

// TODO: 在 lazy_static 中使用异步
async fn init_mysql_config() -> Result<()> {
    //初始化连接池
    let mysql_config = Config::get_mysql_config();
    let driver_url = format!(
        "mysql://{}:{}@{}",
        mysql_config.username, mysql_config.password, mysql_config.url
    );
    RB.link(driver_url.as_str()).await?;
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<()> {
    //启用日志输出
    fast_log::fast_log::init_log("requests.log", 1000, Level::Info, None, true).unwrap();

    init_mysql_config().await?;

    // env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .data(CurrentUser::default())
            .app_data(web::Data::new(CurrentUser::default()))
            // .wrap(middleware::RLog)
            .wrap(Logger::new("[%s] %r %a %Dms"))
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, render_500))
            // 测试
            .route("/test", web::get().to(test))
            // 系统：系统授权接口
            // 系统-服务监控管理
            // TODO GET /api/monitor 查询服务监控
            // 系统:定时任务管理
            // TODO GET /api/jobs 查询定时任务
            // TODO POST /api/jobs 新增定时任务
            // TODO PUT /api/jobs 修改定时任务
            // TODO DELETE /api/jobs 删除定时任务
            // TODO PUT /api/jobs/{id} 更改定时任务状态
            // TODO GET /api/jobs/download 导出任务数据
            // TODO PUT /api/jobs/exec/{id} 执行定时任务
            // TODO GET /api/jobs/logs 查询任务执行日志
            // TODO GET /api/jobs/logs/download 导出日志数据
            // 系统：代码生成器配置管理
            // 修改
            .route("/api/genConfig", web::put().to(GenConfigController::update))
            // 查询
            .route(
                "/api/genConfig/{table_name}",
                web::get().to(GenConfigController::get_by_table_name),
            )
            // 系统：代码生成管理
            // 保存字段数据
            .route(
                "/api/generator",
                web::put().to(ColumnConfigController::update_batch),
            )
            // 生成代码
            .route(
                "/api/generator/{table_name}/{event_type}",
                web::get().to(ColumnConfigController::generator),
            )
            // 查询字段数据
            .route(
                "/api/generator/columns",
                web::get().to(ColumnConfigController::columns),
            )
            // 同步字段数据
            .route(
                "/api/generator/sync",
                web::post().to(ColumnConfigController::sync),
            )
            // 查询数据库数据
            .route(
                "/api/generator/tables",
                web::get().to(ColumnConfigController::tables),
            )
            // 查询数据库数据
            .route(
                "/api/generator/tables/all",
                web::get().to(ColumnConfigController::tables),
            )
            // 系统：在线用户管理
            // TODO 查询在线用户
            .route("/auth/online", web::get().to(OnlineController::online))
            // TODO 踢出用户
            .route("/auth/online", web::delete().to(OnlineController::offline))
            // TODO GET /auth/online/download 导出数据
            // 系统：字典管理
            // 查询字典
            .route("/api/dict", web::get().to(DictController::page))
            // 新增字典
            .route("/api/dict", web::post().to(DictController::save))
            // 修改字典
            .route("/api/dict", web::put().to(DictController::update))
            // 删除字典
            .route("/api/dict", web::delete().to(DictController::delete_batch))
            // 查询字典
            .route("/api/dict/all", web::get().to(DictController::list))
            // 导出字典数据
            .route("/api/dict/download", web::get().to(DictController::export))
            // 系统：字典详情管理
            // 查询字典详情
            .route("/api/dictDetail", web::get().to(DictDetailController::page))
            // 新增字典详情
            .route(
                "/api/dictDetail",
                web::post().to(DictDetailController::save),
            )
            // 修改字典详情
            .route(
                "/api/dictDetail",
                web::put().to(DictDetailController::update),
            )
            // 删除字典详情
            .route(
                "/api/dictDetail/{id}",
                web::delete().to(DictDetailController::remove_by_id),
            )
            // 查询多个字典详情
            .route(
                "/api/dictDetail/map",
                web::get().to(DictDetailController::maps),
            )
            // 系统：岗位管理
            // 查询岗位
            .route("/api/job", web::get().to(JobController::page))
            // 新增岗位
            .route("/api/job", web::post().to(JobController::save))
            // 修改岗位
            .route("/api/job", web::put().to(JobController::update))
            // 删除岗位
            .route(
                "/api/job",
                web::delete().to(JobController::remove_batch_by_ids),
            )
            // 导出岗位数据
            .route("/api/job/download", web::get().to(JobController::export))
            // 系统：日志管理
            // 日志查询
            .route("/api/logs", web::get().to(LogController::page))
            // 删除所有ERROR日志
            .route(
                "/api/logs/del/error",
                web::delete().to(LogController::delete_error),
            )
            // 删除所有INFO日志
            .route(
                "/api/logs/del/info",
                web::delete().to(LogController::delete_info),
            )
            // 导出数据
            .route(
                "/api/logs/download",
                web::get().to(LogController::export_info),
            )
            // 错误日志查询
            .route("/api/logs/error", web::get().to(LogController::page_error))
            // 导出错误数据
            .route(
                "/api/logs/error/download",
                web::get().to(LogController::export_error),
            )
            // 日志异常详情查询
            .route(
                "/api/logs/error/{id}",
                web::get().to(LogController::error_detail),
            )
            // 用户日志查询
            .route("/api/logs/user", web::get().to(LogController::user_log))
            // 系统：用户管理
            // 查询用户
            .route("/api/users", web::get().to(UserController::page))
            // 新增用户
            .route("/api/users", web::post().to(UserController::save))
            // 修改用户
            .route("/api/users", web::put().to(UserController::update))
            // 删除用户
            .route("/api/users", web::delete().to(UserController::delete_batch))
            // 修改用户：个人中心
            .route("/api/users/center", web::put().to(UserController::center))
            // 导出用户数据
            .route("/api/users/download", web::get().to(UserController::export))
            // 修改头像
            .route(
                "/api/users/updateAvatar",
                web::post().to(UserController::update_avatar),
            )
            // 修改邮箱
            .route(
                "/api/users/updateEmail/{code}",
                web::post().to(UserController::update_email),
            )
            // 修改密码
            .route(
                "/api/users/updatePass",
                web::post().to(UserController::update_pass),
            )
            // 系统：系统授权接口
            // 获取验证码
            .route("/auth/code", web::get().to(AuthController::code))
            // 获取用户信息
            .route("/auth/info", web::get().to(AuthController::user_info))
            // 登录授权
            .route("/auth/login", web::post().to(AuthController::login))
            // 退出登录
            .route("/auth/logout", web::delete().to(AuthController::logout))
            // 系统：菜单管理
            // 查询菜单
            .route("/api/menus", web::get().to(MenuController::page))
            // 新增菜单
            .route("/api/menus", web::post().to(MenuController::save))
            // 修改菜单
            .route("/api/menus", web::put().to(MenuController::update))
            // 删除菜单
            .route(
                "/api/menus",
                web::delete().to(MenuController::remove_batch_by_ids),
            )
            // 获取前端所需菜单
            .route("/api/menus/build", web::get().to(MenuController::build))
            // 根据菜单ID返回所有子节点ID，包含自身ID
            .route("/api/menus/child", web::get().to(MenuController::child))
            // 导出菜单数据
            .route("/api/menus/download", web::get().to(MenuController::export))
            // 返回全部的菜单
            .route("/api/menus/lazy", web::get().to(MenuController::lazy))
            // 查询菜单:根据ID获取同级与上级数据
            .route(
                "/api/menus/superior",
                web::post().to(MenuController::superior),
            )
            // 系统：角色管理
            // 查询角色
            .route("/api/roles", web::get().to(RoleController::page))
            // 新增角色
            .route("/api/roles", web::post().to(RoleController::save))
            // 修改角色
            .route("/api/roles", web::put().to(RoleController::update))
            // 删除角色
            .route("/api/roles", web::delete().to(RoleController::delete_batch))
            // 返回全部的角色
            .route("/api/roles/all", web::get().to(RoleController::all))
            // 导出角色数据
            .route("/api/roles/download", web::get().to(RoleController::export))
            // 获取用户级别
            .route("/api/roles/level", web::get().to(RoleController::level))
            // 修改角色菜单
            .route("/api/roles/menu", web::put().to(RoleController::role_menu))
            // 获取单个role
            .route("/api/roles/{id}", web::get().to(RoleController::get_by_id))
            // 系统：部门管理
            // 查询部门
            .route("/api/dept", web::get().to(DeptController::page))
            // 新增部门
            .route("/api/dept", web::post().to(DeptController::save))
            // 修改部门
            .route("/api/dept", web::put().to(DeptController::update))
            // 删除部门
            .route(
                "/api/dept",
                web::delete().to(DeptController::remove_batch_by_ids),
            )
            // 导出部门数据
            .route("/api/dept/download", web::get().to(DeptController::export))
            // 查询部门:根据ID获取同级与上级数据
            .route(
                "/api/dept/superior",
                web::post().to(DeptController::superior),
            )
            // 系统：限流测试管理
            // TODO GET /api/limit 测试
            // 系统：验证码管理
            // TODO POST /api/code/email/resetPass 重置密码，发送验证码
            // 重置邮箱，发送验证码
            .route(
                "/api/code/resetEmail",
                web::post().to(VerifyController::reset_email),
            )
            // TODO GET /api/code/validated 验证码验证
            // 工具：七牛云存储管理
            // TODO GET /api/qiNiuContent 查询文件
            // .route(
            //     "/api/qiNiuContent/{id}",
            //     web::get().to(QiniuController::get_one),
            // )
            // TODO POST /api/qiNiuContent 上传文件
            // TODO DELETE /api/qiNiuContent 删除多张图片
            // TODO DELETE /api/qiNiuContent/{id} 删除文件
            // TODO GET /api/qiNiuContent/config queryConfig
            // TODO PUT /api/qiNiuContent/config 配置七牛云存储
            // TODO GET /api/qiNiuContent/download 导出数据
            // TODO GET /api/qiNiuContent/download/{id} 下载文件
            // TODO POST /api/qiNiuContent/synchronize 同步七牛云数据
            // 工具：支付宝管理
            // TODO GET queryConfig
            // .route("/api/aliPay/{id}", web::get().to(AlipayController::get_one))
            // TODO PUT /api/aliPay 配置支付宝
            // .route("/api/aliPay", web::put().to(AlipayController::update))
            // TODO POST /api/aliPay/toPayAsPC PC网页支付
            // TODO POST /api/aliPay/toPayAsWeb 手机网页支付
            // 工具：本地存储管理
            // 查询文件
            .route(
                "/api/local_storage",
                web::get().to(LocalStorageController::page),
            )
            // 上传文件
            .route(
                "/api/local_storage",
                web::post().to(LocalStorageController::upload),
            )
            // TODO PUT /api/localStorage 修改文件
            // 多选删除
            .route(
                "/api/local_storage",
                web::post().to(LocalStorageController::delete_batch_by_ids),
            )
            // TODO GET /api/localStorage/download 导出数据
            // TODO POST /api/localStorage/pictures 上传图片
            // 工具：邮件管理
            // 获取邮箱配置
            .route("/api/email", web::get().to(EmailController::get_by_id))
            // 发送邮件
            .route("/api/email", web::post().to(EmailController::send_email))
            // 配置邮件
            .route("/api/email", web::put().to(EmailController::update_by_id))
            // 运维：应用管理
            // 查询应用
            .route("/api/app", web::get().to(AppController::page))
            // 新增应用
            .route("/api/app", web::post().to(AppController::save))
            // 修改应用
            .route("/api/app", web::put().to(AppController::update))
            // 删除应用
            .route(
                "/api/app",
                web::delete().to(AppController::remove_batch_by_ids),
            )
            // 导出应用数据
            .route("/api/app/download", web::get().to(AppController::export))
        // 运维：数据库管理
        // TODO GET /api/database 查询数据库
        // TODO POST /api/database 新增数据库
        // TODO PUT /api/database 修改数据库
        // TODO DELETE /api/database 删除数据库
        // TODO GET /api/database/download 导出数据库数据
        // TODO POST /api/database/testConnect 测试数据库链接
        // TODO POST /api/database/upload 执行SQL脚本
        // 运维：服务器管理
        // TODO GET /api/serverDeploy 查询服务器
        // TODO POST /api/serverDeploy 新增服务器
        // TODO PUT /api/serverDeploy 修改服务器
        // TODO DELETE /api/serverDeploy 删除Server
        // TODO GET /api/serverDeploy/download 导出服务器数据
        // TODO POST /api/serverDeploy/testConnect 测试连接服务器
        // 运维：部署历史管理
        // TODO GET /api/deployHistory 查询部署历史
        // TODO DELETE /api/deployHistory 删除部署历史
        // TODO GET /api/deployHistory/download 导出部署历史数据
        // 运维：部署管理
        // TODO GET /api/deploy 查询部署
        // TODO POST /api/deploy 新增部署
        // TODO PUT /api/deploy 修改部署
        // TODO DELETE /api/deploy 删除部署
        // TODO GET /api/deploy/download 导出部署数据
        // TODO POST /api/deploy/serverReduction 系统还原
        // TODO POST /api/deploy/serverStatus 服务运行状态
        // TODO POST /api/deploy/startServer 启动服务
        // TODO POST /api/deploy/stopServer 停止服务
        // TODO POST /api/deploy/upload 上传文件部署
        // 访问文件
        // .route("/{filename:.*}", web::get().to(local_file))
    })
        .bind("127.0.0.1:10241")?
        .run()
        .await;
    Ok(())
}
