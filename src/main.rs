fn main() {
    Mainwindow::new().unwrap().run().unwrap();
}

slint::slint! {
/*    component Title inherits Text {
        width: 100px;
        height: 40px;
        text: "Finance Tracker";
        color: white;
    } */
    
    export component Mainwindow inherits Window {
        preferred_width: 650px;
        preferred_height: 750px;
        background: black;
        // TitleText{}
        Text {
            text: "Hey there, it is I, Commandliner";
            color: green;
        }
    }
}
