use tauri::{CustomMenuItem, Menu, Submenu, WindowMenuEvent};

// 应用菜单项
pub fn init() -> Menu {
    //（自定义菜单）
    let file_menu = Submenu::new(
        "初始化环境",
        Menu::new()
            .add_item(CustomMenuItem::new("node".to_string(), "安装node"))
            .add_item(CustomMenuItem::new("nvm".to_string(), "安装nvm"))
            .add_item(CustomMenuItem::new("nrm".to_string(), "安装nrm"))
            .add_item(CustomMenuItem::new("yarn".to_string(), "安装yarn"))
            .add_item(CustomMenuItem::new("pnpm".to_string(), "安装pnpm"))
            .add_item(CustomMenuItem::new("ts".to_string(), "安装typecript")),
    );
    // （自定义菜单）
    let edit_menu = Submenu::new(
        "开发工具",
        Menu::new()
            .add_item(CustomMenuItem::new("vscode".to_string(), "安装vscode"))
            .add_item(CustomMenuItem::new(
                "tortoisegit".to_string(),
                "安装tortoisegit",
            ))
            .add_item(CustomMenuItem::new("git".to_string(), "安装git")),
    );
    // （自定义菜单）
    let study_menu = Submenu::new(
        "学习资料",
        Menu::new()
            .add_item(CustomMenuItem::new("vue".to_string(), "vue"))
            .add_item(CustomMenuItem::new("git-1".to_string(), "git")),
    );
    // （自定义菜单）
    let help_menu = Submenu::new(
        "帮助",
        Menu::new()
            .add_item(CustomMenuItem::new("about".to_string(), "关于"))
            .add_item(CustomMenuItem::new("license".to_string(), "查看许可证")),
    );

    Menu::new()
        .add_submenu(file_menu)
        .add_submenu(edit_menu)
        .add_submenu(study_menu)
        .add_submenu(help_menu)
}

// 应用菜单处理事件
pub fn handler(event: WindowMenuEvent) {
    // 匹配菜单 id
    match event.menu_item_id() {
        _ => {
            let main_window = event.window();
            let info = event.menu_item_id().to_string();
            main_window.emit("clickMenuItem", info).unwrap();
        }
    }
}
