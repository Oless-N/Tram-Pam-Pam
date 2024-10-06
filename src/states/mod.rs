

pub trait State {
    fn update(&mut self);  // Метод для оновлення стану
    fn enter(&mut self);   // Метод, що викликається при вході в стан
    fn exit(&mut self);    // Метод, що викликається при виході зі стану
}

// Пример реалізації одного зі станів
pub struct IdleState;

impl IdleState {
    pub fn new() -> Self {
        IdleState
    }
}

impl State for IdleState {
    fn update(&mut self) {
        // Логіка для оновлення стану Idle
    }

    fn enter(&mut self) {
        // Логіка для входу в стан Idle
    }

    fn exit(&mut self) {
        // Логіка для виходу зі стану Idle
    }
}
