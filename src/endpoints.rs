#[derive(Debug)]
#[allow(non_snake_case)]
pub struct KeyActivateArguments<'a, 'b> {
    pub ProductId: u64,
    pub Key: &'a str,
    pub MachineCode: &'b str,
    pub FieldsToReturn: u64,
    pub FloatingTimeInterval: u64,
    pub MaxOverdraft: u64,
}

impl Default for KeyActivateArguments<'static, 'static> {
    fn default() -> Self {
        Self {
            ProductId: 0,
            Key: "",
            MachineCode: "",
            FieldsToReturn: 0,
            FloatingTimeInterval: 0,
            MaxOverdraft: 0,
        }
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct GetKeyArguments<'a> {
    pub ProductId: u64,
    pub Key: &'a str,
}