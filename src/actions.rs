pub fn start_server() {
	use tmux_interface::{TmuxInterface, NewSession};
		let mut tmux = TmuxInterface::new();
   
		let new_session = NewSession {
			cwd: Some("./server"),
			session_name: Some("mcserver"),
			shell_command: Some("./test.sh"),
			..Default::default()
		};
		tmux.new_session(Some(&new_session)).unwrap();
}