#[derive(Debug)]
pub struct Data {
    pub message: Vec<u8>,// Закодируем сообщение 8 битными цифрами
}
impl Data {
    pub fn is_empty(&self) -> bool {
        if self.message.is_empty(){
            true
        } else {
            false
        }
    }
    pub fn size(&self) -> usize {
        self.message.len()
    }
}
