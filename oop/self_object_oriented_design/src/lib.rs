pub mod dev {
    pub struct CD {
        state: Option<Box<dyn State>>,
        details: Vec<String>
    }

    impl CD {
        pub fn new() -> CD {
            CD {
                state: Some(Box::new(Unmounted {})),
                details: vec![]
            }
        }

        pub fn mount(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.mount());
            }
        }

        pub fn rip(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.rip());
            }

        }
        pub fn unmount(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.unmount());
            }
        }

        pub fn detect_metadata(&mut self, v: Vec<String>) {
            if let Some(s) = self.state.take() {
                let (s, v) = s.detect_metadata(v);
                self.state = Some(s);
                if self.details.len() == 0 {
                    self.details = v;
                }
                // String::from("data1"),
                // String::from("data2"),
                // String::from("data3"),
                // String::from("data4"),

            }
        }

        pub fn cd_rip_information(&mut self) -> String {
            self.state.as_ref().unwrap().cd_rip_information(self)

        }
    }
    pub struct Unmounted {}
    impl State for Unmounted {
        fn detect_metadata(self: Box<Self>, v: Vec<String>) -> (Box<dyn State>, Vec<String>) {
            (self, vec![])
        }

        fn rip(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn mount(self: Box<Self>) -> Box<dyn State> {
            Box::new(Mounted {})
        }

        fn unmount(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn cd_rip_information(&self, cd: &CD) -> String {
            String::from("CD not mounted")
        }
    }
    pub struct Mounted {}

    impl State for Mounted {
        fn detect_metadata(self: Box<Self>, v: Vec<String>) -> (Box<dyn State>, Vec<String>) {
            (self, vec![])
        }

        fn rip(self: Box<Self>) -> Box<dyn State> {
            Box::new(Ripping {})
        }

        fn mount(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn unmount(self: Box<Self>) -> Box<dyn State> {
            Box::new(Unmounted {})
        }

        fn cd_rip_information(&self, cd: &CD) -> String {
            String::from("CD not ripped")
        }
    }
    pub struct Ripping {}
    impl State for Ripping {
        fn detect_metadata(self: Box<Self>, v: Vec<String>) -> (Box<dyn State>, Vec<String>) {
            (Box::new(Ripped {}), v)
        }

        fn rip(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn mount(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn unmount(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn cd_rip_information(&self, cd: &CD) -> String {
            String::from("CD is being ripped")
        }
    }
    pub struct Ripped {}

    impl State for Ripped {
        fn detect_metadata(self: Box<Self>, v: Vec<String>) -> (Box<dyn State>, Vec<String>) {
            (self, v)
        }

        fn rip(self: Box<Self>) -> Box<dyn State> {
            todo!()
        }

        fn mount(self: Box<Self>) -> Box<dyn State> {
            todo!()
        }

        fn unmount(self: Box<Self>) -> Box<dyn State> {
            todo!()
        }

        fn cd_rip_information(&self, cd: &CD) -> String {
            let mut s: String = String::new();
            for v in &cd.details {
                s.push_str(&v[..]);
                s.push_str("\n");
            }
            s
        }
    }
    trait State {
        fn detect_metadata(self: Box<Self>, v: Vec<String>) -> (Box<dyn State>, Vec<String>);
        fn rip(self: Box<Self>) -> Box<dyn State>;
        fn mount(self: Box<Self>) -> Box<dyn State>;
        fn unmount(self: Box<Self>) -> Box<dyn State>;
        fn cd_rip_information(&self, cd: &CD) -> String;

    }
}

// cd goes
// 1. unmounted -> mount action
// 2. mounted -> detect metadata
// 3. detected metadata -> rip
// 4. ripped -> display cd info