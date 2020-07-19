use tmux_interface::{TmuxInterface, NewSession};


fn main() {
    let mut tmux = TmuxInterface::new();

    let new_session = NewSession {
        cwd: Some("./server"),
        session_name: Some("mcserver"),
        shell_command: Some("java -Xms1024M -Xmx3048M -jar ./server.jar nogui"),
        ..Default::default()
    };
    tmux.new_session(Some(&new_session)).unwrap();
}
