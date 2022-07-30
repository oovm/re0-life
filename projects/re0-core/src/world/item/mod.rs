pub struct Talent {
    /// 标识符
    pub id: String,
    /// 显示名
    pub display_name: String,
    /// 描述
    pub description: String,
    /// 是否可以初始随机获得
    pub initial: bool,
    /// 稀有度, 越小越稀有
    pub rarity: usize,
    /// 是否可以重复获得
    pub unique: bool,
    /// 穿越是否保留
    pub keep: bool,
}
