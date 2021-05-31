pub trait Controller {
    /// 处理额外的追回数据
    fn index_after(_data: &mut tera::Context);

    /// 处理编辑时需要展现出来的附加数据
    fn edit_after(_data: &mut tera::Context);
    /// 保存之后处理
    fn save_after() ;

    /// 删除之后处理
    fn delete_after() ;
}