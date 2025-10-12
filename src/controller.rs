mod controller {
    use std::collections::{HashMap, LinkedList};

    use gilrs::Button;

    enum KeyState {
        Key(bool),
        Trigger(f32),
        Axis(f32)
    }

    struct Controller {
        keys: HashMap<gilrs::Button, KeyState>
    }

    impl Controller {
        fn update(self, state: KeyState) {
            
        }
    }
}
