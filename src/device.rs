use crate::data::Data;

enum DeviceType {
    Output,
    Input,
}
struct DataHandler{
    stream: Vec<Data>, //single threaded processing
}
impl DataHandler {
    fn test() {
        ()
    }
}
pub struct Device {}
