use serde::{Serialize, Deserialize};
use crate::syntax_elements::{
    attributes::ARSAttribute,
    constants::ARSConst,
    enums::ARSEnum,
    functions::ARSFunction,
    impl_blocks::ARSImpl,
    macros::ARSMacro,
    modules::ARSModule,
    statics::ARSStatic,
    structs::ARSStruct,
    traits::ARSTrait,
    type_aliases::ARSTypeAlias,
    use_statements::ARSUse,
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ARSFile {
    pub attributes: Vec<ARSAttribute>,
    pub uses: Vec<ARSUse>,
    pub constants: Vec<ARSConst>,
    pub statics: Vec<ARSStatic>,
    pub type_aliases: Vec<ARSTypeAlias>,
    pub macros: Vec<ARSMacro>,
    pub structs: Vec<ARSStruct>,
    pub enums: Vec<ARSEnum>,
    pub traits: Vec<ARSTrait>,
    pub impl_blocks: Vec<ARSImpl>,
    pub modules: Vec<ARSModule>,
    pub functions: Vec<ARSFunction>,

}
