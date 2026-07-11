#[derive(Debug, Clone, Default)]
pub struct NotificationService;

impl NotificationService {
    pub fn notify(&self, title: &str, body: &str) {
        let _ = (title, body);
    }
}
