fn main() {
    Mainwindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component Mainwindow inherits Window {
        Text {
            text: "Hey there, it is I, Commandliner";
            color: green;
        }
    }
}
